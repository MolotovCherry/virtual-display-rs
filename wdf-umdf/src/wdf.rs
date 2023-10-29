#![allow(non_snake_case)]

use std::ffi::c_void;

use wdf_umdf_sys::{
    DEVPROPTYPE, NTSTATUS, PCUNICODE_STRING, PCWDF_OBJECT_CONTEXT_TYPE_INFO, PDRIVER_OBJECT,
    POOL_TYPE, PWDFDEVICE_INIT, PWDF_DRIVER_CONFIG, PWDF_OBJECT_ATTRIBUTES, WDFDEVICE, WDFDRIVER,
    WDFMEMORY, WDFOBJECT, WDF_DEVICE_FAILED_ACTION, WDF_NO_HANDLE, WDF_NO_OBJECT_ATTRIBUTES,
    WDF_OBJECT_ATTRIBUTES, _WDF_DEVICE_PROPERTY_DATA, _WDF_PNPPOWER_EVENT_CALLBACKS,
};

use crate::IntoHelper;

#[derive(Debug, thiserror::Error)]
pub enum WdfError {
    #[error("{0}")]
    WdfFunctionNotAvailable(&'static str),
    #[error("{0}")]
    CallFailed(NTSTATUS),
    #[error("Failed to upgrade Arc pointer")]
    UpgradeFailed,
    #[error("Failed to lock")]
    LockFailed,
    // this is required for success status for ()
    #[error("This is not an error, ignore it")]
    Success,
}

impl From<()> for WdfError {
    fn from(_: ()) -> Self {
        WdfError::Success
    }
}

impl From<WdfError> for NTSTATUS {
    fn from(value: WdfError) -> Self {
        use WdfError::*;
        match value {
            WdfFunctionNotAvailable(_) => Self::STATUS_NOT_FOUND,
            CallFailed(status) => status,
            UpgradeFailed => Self::STATUS_INVALID_HANDLE,
            LockFailed => Self::STATUS_WAS_LOCKED,
            Success => 0.into(),
        }
    }
}

impl From<NTSTATUS> for WdfError {
    fn from(value: NTSTATUS) -> Self {
        WdfError::CallFailed(value)
    }
}

macro_rules! WdfCall {
    ($name:ident ( $($args:expr),* )) => {{
        let fn_handle = {
            ::paste::paste! {
                const FN_INDEX: usize = ::wdf_umdf_sys::WDFFUNCENUM::[<$name TableIndex>].0 as usize;

                // validate that wdf function can be used
                let is_available = ::wdf_umdf_sys::WdfIsFunctionAvailable!($name);

                if is_available {
                    // SAFETY: Only immutable accesses are done to this
                    let fn_table = unsafe { ::wdf_umdf_sys::WdfFunctions_02031 };

                    // SAFETY: Read-only, initialized by the time we use it, and checked to be in bounds
                    let fn_handle = unsafe {
                        fn_table
                        .add(FN_INDEX)
                        .cast::<::wdf_umdf_sys::[<PFN_ $name:upper>]>()
                    };

                    // SAFETY: Ensured that this is present by if condition from `WdfIsFunctionAvailable!`
                    let fn_handle = unsafe { fn_handle.read() };
                    // SAFETY: All available function handles are not null
                    let fn_handle = unsafe { fn_handle.unwrap_unchecked() };

                    Ok(fn_handle)
                } else {
                    Err($crate::WdfError::WdfFunctionNotAvailable(concat!(stringify!($name), " is not available")))
                }
            }
        };

        if let Ok(fn_handle) = fn_handle {
            // SAFETY: Pointer to globals is always immutable
            let globals = unsafe { ::wdf_umdf_sys::WdfDriverGlobals };

            // SAFETY: None. User is responsible for safety and must use their own unsafe block
            Ok(unsafe { fn_handle(globals, $($args),*) })
        } else {
            // SAFETY: We checked if it was Ok above, and it clearly isn't
            Err(unsafe {
                fn_handle.unwrap_err_unchecked()
            })
        }
    }};
}

/// Unlike the official WDF_DECLARE_CONTEXT_TYPE macro, you only need to declare this on the actual data struct want to use
/// Safety is maintained through a RwLock of the underlying data
///
/// This generates associated fns init/get/drop/get_type_info on your $context_type with the same visibility
///
/// Example:
/// ```rust
/// pub struct IndirectDeviceContext {
///     device: WDFDEVICE,
/// }
///
/// impl IndirectDeviceContext {
///     pub fn new(device: WDFDEVICE) -> Self {
///         Self { device }
///     }
/// }
///
/// WDF_DECLARE_CONTEXT_TYPE!(pub IndirectDeviceContext);
///
/// // with a `device: WDFDEVICE`
/// let context = IndirectDeviceContext::new(device as WDFOBJECT);
/// IndirectDeviceContext::init(context);
/// // elsewhere
/// let mutable_access = IndirectDeviceContext::get_mut(device).unwrap();
/// ```
#[macro_export]
macro_rules! WDF_DECLARE_CONTEXT_TYPE {
    ($sv:vis $context_type:ident) => {
        $crate::paste! {
            // keep it in a mod block to disallow access to private types
            mod [<WdfObject $context_type>] {
                use super::$context_type;

                // Require `T: Sync` for safety. User has to uphold the invariant themselves
                #[repr(transparent)]
                #[allow(non_camel_case_types)]
                struct [<_WDF_ $context_type _STATIC_WRAPPER>]<T> {
                    cell: ::std::cell::UnsafeCell<$crate::wdf_umdf_sys::_WDF_OBJECT_CONTEXT_TYPE_INFO>,
                    _phantom: ::std::marker::PhantomData<T>
                }

                // SAFETY: `T` impls Sync too
                unsafe impl<T: Sync> Sync for [<_WDF_ $context_type _STATIC_WRAPPER>]<T> {}

                // Unsure if C mutates this data, but it's in an unsafecell just in case
                #[allow(non_upper_case_globals)]
                static [<_WDF_ $context_type _TYPE_INFO>]: [<_WDF_ $context_type _STATIC_WRAPPER>]<$context_type> =
                    [<_WDF_ $context_type _STATIC_WRAPPER>] {
                        cell: ::std::cell::UnsafeCell::new(
                            $crate::wdf_umdf_sys::_WDF_OBJECT_CONTEXT_TYPE_INFO {
                                Size: ::std::mem::size_of::<$crate::wdf_umdf_sys::_WDF_OBJECT_CONTEXT_TYPE_INFO>() as u32,
                                ContextName: concat!(stringify!($context_type), "\0")
                                    .as_ptr().cast::<::std::ffi::c_char>(),
                                ContextSize: ::std::mem::size_of::<[<WdfObject $context_type>]>(),
                                // SAFETY:
                                // StaticWrapper and UnsafeCell are both repr(transparent), so cast to underlying _WDF_OBJECT_CONTEXT_TYPE_INFO is ok
                                UniqueType: &[<_WDF_ $context_type _TYPE_INFO>] as *const _ as *const _,
                                EvtDriverGetUniqueContextType: ::std::option::Option::None,
                            }
                        ),

                        _phantom: ::std::marker::PhantomData
                    };

                /// Allows us to keep ONE main Arc allocation while handing out weak pointers to the rest of the clones.
                /// In this way, we can drop the allocation by dropping 1 arc, while letting others still access it
                enum ArcPointer<T> {
                    Strong(::std::sync::Arc<T>),
                    Weak(::std::sync::Weak<T>)
                }

                #[repr(transparent)]
                struct [<WdfObject $context_type>](ArcPointer<::std::sync::RwLock<$context_type>>);

                impl $context_type {
                    /// Initialize and place context into internal WdfObject
                    ///
                    /// SAFETY:
                    /// - handle must be a fresh unused object with no data in its context already
                    /// - context type must already have been set up for handle
                    /// - Must be set only once regardless of the object. For all other objects, use clone_into()
                    $sv unsafe fn init(
                        self,
                        handle: $crate::wdf_umdf_sys::WDFOBJECT,
                    ) -> ::std::result::Result<(), $crate::WdfError> {
                        let context = unsafe {
                            $crate::WdfObjectGetTypedContextWorker(handle, [<_WDF_ $context_type _TYPE_INFO>].cell.get())?
                        } as *mut ::std::mem::MaybeUninit<[<WdfObject $context_type>]>;

                        let context = &mut *context;

                        // Write to the memory location, making the data in it init
                        context.write(
                            [<WdfObject $context_type>](
                                ArcPointer::Strong(::std::sync::Arc::new(::std::sync::RwLock::new(self)))
                            )
                        );

                        Ok(())
                    }

                    /// Initialize handle's context and clone a Weak pointer to self context into it.
                    /// Internally, these are Arc's, so they will always point to the same data.
                    /// When the main Arc drops, none of these may access memory any longer
                    ///
                    /// SAFETY:
                    /// - handle must be a fresh unused object with no data in its context already
                    /// - to_handle must have set context_type for this type via WDF_OBJECT_ATTRIBUTES when it was created
                    /// - to_handle must be a valid T
                    $sv unsafe fn clone_into(
                        &self,
                        handle: $crate::wdf_umdf_sys::WDFOBJECT
                    ) -> ::std::result::Result<(), $crate::WdfError> {
                        let context = unsafe {
                            $crate::WdfObjectGetTypedContextWorker(handle, [<_WDF_ $context_type _TYPE_INFO>].cell.get())?
                        } as *mut ::std::mem::MaybeUninit<[<WdfObject $context_type>]>;

                        let context = &mut *context;

                        let from_context = unsafe {
                            $crate::WdfObjectGetTypedContextWorker(self.device as *mut _, [<_WDF_ $context_type _TYPE_INFO>].cell.get())?
                        } as *mut [<WdfObject $context_type>];

                        let from_context = match &(*from_context).0 {
                            ArcPointer::Strong(a) => a.clone(),
                            ArcPointer::Weak(a) => a.upgrade().ok_or($crate::WdfError::UpgradeFailed)?.clone(),
                        };

                        // Write to the memory location, making the data in it init
                        // clones the arc into new handle
                        context.write(
                            [<WdfObject $context_type>](ArcPointer::Weak(::std::sync::Arc::downgrade(&from_context)))
                        );

                        Ok(())
                    }

                    /// NOTE: Dropping memory that was created via `clone_into` will never drop the main allocation.
                    ///       To drop the main allocation, you need to drop the instance made via `init`.
                    ///       That instance can be obtained through the original handle you created it through
                    ///
                    /// SAFETY:
                    /// - Data in context is assumed to already be init and a valid T
                    /// - Therefore, init for the context must already have been done on this handle
                    /// - No other mutable/non-mutable refs can exist to data when this is called, or it will alias
                    ///
                    /// This may overwrite data in the handle's context memory, it is UB to read it after drop (e.g. get*)
                    $sv unsafe fn drop(
                        handle: $crate::wdf_umdf_sys::WDFOBJECT,
                    ) -> ::std::result::Result<(), $crate::WdfError> {
                        let context = $crate::WdfObjectGetTypedContextWorker(
                            handle,
                            [<_WDF_ $context_type _TYPE_INFO>].cell.get(),
                        )? as *mut [<WdfObject $context_type>];

                        // drop the memory
                        ::std::ptr::drop_in_place(context);

                        Ok(())
                    }

                    /// Borrow the context immutably
                    /// Function returns with error and won't call cb if it failed to lock
                    ///
                    /// SAFETY:
                    /// - Must have initialized WdfObject first
                    /// - Data must not have been dropped
                    /// - Object must not have been destroyed
                    $sv unsafe fn get<F>(
                        handle: *mut $crate::wdf_umdf_sys::WDFDEVICE__,
                        cb: F
                    ) -> ::std::result::Result<(), $crate::WdfError>
                    where
                        F: ::std::ops::FnOnce(&$context_type)
                    {
                        let context = unsafe {
                            $crate::WdfObjectGetTypedContextWorker(handle as *mut _,
                                // SAFETY: Reading is always fine, since user cannot obtain mutable reference
                                (&*[<_WDF_ $context_type _TYPE_INFO>].cell.get()).UniqueType
                            )?
                        } as *mut [<WdfObject $context_type>];

                        let context = &*context;

                        let context = match &context.0 {
                            ArcPointer::Strong(a) => a.clone(),
                            ArcPointer::Weak(a) => a.upgrade().ok_or($crate::WdfError::UpgradeFailed)?.clone(),
                        };

                        let guard = context.read().map_err(|_| $crate::WdfError::LockFailed)?;

                        cb(&*guard);

                        Ok(())
                    }

                    /// Borrow the context immutably
                    /// Function returns with error and won't call cb if it failed to lock
                    ///
                    /// SAFETY:
                    /// - Must have initialized WdfObject first
                    /// - Data must not have been dropped
                    /// - Object must not have been destroyed
                    $sv unsafe fn get_mut<F>(
                        handle: *mut $crate::wdf_umdf_sys::WDFDEVICE__,
                        cb: F
                    ) -> ::std::result::Result<(), $crate::WdfError>
                    where
                        F: ::std::ops::FnOnce(&mut $context_type)
                    {
                        let context = unsafe {
                            $crate::WdfObjectGetTypedContextWorker(handle as *mut _,
                                // SAFETY: Reading is always fine, since user cannot obtain mutable reference
                                (&*[<_WDF_ $context_type _TYPE_INFO>].cell.get()).UniqueType
                            )?
                        } as *mut [<WdfObject $context_type>];

                        let context = &*context;

                        let context = match &context.0 {
                            ArcPointer::Strong(a) => a.clone(),
                            ArcPointer::Weak(a) => a.upgrade().ok_or($crate::WdfError::UpgradeFailed)?.clone(),
                        };

                        let mut guard = context.write().map_err(|_| $crate::WdfError::LockFailed)?;

                        cb(&mut *guard);

                        Ok(())
                    }

                                        /// Borrow the context immutably
                    /// Function returns with error and won't call cb if it failed to lock
                    ///
                    /// SAFETY:
                    /// - Must have initialized WdfObject first
                    /// - Data must not have been dropped
                    /// - Object must not have been destroyed
                    $sv unsafe fn try_get<F>(
                        handle: *mut $crate::wdf_umdf_sys::WDFDEVICE__,
                        cb: F
                    ) -> ::std::result::Result<(), $crate::WdfError>
                    where
                        F: ::std::ops::FnOnce(&$context_type)
                    {
                        let context = unsafe {
                            $crate::WdfObjectGetTypedContextWorker(handle as *mut _,
                                // SAFETY: Reading is always fine, since user cannot obtain mutable reference
                                (&*[<_WDF_ $context_type _TYPE_INFO>].cell.get()).UniqueType
                            )?
                        } as *mut [<WdfObject $context_type>];

                        let context = &*context;

                        let context = match &context.0 {
                            ArcPointer::Strong(a) => a.clone(),
                            ArcPointer::Weak(a) => a.upgrade().ok_or($crate::WdfError::UpgradeFailed)?.clone(),
                        };

                        let guard = context.try_read().map_err(|_| $crate::WdfError::LockFailed)?;

                        cb(&*guard);

                        Ok(())
                    }

                    /// Try to borrow the context mutably. Immediately returns if it's locked
                    /// Function returns with error and won't call cb if it failed to lock
                    ///
                    /// SAFETY:
                    /// - Must have initialized WdfObject first
                    /// - Data must not have been dropped
                    /// - Object must not have been destroyed
                    $sv unsafe fn try_get_mut<F>(
                        handle: *mut $crate::wdf_umdf_sys::WDFDEVICE__,
                        cb: F
                    ) -> ::std::result::Result<(), $crate::WdfError>
                    where
                        F: ::std::ops::FnOnce(&mut $context_type)
                    {
                        let context = unsafe {
                            $crate::WdfObjectGetTypedContextWorker(handle as *mut _,
                                // SAFETY: Reading is always fine, since user cannot obtain mutable reference
                                (&*[<_WDF_ $context_type _TYPE_INFO>].cell.get()).UniqueType
                            )?
                        } as *mut [<WdfObject $context_type>];

                        let context = &*context;

                        let context = match &context.0 {
                            ArcPointer::Strong(a) => a.clone(),
                            ArcPointer::Weak(a) => a.upgrade().ok_or($crate::WdfError::UpgradeFailed)?.clone(),
                        };

                        let mut guard = context.try_write().map_err(|_| $crate::WdfError::LockFailed)?;

                        cb(&mut *guard);

                        Ok(())
                    }

                    // SAFETY:
                    // - No other mutable refs must exist to target type
                    // - Underlying memory must remain immutable and unchanged until reference is dropped
                    $sv unsafe fn get_type_info() -> &'static $crate::wdf_umdf_sys::_WDF_OBJECT_CONTEXT_TYPE_INFO {
                        unsafe { &*[<_WDF_ $context_type _TYPE_INFO>].cell.get() }
                    }
                }
            }
        }
    };
}

