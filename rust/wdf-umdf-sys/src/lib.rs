#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused)]

mod bindings;
mod ntstatus;

use std::fmt::{self, Display};

pub use bindings::*;
pub use ntstatus::*;
pub use paste::paste;

#[macro_export]
macro_rules! WdfIsFunctionAvailable {
    ($name:ident) => {{
        // SAFETY: We only ever do read access
        let higher = unsafe { $crate::WdfClientVersionHigherThanFramework } != 0;
        // SAFETY: We only ever do read access
        let fn_count = unsafe { $crate::WdfFunctionCount };

        // https://github.com/microsoft/Windows-Driver-Frameworks/blob/main/src/publicinc/wdf/umdf/2.33/wdffuncenum.h#L126
        $crate::paste! {
            // index is always positive, see
            // https://github.com/microsoft/Windows-Driver-Frameworks/blob/main/src/publicinc/wdf/umdf/2.33/wdffuncenum.h
            const FN_INDEX: u32 = $crate::WDFFUNCENUM::[<$name TableIndex>].0 as u32;

            FN_INDEX < $crate::WDF_ALWAYS_AVAILABLE_FUNCTION_COUNT
            || !higher || FN_INDEX < fn_count
        }
    }};
}

#[macro_export]
macro_rules! WdfIsStructureAvailable {
    ($name:ident) => {{
        // SAFETY: We only ever do read access
        let higher = unsafe { $crate::WdfClientVersionHigherThanFramework } != 0;
        // SAFETY: We only ever do read access
        let struct_count = unsafe { $crate::WdfStructureCount };

        // https://github.com/microsoft/Windows-Driver-Frameworks/blob/main/src/publicinc/wdf/umdf/2.33/wdffuncenum.h#L141
        $crate::paste! {
            // index is always positive, see
            // https://github.com/microsoft/Windows-Driver-Frameworks/blob/main/src/publicinc/wdf/umdf/2.33/wdffuncenum.h
            const STRUCT_INDEX: u32 = $crate::WDFSTRUCTENUM::[<INDEX_ $name>].0 as u32;

            !higher || STRUCT_INDEX < struct_count
        }
    }};
}

#[macro_export]
macro_rules! IddCxIsFunctionAvailable {
    ($name:ident) => {{
        // SAFETY: We only ever do read access
        let higher = unsafe { $crate::IddClientVersionHigherThanFramework } != 0;
        // SAFETY: We only ever do read access
        let fn_count = unsafe { $crate::IddFunctionCount };

        $crate::paste! {
            const FN_INDEX: u32 = $crate::IDDFUNCENUM::[<$name TableIndex>].0 as u32;

            FN_INDEX < $crate::IDD_ALWAYS_AVAILABLE_FUNCTION_COUNT
            || !higher || FN_INDEX < fn_count
        }
    }};
}

#[macro_export]
macro_rules! IddCxIsStructureAvailable {
    ($name:ident) => {{
        // SAFETY: We only ever do read access
        let higher = unsafe { $crate::IddClientVersionHigherThanFramework } != 0;
        // SAFETY: We only ever do read access
        let struct_count = unsafe { $crate::IddStructureCount };

        $crate::paste! {
            const STRUCT_INDEX: u32 = $crate::IDDSTRUCTENUM::[<INDEX_ $name>].0 as u32;

            !higher || STRUCT_INDEX < struct_count
        }
    }};
}

macro_rules! WDF_STRUCTURE_SIZE {
    ($name:ty) => {
        ::core::mem::size_of::<$name>() as u32
    };
}

#[macro_export]
macro_rules! WDF_NO_HANDLE {
    () => {
        ::core::ptr::null_mut()
    };
}

#[macro_export]
macro_rules! WDF_NO_OBJECT_ATTRIBUTES {
    () => {
        ::core::ptr::null_mut()
    };
}

