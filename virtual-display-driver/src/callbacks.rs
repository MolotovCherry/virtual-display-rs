use std::{
    mem::{self, MaybeUninit},
    ptr::NonNull,
};

use wdf_umdf::IntoHelper;
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

use crate::{
    context::{DeviceContext, MonitorContext},
    edid::get_edid_serial,
    ipc::{AdapterObject, ADAPTER, MONITOR_MODES},
};

pub extern "C-unwind" fn adapter_init_finished(
    adapter_object: *mut IDDCX_ADAPTER__,
    _p_in_args: *const IDARG_IN_ADAPTER_INIT_FINISHED,
) -> NTSTATUS {
    // store adapter object for listener to use
    ADAPTER
        .set(AdapterObject(NonNull::new(adapter_object).unwrap()))
        .unwrap();

    DeviceContext::finish_init();

    NTSTATUS::STATUS_SUCCESS
}

pub extern "C-unwind" fn device_d0_entry(
    device: WDFDEVICE,
    _previous_state: WDF_POWER_DEVICE_STATE,
) -> NTSTATUS {
    let status = unsafe {
        DeviceContext::get_mut(device.cast(), |context| {
            context.init_adapter();
        })
        .into_status()
    };

    if !status.is_success() {
        return status;
    }

    NTSTATUS::STATUS_SUCCESS
}

fn display_info(width: u32, height: u32, refresh_rate: u32) -> DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    let clock_rate = refresh_rate * (height + 4) * (height + 4) + 1000;

    DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
        pixelRate: u64::from(clock_rate),
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

    let monitors = MONITOR_MODES.get().unwrap().lock().unwrap();

    let edid = unsafe {
        std::slice::from_raw_parts(
            in_args.MonitorDescription.pData as *const u8,
            in_args.MonitorDescription.DataSize as usize,
        )
    };

    let monitor_index = get_edid_serial(edid);

    let monitor = monitors
        .iter()
        .find(|&m| m.monitor.id == monitor_index)
        .expect("to be found");

    let number_of_modes: u32 = monitor
        .monitor
        .modes
        .iter()
        .map(|m| u32::try_from(m.refresh_rates.len()).expect("Cannot use > u32::MAX modes"))
        .sum();

    out_args.MonitorModeBufferOutputCount = number_of_modes;
    if in_args.MonitorModeBufferInputCount < number_of_modes {
        // Return success if there was no buffer, since the caller was only asking for a count of modes
        return if in_args.MonitorModeBufferInputCount > 0 {
            NTSTATUS::STATUS_BUFFER_TOO_SMALL
        } else {
            NTSTATUS::STATUS_SUCCESS
        };
    }

    let monitor_modes = unsafe {
        std::slice::from_raw_parts_mut(
            in_args
                .pMonitorModes
                .cast::<MaybeUninit<IDDCX_MONITOR_MODE>>(),
            number_of_modes as usize,
        )
    };

    let mut monitor_modes_iter = monitor_modes.iter_mut();

    for mode in &monitor.monitor.modes {
        // create a new iterator over N next elements of the iterator
        let next_n = monitor_modes_iter
            .by_ref()
            .take(mode.refresh_rates.len())
            .zip(&mode.refresh_rates);

        for (out_mode, &refresh_rate) in next_n {
            out_mode.write(IDDCX_MONITOR_MODE {
                Size: u32::try_from(mem::size_of::<IDDCX_MONITOR_MODE>()).unwrap(),
                Origin: IDDCX_MONITOR_MODE_ORIGIN::IDDCX_MONITOR_MODE_ORIGIN_MONITORDESCRIPTOR,
                MonitorVideoSignalInfo: display_info(mode.width, mode.height, refresh_rate),
            });
        }
    }

    // Set the preferred mode as represented in the EDID
    out_args.PreferredMonitorModeIdx = 0;

    NTSTATUS::STATUS_SUCCESS
}

pub extern "C-unwind" fn monitor_get_default_modes(
    _monitor_object: *mut IDDCX_MONITOR__,
    _p_in_args: *const IDARG_IN_GETDEFAULTDESCRIPTIONMODES,
    _p_out_args: *mut IDARG_OUT_GETDEFAULTDESCRIPTIONMODES,
) -> NTSTATUS {
    NTSTATUS::STATUS_NOT_IMPLEMENTED
}

pub fn target_mode(width: u32, height: u32, refresh_rate: u32) -> IDDCX_TARGET_MODE {
    let total_size = DISPLAYCONFIG_2DREGION {
        cx: width,
        cy: height,
    };

    IDDCX_TARGET_MODE {
        Size: u32::try_from(mem::size_of::<IDDCX_TARGET_MODE>()).unwrap(),

        TargetVideoSignalInfo: DISPLAYCONFIG_TARGET_MODE {
            targetVideoSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
                pixelRate: u64::from(refresh_rate) * u64::from(width) * u64::from(height),
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
    }
}

pub extern "C-unwind" fn monitor_query_modes(
    monitor_object: *mut IDDCX_MONITOR__,
    p_in_args: *const IDARG_IN_QUERYTARGETMODES,
    p_out_args: *mut IDARG_OUT_QUERYTARGETMODES,
) -> NTSTATUS {
    // find out which monitor this belongs too

    let monitors = MONITOR_MODES.get().unwrap().lock().unwrap();

    // we have stored the monitor object per id, so we should be able to compare pointers
    let monitor = monitors
        .iter()
        .find(|&m| m.monitor_object.unwrap().as_ptr() == monitor_object)
        .unwrap();

    let number_of_modes = monitor
        .monitor
        .modes
        .iter()
        .map(|m| u32::try_from(m.refresh_rates.len()).expect("Cannot use > u32::MAX modes"))
        .sum();

    // Create a set of modes supported for frame processing and scan-out. These are typically not based on the
    // monitor's descriptor and instead are based on the static processing capability of the device. The OS will
    // report the available set of modes for a given output as the intersection of monitor modes with target modes.

    let out_args = unsafe { &mut *p_out_args };
    out_args.TargetModeBufferOutputCount = number_of_modes;

    let in_args = unsafe { &*p_in_args };

    if in_args.TargetModeBufferInputCount >= number_of_modes {
        let out_target_modes = unsafe {
            std::slice::from_raw_parts_mut(
                in_args
                    .pTargetModes
                    .cast::<MaybeUninit<IDDCX_TARGET_MODE>>(),
                number_of_modes as usize,
            )
        };

        let mut out_target_modes_iter = out_target_modes.iter_mut();

        for mode in &monitor.monitor.modes {
            // create a new iterator over N next elements of the iterator
            let next_n = out_target_modes_iter
                .by_ref()
                .take(mode.refresh_rates.len())
                .zip(&mode.refresh_rates);

            for (out_target, &refresh_rate) in next_n {
                let target_mode = target_mode(mode.width, mode.height, refresh_rate);
                out_target.write(target_mode);
            }
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
    let p_in_args = unsafe { &*p_in_args };

    unsafe {
        MonitorContext::get_mut(monitor_object.cast(), |context| {
            context.assign_swap_chain(
                p_in_args.hSwapChain,
                p_in_args.RenderAdapterLuid,
                p_in_args.hNextSurfaceAvailable,
            );
        })
    }
    .into_status()
}

pub extern "C-unwind" fn unassign_swap_chain(monitor_object: *mut IDDCX_MONITOR__) -> NTSTATUS {
    unsafe {
        MonitorContext::get_mut(monitor_object.cast(), |context| {
            context.unassign_swap_chain();
        })
    }
    .into_status()
}
