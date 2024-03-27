use std::{
    ffi::c_void,
    mem::{self, size_of},
    num::{ParseIntError, TryFromIntError},
    ptr::{addr_of_mut, NonNull},
};

use anyhow::anyhow;
use log::{error, warn};
use wdf_umdf::{
    IddCxAdapterInitAsync, IddCxError, IddCxMonitorArrival, IddCxMonitorCreate,
    IddCxMonitorSetupHardwareCursor, WdfError, WdfObjectDelete, WDF_DECLARE_CONTEXT_TYPE,
};
use wdf_umdf_sys::{
    DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY, HANDLE, IDARG_IN_ADAPTER_INIT, IDARG_IN_MONITORCREATE,
    IDARG_IN_SETUP_HWCURSOR, IDARG_OUT_ADAPTER_INIT, IDARG_OUT_MONITORARRIVAL,
    IDARG_OUT_MONITORCREATE, IDDCX_ADAPTER, IDDCX_ADAPTER_CAPS, IDDCX_CURSOR_CAPS,
    IDDCX_ENDPOINT_DIAGNOSTIC_INFO, IDDCX_ENDPOINT_VERSION, IDDCX_FEATURE_IMPLEMENTATION,
    IDDCX_MONITOR, IDDCX_MONITOR_DESCRIPTION, IDDCX_MONITOR_DESCRIPTION_TYPE, IDDCX_MONITOR_INFO,
    IDDCX_SWAPCHAIN, IDDCX_TRANSMISSION_TYPE, IDDCX_XOR_CURSOR_SUPPORT, LUID, NTSTATUS, WDFDEVICE,
    WDFOBJECT, WDF_OBJECT_ATTRIBUTES,
};
use windows::{
    core::{s, w, GUID},
    Win32::{Foundation::TRUE, System::Threading::CreateEventA},
};

use crate::{
    direct_3d_device::Direct3DDevice,
    edid::Edid,
    ipc::{startup, MONITOR_MODES},
    swap_chain_processor::SwapChainProcessor,
};

// Maximum amount of monitors that can be connected
pub const MAX_MONITORS: u8 = 16;

pub struct DeviceContext {
    device: WDFDEVICE,
    adapter: Option<IDDCX_ADAPTER>,
}

// SAFETY: Raw ptr is managed by external library
unsafe impl Send for DeviceContext {}
unsafe impl Sync for DeviceContext {}

// for now, `device` is hardcoded into the macro, so it needs to be there even if unused
#[allow(unused)]
pub struct MonitorContext {
    device: IDDCX_MONITOR,
    swap_chain_processor: Option<SwapChainProcessor>,
}

// SAFETY: Raw ptr is managed by external library
unsafe impl Send for MonitorContext {}
unsafe impl Sync for MonitorContext {}

WDF_DECLARE_CONTEXT_TYPE!(pub DeviceContext);
WDF_DECLARE_CONTEXT_TYPE!(pub MonitorContext);

