use std::{
    mem::{self, MaybeUninit},
    sync::{
        atomic::{AtomicU32, Ordering},
        OnceLock,
    },
};

use log::info;
use wdf_umdf_sys::{
    DISPLAYCONFIG_VIDEO_SIGNAL_INFO__bindgen_ty_1,
    DISPLAYCONFIG_VIDEO_SIGNAL_INFO__bindgen_ty_1__bindgen_ty_1, DISPLAYCONFIG_2DREGION,
    DISPLAYCONFIG_RATIONAL, DISPLAYCONFIG_SCANLINE_ORDERING, DISPLAYCONFIG_TARGET_MODE,
    DISPLAYCONFIG_VIDEO_SIGNAL_INFO, IDARG_IN_ADAPTER_INIT_FINISHED, IDARG_IN_COMMITMODES,
    IDARG_IN_GETDEFAULTDESCRIPTIONMODES, IDARG_IN_PARSEMONITORDESCRIPTION,
    IDARG_IN_QUERYTARGETMODES, IDARG_IN_SETSWAPCHAIN, IDARG_OUT_GETDEFAULTDESCRIPTIONMODES,
    IDARG_OUT_PARSEMONITORDESCRIPTION, IDARG_OUT_QUERYTARGETMODES, IDDCX_ADAPTER__,
    IDDCX_MONITOR_MODE, IDDCX_MONITOR_MODE_ORIGIN, IDDCX_MONITOR__, IDDCX_TARGET_MODE, NTSTATUS,
    WDFDEVICE, WDF_POWER_DEVICE_STATE,
};

use crate::device_context::DeviceContext;

static MONITOR_MODES: OnceLock<Vec<MonitorMode>> = OnceLock::new();
pub static MONITOR_COUNT: AtomicU32 = AtomicU32::new(0);

#[derive(Debug)]
struct MonitorMode {
    width: u32,
    height: u32,
    refresh_rate: u32,
}

pub fn load_monitors() {
    MONITOR_MODES
        .set(vec![MonitorMode {
            width: 1920,
            height: 1080,
            refresh_rate: 120,
        }])
        .unwrap();

    MONITOR_COUNT.store(1, Ordering::Relaxed);
}

pub extern "C-unwind" fn adapter_init_finished(
    adapter_object: *mut IDDCX_ADAPTER__,
    _p_in_args: *const IDARG_IN_ADAPTER_INIT_FINISHED,
) -> NTSTATUS {
    let Some(context) = (unsafe { DeviceContext::get(adapter_object as *mut _) }) else {
        return NTSTATUS::STATUS_NOT_FOUND;
    };

    let mut context = context.write().unwrap();

    context.finish_init()
}

pub extern "C-unwind" fn device_d0_entry(
    device: WDFDEVICE,
    _previous_state: WDF_POWER_DEVICE_STATE,
) -> NTSTATUS {
    let Some(context) = (unsafe { DeviceContext::get(device) }) else {
        return NTSTATUS::STATUS_NOT_FOUND;
    };

    let mut context = context.write().unwrap();

    context.init_adapter()
}

