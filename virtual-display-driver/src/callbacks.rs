use std::{
    mem::{self, MaybeUninit},
    ptr::NonNull,
};

use log::error;
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
    edid::Edid,
    ipc::{AdapterObject, FlattenModes, ADAPTER, MONITOR_MODES},
};

pub extern "C-unwind" fn adapter_init_finished(
    adapter_object: *mut IDDCX_ADAPTER__,
    _p_in_args: *const IDARG_IN_ADAPTER_INIT_FINISHED,
) -> NTSTATUS {
    let Some(adapter_ptr) = NonNull::new(adapter_object) else {
        error!("Adapter ptr was null");
        return NTSTATUS::STATUS_INVALID_ADDRESS;
    };

    // store adapter object for listener to use
    if ADAPTER.set(AdapterObject(adapter_ptr)).is_err() {
        error!("Failed to set adapter");
        return NTSTATUS::STATUS_ADAPTER_HARDWARE_ERROR;
    }

    DeviceContext::finish_init();

    NTSTATUS::STATUS_SUCCESS
}

pub extern "C-unwind" fn device_d0_entry(
    device: WDFDEVICE,
    _previous_state: WDF_POWER_DEVICE_STATE,
) -> NTSTATUS {
    let status: NTSTATUS = unsafe {
        DeviceContext::get_mut(device.cast(), |context| {
            if let Err(e) = context.init_adapter() {
                error!("Failed to init adapter: {e:?}");
            }
        })
        .into()
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

    let Some(monitors) = MONITOR_MODES.get() else {
        error!("Failed to get monitor oncelock data");
        return NTSTATUS::STATUS_DRIVER_INTERNAL_ERROR;
    };
    let Ok(monitors) = monitors.lock() else {
        error!("MONITOR_MODES mutex poisoned");
        return NTSTATUS::STATUS_DRIVER_INTERNAL_ERROR;
    };

    let edid = unsafe {
        std::slice::from_raw_parts(
            in_args.MonitorDescription.pData as *const u8,
            in_args.MonitorDescription.DataSize as usize,
        )
    };

    let monitor_index = Edid::get_serial(edid);
    let Ok(monitor_index) = monitor_index else {
        error!(
            "We got an edid {} bytes long, but this is incorrect",
            edid.len()
        );
        return NTSTATUS::STATUS_INVALID_VIEW_SIZE;
    };

    let Some(monitor) = monitors.iter().find(|&m| m.monitor.id == monitor_index) else {
        error!("Failed to find monitor id {monitor_index}");
        return NTSTATUS::STATUS_DRIVER_INTERNAL_ERROR;
    };

    let number_of_modes: u32 = monitor
        .monitor
        .modes
        .iter()
        .map(|m| u32::try_from(m.refresh_rates.len()).expect("Cannot use > u32::MAX refresh rates"))
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

    for (mode, out_mode) in monitor
        .monitor
        .modes
        .flatten()
        .zip(monitor_modes.iter_mut())
    {
        out_mode.write(IDDCX_MONITOR_MODE {
            #[allow(clippy::cast_possible_truncation)]
            Size: mem::size_of::<IDDCX_MONITOR_MODE>() as u32,
            Origin: IDDCX_MONITOR_MODE_ORIGIN::IDDCX_MONITOR_MODE_ORIGIN_MONITORDESCRIPTOR,
            MonitorVideoSignalInfo: display_info(mode.width, mode.height, mode.refresh_rate),
        });
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
        #[allow(clippy::cast_possible_truncation)]
        Size: mem::size_of::<IDDCX_TARGET_MODE>() as u32,

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

    let Some(monitors) = MONITOR_MODES.get() else {
        error!("Failed to get monitor oncelock data");
        return NTSTATUS::STATUS_DRIVER_INTERNAL_ERROR;
    };
    let Ok(monitors) = monitors.lock() else {
        error!("MONITOR_MODES mutex poisoned");
        return NTSTATUS::STATUS_DRIVER_INTERNAL_ERROR;
    };

    // we have stored the monitor object per id, so we should be able to compare pointers
    let Some(monitor) = monitors.iter().find(|&m| {
        m.monitor_object
            .is_some_and(|p| p.as_ptr() == monitor_object)
    }) else {
        error!("Failed to find monitor object in cache for {monitor_object:?}");
        return NTSTATUS::STATUS_DRIVER_INTERNAL_ERROR;
    };

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

        for (mode, out_target) in monitor
            .monitor
            .modes
            .flatten()
            .zip(out_target_modes.iter_mut())
        {
            let target_mode = target_mode(mode.width, mode.height, mode.refresh_rate);

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
    let p_in_args = unsafe { &*p_in_args };

    unsafe {
        MonitorContext::get_mut(monitor_object.cast(), |context| {
            context.assign_swap_chain(
                p_in_args.hSwapChain,
                p_in_args.RenderAdapterLuid,
                p_in_args.hNextSurfaceAvailable,
            );
        })
        .into()
    }
}

pub extern "C-unwind" fn unassign_swap_chain(monitor_object: *mut IDDCX_MONITOR__) -> NTSTATUS {
    unsafe {
        MonitorContext::get_mut(monitor_object.cast(), |context| {
            context.unassign_swap_chain();
        })
        .into()
    }
}