/// # Safety
///
/// None. User is responsible for safety.
pub unsafe fn WdfDriverCreate(
    // in
    DriverObject: PDRIVER_OBJECT,
    // in
    RegistryPath: PCUNICODE_STRING,
    // in, optional
    DriverAttributes: Option<PWDF_OBJECT_ATTRIBUTES>,
    // in
    DriverConfig: PWDF_DRIVER_CONFIG,
    // out, optional
    Driver: Option<&mut WDFDRIVER>,
) -> Result<NTSTATUS, WdfError> {
    WdfCall! {
        WdfDriverCreate(
            DriverObject,
            RegistryPath,
            DriverAttributes.unwrap_or(WDF_NO_OBJECT_ATTRIBUTES!()),
            DriverConfig,
            Driver
                .map(|d| d as *mut _)
                .unwrap_or(WDF_NO_HANDLE!())
        )
    }
    .into_result()
}

/// # Safety
///
/// None. User is responsible for safety.
pub unsafe fn WdfDeviceCreate(
    // in, out
    DeviceInit: &mut PWDFDEVICE_INIT,
    // in, optional
    DeviceAttributes: Option<&mut WDF_OBJECT_ATTRIBUTES>,
    // out
    Device: &mut WDFDEVICE,
) -> Result<NTSTATUS, WdfError> {
    WdfCall! {
        WdfDeviceCreate(
            DeviceInit,
            DeviceAttributes.map(|d| d as *mut _).unwrap_or(WDF_NO_OBJECT_ATTRIBUTES!() as *mut _),
            Device
        )
    }
    .into_result()
}