fn display_info(width: u32, height: u32, refresh_rate: u32) -> DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    let clock_rate = refresh_rate * (height + 4) * (height + 4) + 1000;

    DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
        pixelRate: clock_rate as u64,
        hSyncFreq: DISPLAYCONFIG_RATIONAL {
            Numerator: clock_rate,
            Denominator: height + 4,
        },
        vSyncFreq: DISPLAYCONFIG_RATIONAL {
            Numerator: clock_rate,
            Denominator: (height + 4) * (height + 4),
        },
        activeSize: DISPLAYCONFIG_2DREGION {
            cx: width,
            cy: height,
        },
        totalSize: DISPLAYCONFIG_2DREGION {
            cx: width + 4,
            cy: height + 4,
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

    let monitor_count = MONITOR_COUNT.load(Ordering::Relaxed);

    out_args.MonitorModeBufferOutputCount = monitor_count;
    if in_args.MonitorModeBufferInputCount < monitor_count {
        // Return success if there was no buffer, since the caller was only asking for a count of modes
        return if in_args.MonitorModeBufferInputCount > 0 {
            NTSTATUS::STATUS_BUFFER_TOO_SMALL
        } else {
            NTSTATUS::STATUS_SUCCESS
        };
    } else {
        let monitor_modes = unsafe {
            std::slice::from_raw_parts_mut(
                in_args
                    .pMonitorModes
                    .cast::<MaybeUninit<IDDCX_MONITOR_MODE>>(),
                monitor_count as usize,
            )
        };

        for (out_mode, in_mode) in monitor_modes.iter_mut().zip(MONITOR_MODES.get().unwrap()) {
            out_mode.write(IDDCX_MONITOR_MODE {
                Size: mem::size_of::<IDDCX_MONITOR_MODE>() as u32,
                Origin: IDDCX_MONITOR_MODE_ORIGIN::IDDCX_MONITOR_MODE_ORIGIN_MONITORDESCRIPTOR,
                MonitorVideoSignalInfo: display_info(
                    in_mode.width,
                    in_mode.height,
                    in_mode.refresh_rate,
                ),
            });
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
    NTSTATUS::STATUS_NOT_IMPLEMENTED
}

pub extern "C-unwind" fn monitor_query_modes(
    _monitor_object: *mut IDDCX_MONITOR__,
    p_in_args: *const IDARG_IN_QUERYTARGETMODES,
    p_out_args: *mut IDARG_OUT_QUERYTARGETMODES,
) -> NTSTATUS {
    let monitor_count = MONITOR_COUNT.load(Ordering::Relaxed);

    // Create a set of modes supported for frame processing and scan-out. These are typically not based on the
    // monitor's descriptor and instead are based on the static processing capability of the device. The OS will
    // report the available set of modes for a given output as the intersection of monitor modes with target modes.

    let out_args = unsafe { &mut *p_out_args };
    out_args.TargetModeBufferOutputCount = monitor_count as u32;

    let in_args = unsafe { &*p_in_args };

    if in_args.TargetModeBufferInputCount >= monitor_count as u32 {
        let out_target_modes = unsafe {
            std::slice::from_raw_parts_mut(
                in_args
                    .pTargetModes
                    .cast::<MaybeUninit<IDDCX_TARGET_MODE>>(),
                monitor_count as usize,
            )
        };

        for (
            &MonitorMode {
                width,
                height,
                refresh_rate,
            },
            out_target,
        ) in MONITOR_MODES.get().unwrap().iter().zip(out_target_modes)
        {
            let total_size = DISPLAYCONFIG_2DREGION {
                cx: width,
                cy: height,
            };

            let target_mode = IDDCX_TARGET_MODE {
                Size: mem::size_of::<IDDCX_TARGET_MODE>() as u32,

                TargetVideoSignalInfo: DISPLAYCONFIG_TARGET_MODE {
                    targetVideoSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
                        pixelRate: refresh_rate as u64 * width as u64 * height as u64,
                        hSyncFreq: DISPLAYCONFIG_RATIONAL {
                            Numerator: refresh_rate * height,
                            Denominator: 1,
                        },
                        vSyncFreq: DISPLAYCONFIG_RATIONAL {
                            Numerator: refresh_rate,
                            Denominator: 1,
                        },
                        totalSize: total_size,
                        activeSize: total_size,
                        scanLineOrdering:
                            DISPLAYCONFIG_SCANLINE_ORDERING::DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE,
                        __bindgen_anon_1: DISPLAYCONFIG_VIDEO_SIGNAL_INFO__bindgen_ty_1 {
                            AdditionalSignalInfo: unsafe {
                                mem::transmute(
                                    DISPLAYCONFIG_VIDEO_SIGNAL_INFO__bindgen_ty_1__bindgen_ty_1::new_bitfield_1(
                                        255, 1, 0,
                                    ),
                                )
                            },
                        },
                    },
                },

                ..Default::default()
            };

            out_target.write(target_mode);
        }
    }

    NTSTATUS::STATUS_SUCCESS
}

pub extern "C-unwind" fn adapter_commit_modes(
    _adapter_object: *mut IDDCX_ADAPTER__,
    _p_in_args: *const IDARG_IN_COMMITMODES,
) -> NTSTATUS {
    NTSTATUS::STATUS_SUCCESS
}

pub extern "C-unwind" fn assign_swap_chain(
    monitor_object: *mut IDDCX_MONITOR__,
    p_in_args: *const IDARG_IN_SETSWAPCHAIN,
) -> NTSTATUS {
    // let Some(context) = (unsafe { DeviceContext::get(monitor_object as *mut _) }) else {
    //     return NTSTATUS::STATUS_NOT_FOUND;
    // };
    // let a = *context;

    NTSTATUS::STATUS_SUCCESS
}

pub extern "C-unwind" fn unassign_swap_chain(monitor_object: *mut IDDCX_MONITOR__) -> NTSTATUS {
    info!("unassign_swap_chain");
    todo!()
}
