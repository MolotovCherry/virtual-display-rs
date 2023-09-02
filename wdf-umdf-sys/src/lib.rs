#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused)]

mod bindings;
pub use bindings::*;
pub use paste::paste;

#[macro_export]
macro_rules! WdfIsFunctionAvailable {
    ($name:ident) => {{
        // SAFETY: We only ever do read access
        let higher = unsafe { $crate::WdfClientVersionHigherThanFramework } != 0;
        // SAFETY: We only ever do read access
        let fn_count = unsafe { $crate::WdfFunctionCount };

        // https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.25/wdffuncenum.h#L81
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

        // https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.25/wdffuncenum.h#L141
        $crate::paste! {
            // index is always positive, see
            // https://github.com/microsoft/Windows-Driver-Frameworks/blob/main/src/publicinc/wdf/umdf/2.33/wdffuncenum.h
            const STRUCT_INDEX: u32 = $crate::WDFSTRUCTENUM::[<INDEX_ $name>].0 as u32;

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

impl WDF_OBJECT_ATTRIBUTES {
    /// Initializes the [`WDF_OBJECT_ATTRIBUTES`] structure
    /// https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.27/wdfobject.h#L134
    ///
    /// Sets
    /// - `ExecutionLevel` to [`WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent`]
    /// - `SynchronizationScope` to [`WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent`]
    #[must_use]
    pub fn init() -> Self {
        // SAFETY: All fields are zero-able
        let mut attributes: Self = unsafe { ::core::mem::zeroed() };

        attributes.Size = WDF_STRUCTURE_SIZE!(WDF_OBJECT_ATTRIBUTES);
        attributes.SynchronizationScope =
            WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent;
        attributes.ExecutionLevel = WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent;

        attributes
    }
}

impl WDF_DRIVER_CONFIG {
    /// Initializes the [`WDF_DRIVER_CONFIG`] structure
    /// https://github.com/microsoft/Windows-Driver-Frameworks/blob/a94b8c30dad524352fab90872aefc83920b98e56/src/publicinc/wdf/umdf/2.23/wdfdriver.h#L131
    #[must_use]
    pub fn init(EvtDriverDeviceAdd: PFN_WDF_DRIVER_DEVICE_ADD) -> Self {
        // SAFETY: All fields are zero-able
        let mut config: Self = unsafe { core::mem::zeroed() };

        config.Size = WDF_STRUCTURE_SIZE!(WDF_DRIVER_CONFIG);

        config.EvtDriverDeviceAdd = EvtDriverDeviceAdd;

        config
    }
}