/// # Safety
///
/// None. User is responsible for safety.
pub unsafe fn WdfDeviceInitSetPnpPowerEventCallbacks(
    // in
    DeviceInit: PWDFDEVICE_INIT,
    // in
    PnpPowerEventCallbacks: *mut _WDF_PNPPOWER_EVENT_CALLBACKS,
) -> Result<(), WdfError> {
    WdfCall! {
        WdfDeviceInitSetPnpPowerEventCallbacks(
            DeviceInit,
            PnpPowerEventCallbacks
        )
    }
}

/// # Safety
///
/// None. User is responsible for safety.
pub unsafe fn WdfObjectGetTypedContextWorker(
    // in
    Handle: WDFOBJECT,
    // in
    TypeInfo: PCWDF_OBJECT_CONTEXT_TYPE_INFO,
) -> Result<*mut c_void, WdfError> {
    WdfCall! {
        WdfObjectGetTypedContextWorker(
            Handle,
            TypeInfo
        )
    }
}

/// # Safety
///
/// None. User is responsible for safety.
pub unsafe fn WdfObjectDelete(
    // in
    Object: WDFOBJECT,
) -> Result<(), WdfError> {
    WdfCall! {
        WdfObjectDelete(
            Object
        )
    }
}

/// # Safety
///
/// None. User is responsible for safety.
pub unsafe fn WdfDeviceSetFailed(
    // in
    Device: WDFDEVICE,
    // in
    FailedAction: WDF_DEVICE_FAILED_ACTION,
) -> Result<(), WdfError> {
    WdfCall! {
        WdfDeviceSetFailed(
            Device,
            FailedAction
        )
    }
}