#[macro_export]
macro_rules! WDF_OBJECT_ATTRIBUTES_SET_CONTEXT_TYPE {
    ($attr:ident, $context_type:ident) => {
        $attr.ContextTypeInfo = $context_type;
    };
}

impl WDF_OBJECT_ATTRIBUTES {
    /// Initializes the [`WDF_OBJECT_ATTRIBUTES`] structure
    /// <https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.33/wdfobject.h#L136/>
    ///
    /// Sets
    /// - `ExecutionLevel` to [`WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent`]
    /// - `SynchronizationScope` to [`WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent`]
    #[must_use]
    pub fn init() -> Self {
        // SAFETY: All fields are zero-able
        let mut attributes: Self = unsafe { ::core::mem::zeroed() };

        attributes.Size = WDF_STRUCTURE_SIZE!(Self);
        attributes.SynchronizationScope =
            WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent;
        attributes.ExecutionLevel = WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent;

        attributes
    }

    #[must_use]
    pub fn init_context_type(context_type: &_WDF_OBJECT_CONTEXT_TYPE_INFO) -> Self {
        let mut attr = Self::init();

        WDF_OBJECT_ATTRIBUTES_SET_CONTEXT_TYPE!(attr, context_type);

        attr
    }
}

impl WDF_DRIVER_CONFIG {
    /// Initializes the [`WDF_DRIVER_CONFIG`] structure
    /// <https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.33/wdfdriver.h#L134/>
    #[must_use]
    pub fn init(EvtDriverDeviceAdd: PFN_WDF_DRIVER_DEVICE_ADD) -> Self {
        // SAFETY: All fields are zero-able
        let mut config: Self = unsafe { core::mem::zeroed() };

        config.Size = WDF_STRUCTURE_SIZE!(Self);

        config.EvtDriverDeviceAdd = EvtDriverDeviceAdd;

        config
    }
}

impl WDF_PNPPOWER_EVENT_CALLBACKS {
    /// Initializes the [`WDF_PNPPOWER_EVENT_CALLBACKS`] structure
    /// <https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.33/wdfdevice.h#L1278/>
    #[must_use]
    pub fn init() -> Self {
        // SAFETY: All fields are zero-able
        let mut callbacks: Self = unsafe { core::mem::zeroed() };
        callbacks.Size = WDF_STRUCTURE_SIZE!(Self);

        callbacks
    }
}

/// If this returns None, the struct is NOT available to be used
macro_rules! IDD_STRUCTURE_SIZE {
    ($name:ty) => {{
        // SAFETY: We only ever do read access, copy is fine
        let higher = unsafe { IddClientVersionHigherThanFramework } != 0;
        // SAFETY: We only ever do read access, copy is fine
        let struct_count = unsafe { IddStructureCount };

        if higher {
            // as u32 is fine, since there's no way there's > 4 billion structs
            const STRUCT_INDEX: u32 =
                $crate::paste! { IDDSTRUCTENUM::[<INDEX_ $name:upper>].0 as u32 };

            // SAFETY: A pointer to a [size_t], copying the pointer is ok
            let ptr = unsafe { IddStructures };

            if STRUCT_INDEX < struct_count {
                // SAFETY: we validated struct index is able to be accessed
                let ptr = unsafe { ptr.add(STRUCT_INDEX as usize) };
                // SAFETY: So it's ok to read
                Some(unsafe { ptr.read() } as u32)
            } else {
                // struct CANNOT be used
                None
            }
        } else {
            Some(::std::mem::size_of::<$name>() as u32)
        }
    }};
}

impl IDD_CX_CLIENT_CONFIG {
    #[must_use]
    pub fn init() -> Option<Self> {
        // SAFETY: All fields are zero-able
        let mut config: Self = unsafe { core::mem::zeroed() };

        config.Size = IDD_STRUCTURE_SIZE!(IDD_CX_CLIENT_CONFIG)?;

        Some(config)
    }
}