#[derive(Debug, thiserror::Error)]
pub enum ContextError {
    #[error("Failed to parse integer: {0:?}")]
    ParseInt(#[from] ParseIntError),
    #[error("Failed to convert integer: {0:?}")]
    TryFromInt(#[from] TryFromIntError),
    #[error("Failed to convert to NTSTATUS: {0:?}")]
    Ntstatus(#[from] NTSTATUS),
    #[error("Failed to convert to IddCxError: {0:?}")]
    IddCx(#[from] IddCxError),
    #[error("Failed to convert to WdfError: {0:?}")]
    Wdf(#[from] WdfError),
    #[error("Windows Error: {0:?}")]
    Win(#[from] windows::core::Error),
    #[error("{0:?}")]
    Other(#[from] anyhow::Error),
}

impl DeviceContext {
    pub fn new(device: WDFDEVICE) -> Self {
        Self {
            device,
            adapter: None,
        }
    }

    pub fn init_adapter(&mut self) -> Result<(), ContextError> {
        let mut version = IDDCX_ENDPOINT_VERSION {
            #[allow(clippy::cast_possible_truncation)]
            Size: size_of::<IDDCX_ENDPOINT_VERSION>() as u32,

            MajorVer: env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>()?,
            MinorVer: env!("CARGO_PKG_VERSION_MINOR").parse::<u32>()?,
            Build: env!("CARGO_PKG_VERSION_PATCH").parse::<u32>()?,
            ..Default::default()
        };

        let mut adapter_caps = IDDCX_ADAPTER_CAPS {
            #[allow(clippy::cast_possible_truncation)]
            Size: size_of::<IDDCX_ADAPTER_CAPS>() as u32,

            MaxMonitorsSupported: u32::from(MAX_MONITORS),

            EndPointDiagnostics: IDDCX_ENDPOINT_DIAGNOSTIC_INFO {
                #[allow(clippy::cast_possible_truncation)]
                Size: size_of::<IDDCX_ENDPOINT_DIAGNOSTIC_INFO>() as u32,
                GammaSupport: IDDCX_FEATURE_IMPLEMENTATION::IDDCX_FEATURE_IMPLEMENTATION_NONE,
                TransmissionType: IDDCX_TRANSMISSION_TYPE::IDDCX_TRANSMISSION_TYPE_WIRED_OTHER,

                pEndPointFriendlyName: w!("Virtual Display Driver Adapter").as_ptr(),
                pEndPointManufacturerName: w!("Cherry").as_ptr(),
                pEndPointModelName: w!("Pro").as_ptr(),

                pFirmwareVersion: addr_of_mut!(version).cast(),
                pHardwareVersion: addr_of_mut!(version).cast(),
            },

            ..Default::default()
        };

        let mut attr = WDF_OBJECT_ATTRIBUTES::init_context_type(unsafe { Self::get_type_info() });

        let adapter_init = IDARG_IN_ADAPTER_INIT {
            // this is WdfDevice because that's what we set last
            WdfDevice: self.device,
            pCaps: addr_of_mut!(adapter_caps).cast(),
            ObjectAttributes: addr_of_mut!(attr).cast(),
        };

        let mut adapter_init_out = IDARG_OUT_ADAPTER_INIT::default();
        unsafe { IddCxAdapterInitAsync(&adapter_init, &mut adapter_init_out)? };

        self.adapter = Some(adapter_init_out.AdapterObject);

        unsafe { self.clone_into(adapter_init_out.AdapterObject as WDFOBJECT)? };

        Ok(())
    }

    pub fn finish_init() -> NTSTATUS {
        // start the socket listener to listen for messages from the client
        startup();

        NTSTATUS::STATUS_SUCCESS
    }

    pub fn create_monitor(&mut self, index: u32) -> Result<(), ContextError> {
        let mut attr =
            WDF_OBJECT_ATTRIBUTES::init_context_type(unsafe { MonitorContext::get_type_info() });

        // use the edid serial number to represent the monitor index for later identification
        let mut edid = Edid::generate_with(index);

        let mut monitor_info = IDDCX_MONITOR_INFO {
            #[allow(clippy::cast_possible_truncation)]
            Size: size_of::<IDDCX_MONITOR_INFO>() as u32,
            // SAFETY: windows-rs + generated _GUID types are same size, with same fields, and repr C
            // see: https://microsoft.github.io/windows-docs-rs/doc/windows/core/struct.GUID.html
            // and: wmdf_umdf_sys::_GUID
            MonitorContainerId: unsafe { mem::transmute(GUID::new()?) },
            MonitorType:
                DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI,

            ConnectorIndex: index,
            MonitorDescription: IDDCX_MONITOR_DESCRIPTION {
                #[allow(clippy::cast_possible_truncation)]
                Size: size_of::<IDDCX_MONITOR_DESCRIPTION>() as u32,

                Type: IDDCX_MONITOR_DESCRIPTION_TYPE::IDDCX_MONITOR_DESCRIPTION_TYPE_EDID,

                #[allow(clippy::cast_possible_truncation)]
                DataSize: edid.len() as u32,

                pData: edid.as_mut_ptr().cast(),
            },
        };

        let monitor_create = IDARG_IN_MONITORCREATE {
            ObjectAttributes: &mut attr,
            pMonitorInfo: &mut monitor_info,
        };

        let mut monitor_create_out = IDARG_OUT_MONITORCREATE::default();
        unsafe {
            IddCxMonitorCreate(
                self.adapter.ok_or(anyhow!("Failed to get adapter"))?,
                &monitor_create,
                &mut monitor_create_out,
            )?
        };

        // store monitor object for later
        {
            let mut lock = MONITOR_MODES
                .get()
                .ok_or(anyhow!("Failed to get OnceLock"))?
                .lock()
                .map_err(|_| anyhow!("Failed to lock mutex"))?;

            for monitor in &mut *lock {
                if monitor.monitor.id == index {
                    monitor.monitor_object = Some(
                        NonNull::new(monitor_create_out.MonitorObject)
                            .ok_or(anyhow!("MonitorObject was null"))?,
                    );
                }
            }
        }

        unsafe {
            let context = MonitorContext::new(monitor_create_out.MonitorObject);
            context.init(monitor_create_out.MonitorObject as WDFOBJECT)?;
        }

        // tell os monitor is plugged in

        let mut arrival_out = IDARG_OUT_MONITORARRIVAL::default();

        unsafe {
            IddCxMonitorArrival(monitor_create_out.MonitorObject, &mut arrival_out)?;
        }

        Ok(())
    }
}

impl MonitorContext {
    pub fn new(device: IDDCX_MONITOR) -> Self {
        Self {
            device,
            swap_chain_processor: None,
        }
    }

    pub fn assign_swap_chain(
        &mut self,
        swap_chain: IDDCX_SWAPCHAIN,
        render_adapter: LUID,
        new_frame_event: HANDLE,
    ) {
        // drop processing thread
        drop(self.swap_chain_processor.take());

        // transmute would work, but one less unsafe block, so why not
        let luid = windows::Win32::Foundation::LUID {
            LowPart: render_adapter.LowPart,
            HighPart: render_adapter.HighPart,
        };

        let device = Direct3DDevice::init(luid);

        if let Ok(device) = device {
            let mut processor = SwapChainProcessor::new();

            processor.run(swap_chain, device, new_frame_event);

            self.swap_chain_processor = Some(processor);

            self.setup_hw_cursor();
        } else {
            // It's important to delete the swap-chain if D3D initialization fails, so that the OS knows to generate a new
            // swap-chain and try again.

            unsafe {
                let _ = WdfObjectDelete(swap_chain.cast());
            }
        }
    }

    pub fn unassign_swap_chain(&mut self) {
        self.swap_chain_processor.take();
    }

    pub fn setup_hw_cursor(&mut self) {
        let mouse_event = unsafe { CreateEventA(None, false, false, s!("vdd_mouse_event")) };
        let Ok(mouse_event) = mouse_event else {
            error!("CreateEventA failed: {mouse_event:?}");
            return;
        };

        // setup hardware cursor
        let cursor_info = IDDCX_CURSOR_CAPS {
            #[allow(clippy::cast_possible_truncation)]
            Size: std::mem::size_of::<IDDCX_CURSOR_CAPS>() as u32,
            AlphaCursorSupport: TRUE.0,
            MaxX: 512,
            MaxY: 512,
            ColorXorCursorSupport: IDDCX_XOR_CURSOR_SUPPORT::IDDCX_XOR_CURSOR_SUPPORT_NONE,
        };

        let hw_cursor = IDARG_IN_SETUP_HWCURSOR {
            CursorInfo: cursor_info,
            hNewCursorDataAvailable: mouse_event.0 as *mut c_void,
        };

        let res = unsafe { IddCxMonitorSetupHardwareCursor(self.device, &hw_cursor) };
        let Ok(res) = res else {
            error!("IddCxMonitorSetupHardwareCursor() failed: {res:?}");
            return;
        };

        if res.is_warning() {
            warn!("IddCxMonitorSetupHardwareCursor() warn: {res:?}");
        }
        if res.is_error() {
            error!("IddCxMonitorSetupHardwareCursor() failed: {res:?}");
        }
    }
}