/// # Safety
///
/// None. User is responsible for safety.
pub unsafe fn WdfDeviceAllocAndQueryPropertyEx(
    // in
    Device: WDFDEVICE,
    // in
    DeviceProperty: &mut _WDF_DEVICE_PROPERTY_DATA,
    // in
    PoolType: POOL_TYPE,
    // in, optional
    PropertyMemoryAttributes: PWDF_OBJECT_ATTRIBUTES,
    // out
    PropertyMemory: &mut WDFMEMORY,
    // out
    Type: &mut DEVPROPTYPE,
) -> Result<NTSTATUS, WdfError> {
    WdfCall! {
        WdfDeviceAllocAndQueryPropertyEx(
            Device,
            DeviceProperty,
            PoolType,
            PropertyMemoryAttributes,
            PropertyMemory,
            Type
        )
    }
}

/// # Safety
///
/// None. User is responsible for safety.
pub unsafe fn WdfMemoryGetBuffer(
    // in
    Memory: WDFMEMORY,
    // out, optional
    BufferSize: Option<&mut usize>,
) -> Result<*mut c_void, WdfError> {
    WdfCall! {
        WdfMemoryGetBuffer(
            Memory,
            BufferSize.map(|s| s as *mut _).unwrap_or(std::ptr::null_mut())
        )
    }
}
