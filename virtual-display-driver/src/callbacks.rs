use log::info;
use wdf_umdf_sys::{
    IDARG_IN_ADAPTER_INIT_FINISHED, IDARG_IN_COMMITMODES, IDARG_IN_GETDEFAULTDESCRIPTIONMODES,
    IDARG_IN_PARSEMONITORDESCRIPTION, IDARG_IN_QUERYTARGETMODES, IDARG_IN_SETSWAPCHAIN,
    IDARG_OUT_GETDEFAULTDESCRIPTIONMODES, IDARG_OUT_PARSEMONITORDESCRIPTION,
    IDARG_OUT_QUERYTARGETMODES, IDDCX_ADAPTER__, IDDCX_MONITOR__, NTSTATUS, WDFDEVICE,
    WDF_POWER_DEVICE_STATE,
};

use crate::device_context::DeviceContext;

pub extern "C-unwind" fn adapter_init_finished(
    adapter_object: *mut IDDCX_ADAPTER__,
    p_in_args: *const IDARG_IN_ADAPTER_INIT_FINISHED,
) -> NTSTATUS {
    info!("adapter_init_finished");
    todo!()
}

pub extern "C-unwind" fn device_d0_entry(
    device: WDFDEVICE,
    _previous_state: WDF_POWER_DEVICE_STATE,
) -> NTSTATUS {
    let Some(mut context) = (unsafe { DeviceContext::get_mut(device) }) else {
        return NTSTATUS::STATUS_NOT_FOUND;
    };

    context.init_adapter()
}

pub extern "C-unwind" fn parse_monitor_description(
    p_in_args: *const IDARG_IN_PARSEMONITORDESCRIPTION,
    p_out_args: *mut IDARG_OUT_PARSEMONITORDESCRIPTION,
) -> NTSTATUS {
    info!("parse_monitor_description");
    todo!()
}

pub extern "C-unwind" fn monitor_get_default_modes(
    _monitor_object: *mut IDDCX_MONITOR__,
    _p_in_args: *const IDARG_IN_GETDEFAULTDESCRIPTIONMODES,
    _p_out_args: *mut IDARG_OUT_GETDEFAULTDESCRIPTIONMODES,
) -> NTSTATUS {
    info!("monitor_get_default_modes");
    NTSTATUS::STATUS_NOT_IMPLEMENTED
}

pub extern "C-unwind" fn monitor_query_modes(
    monitor_object: *mut IDDCX_MONITOR__,
    p_in_args: *const IDARG_IN_QUERYTARGETMODES,
    p_out_args: *mut IDARG_OUT_QUERYTARGETMODES,
) -> NTSTATUS {
    info!("monitor_query_modes");
    todo!()
}

pub extern "C-unwind" fn adapter_commit_modes(
    adapter_object: *mut IDDCX_ADAPTER__,
    p_in_args: *const IDARG_IN_COMMITMODES,
) -> NTSTATUS {
    info!("adapter_commit_modes");
    todo!()
}

pub extern "C-unwind" fn assign_swap_chain(
    monitor_object: *mut IDDCX_MONITOR__,
    p_in_args: *const IDARG_IN_SETSWAPCHAIN,
) -> NTSTATUS {
    info!("assign_swap_chain");
    todo!()
}

pub extern "C-unwind" fn unassign_swap_chain(monitor_object: *mut IDDCX_MONITOR__) -> NTSTATUS {
    info!("unassign_swap_chain");
    todo!()
}
