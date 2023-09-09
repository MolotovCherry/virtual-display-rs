use std::mem;

use log::info;
use wdf_umdf_sys::{
    DISPLAYCONFIG_VIDEO_SIGNAL_INFO__bindgen_ty_1,
    DISPLAYCONFIG_VIDEO_SIGNAL_INFO__bindgen_ty_1__bindgen_ty_1, DISPLAYCONFIG_2DREGION,
    DISPLAYCONFIG_RATIONAL, DISPLAYCONFIG_SCANLINE_ORDERING, DISPLAYCONFIG_VIDEO_SIGNAL_INFO,
    IDARG_IN_ADAPTER_INIT_FINISHED, IDARG_IN_COMMITMODES, IDARG_IN_GETDEFAULTDESCRIPTIONMODES,
    IDARG_IN_PARSEMONITORDESCRIPTION, IDARG_IN_QUERYTARGETMODES, IDARG_IN_SETSWAPCHAIN,
    IDARG_OUT_GETDEFAULTDESCRIPTIONMODES, IDARG_OUT_PARSEMONITORDESCRIPTION,
    IDARG_OUT_QUERYTARGETMODES, IDDCX_ADAPTER__, IDDCX_MONITOR_MODE, IDDCX_MONITOR_MODE_ORIGIN,
    IDDCX_MONITOR__, NTSTATUS, WDFDEVICE, WDF_POWER_DEVICE_STATE,
};

use crate::device_context::{DeviceContext, MAX_MONITORS};

pub extern "C-unwind" fn adapter_init_finished(
    adapter_object: *mut IDDCX_ADAPTER__,
    _p_in_args: *const IDARG_IN_ADAPTER_INIT_FINISHED,
) -> NTSTATUS {
    let Some(mut context) = (unsafe { DeviceContext::get_mut(adapter_object as *mut _) }) else {
        return NTSTATUS::STATUS_NOT_FOUND;
    };

    context.finish_init()
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

fn display_info(
    horizontal: u32,
    vertical: u32,
    refresh_rate: u32,
) -> DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    let clock_rate = refresh_rate * (vertical + 4) * (vertical + 4) + 1000;

    DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
        pixelRate: clock_rate as u64,
        hSyncFreq: DISPLAYCONFIG_RATIONAL {
            Numerator: clock_rate,
            Denominator: vertical + 4,
        },
        vSyncFreq: DISPLAYCONFIG_RATIONAL {
            Numerator: clock_rate,
            Denominator: (vertical + 4) * (vertical + 4),
        },
        activeSize: DISPLAYCONFIG_2DREGION {
            cx: horizontal,
            cy: vertical,
        },
        totalSize: DISPLAYCONFIG_2DREGION {
            cx: horizontal + 4,
            cy: horizontal + 4,
        },
        __bindgen_anon_1: DISPLAYCONFIG_VIDEO_SIGNAL_INFO__bindgen_ty_1 {
            AdditionalSignalInfo: unsafe {
                mem::transmute(
                    DISPLAYCONFIG_VIDEO_SIGNAL_INFO__bindgen_ty_1__bindgen_ty_1::new_bitfield_1(
                        255, 0, 0,
                    ),
                )
            },
        },
        scanLineOrdering:
            DISPLAYCONFIG_SCANLINE_ORDERING::DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE,
    }
}

pub extern "C-unwind" fn parse_monitor_description(
    p_in_args: *const IDARG_IN_PARSEMONITORDESCRIPTION,
    p_out_args: *mut IDARG_OUT_PARSEMONITORDESCRIPTION,
) -> NTSTATUS {
    let in_args = unsafe { &*p_in_args };
    let out_args = unsafe { &mut *p_out_args };

    let monitor_count = 1;

    out_args.MonitorModeBufferOutputCount = monitor_count;
    if in_args.MonitorModeBufferInputCount < monitor_count {
        // Return success if there was no buffer, since the caller was only asking for a count of modes
        return if in_args.MonitorModeBufferInputCount > 0 {
            NTSTATUS::STATUS_BUFFER_TOO_SMALL
        } else {
            NTSTATUS::STATUS_SUCCESS
        };
    } else {
        //

        let monitor_modes = unsafe {
            &mut *in_args
                .pMonitorModes
                .cast::<[IDDCX_MONITOR_MODE; MAX_MONITORS as usize]>()
        };

        for mode in monitor_modes.iter_mut().take(monitor_count as usize) {
            mode.Size = mem::size_of::<IDDCX_MONITOR_MODE>() as u32;
            mode.Origin = IDDCX_MONITOR_MODE_ORIGIN::IDDCX_MONITOR_MODE_ORIGIN_MONITORDESCRIPTOR;
            mode.MonitorVideoSignalInfo = display_info(1920, 1080, 120);
        }

        // Set the preferred mode as represented in the EDID
        out_args.PreferredMonitorModeIdx = 0;
    }

    NTSTATUS::STATUS_SUCCESS
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
    _monitor_object: *mut IDDCX_MONITOR__,
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
