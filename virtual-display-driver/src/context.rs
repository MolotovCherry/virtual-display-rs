use std::{
    mem::{self, size_of},
    ptr::{addr_of_mut, NonNull},
};

use wdf_umdf::{
    IddCxAdapterInitAsync, IddCxMonitorArrival, IddCxMonitorCreate, IntoHelper, WdfObjectDelete,
    WDF_DECLARE_CONTEXT_TYPE,
};
use wdf_umdf_sys::{
    DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY, HANDLE, IDARG_IN_ADAPTER_INIT, IDARG_IN_MONITORCREATE,
    IDARG_OUT_ADAPTER_INIT, IDARG_OUT_MONITORARRIVAL, IDARG_OUT_MONITORCREATE, IDDCX_ADAPTER,
    IDDCX_ADAPTER_CAPS, IDDCX_ENDPOINT_DIAGNOSTIC_INFO, IDDCX_ENDPOINT_VERSION,
    IDDCX_FEATURE_IMPLEMENTATION, IDDCX_MONITOR, IDDCX_MONITOR_DESCRIPTION,
    IDDCX_MONITOR_DESCRIPTION_TYPE, IDDCX_MONITOR_INFO, IDDCX_SWAPCHAIN, IDDCX_TRANSMISSION_TYPE,
    LUID, NTSTATUS, WDFDEVICE, WDFOBJECT, WDF_OBJECT_ATTRIBUTES,
};
use widestring::u16cstr;
use windows::core::GUID;

use crate::{
    direct_3d_device::Direct3DDevice,
    edid::generate_edid_with,
    ipc::{startup, MONITOR_MODES},
    swap_chain_processor::SwapChainProcessor,
};

// Maximum amount of monitors that can be connected
pub const MAX_MONITORS: u8 = 10;

#[allow(clippy::module_name_repetitions)]
pub struct DeviceContext {
    device: WDFDEVICE,
    adapter: Option<IDDCX_ADAPTER>,
}

// SAFETY: Raw ptr is managed by external library
unsafe impl Send for DeviceContext {}
unsafe impl Sync for DeviceContext {}

// for now, `device` is hardcoded into the macro, so it needs to be there even if unused
#[allow(unused)]
#[allow(clippy::module_name_repetitions)]
pub struct MonitorContext {
    device: IDDCX_MONITOR,
    swap_chain_processor: Option<SwapChainProcessor>,
}

// SAFETY: Raw ptr is managed by external library
unsafe impl Send for MonitorContext {}
unsafe impl Sync for MonitorContext {}

WDF_DECLARE_CONTEXT_TYPE!(pub DeviceContext);
WDF_DECLARE_CONTEXT_TYPE!(pub MonitorContext);

impl DeviceContext {
    pub fn new(device: WDFDEVICE) -> Self {
        Self {
            device,
            adapter: None,
        }
    }

    pub fn init_adapter(&mut self) -> NTSTATUS {
        let mut version = IDDCX_ENDPOINT_VERSION {
            Size: u32::try_from(size_of::<IDDCX_ENDPOINT_VERSION>()).unwrap(),
            MajorVer: env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap(),
            MinorVer: concat!(
                env!("CARGO_PKG_VERSION_MINOR"),
                env!("CARGO_PKG_VERSION_PATCH")
            )
            .parse::<u32>()
            .unwrap(),
            ..Default::default()
        };

        let mut adapter_caps = IDDCX_ADAPTER_CAPS {
            Size: u32::try_from(size_of::<IDDCX_ADAPTER_CAPS>()).unwrap(),
            MaxMonitorsSupported: u32::from(MAX_MONITORS),

            EndPointDiagnostics: IDDCX_ENDPOINT_DIAGNOSTIC_INFO {
                Size: u32::try_from(size_of::<IDDCX_ENDPOINT_DIAGNOSTIC_INFO>()).unwrap(),
                GammaSupport: IDDCX_FEATURE_IMPLEMENTATION::IDDCX_FEATURE_IMPLEMENTATION_NONE,
                TransmissionType: IDDCX_TRANSMISSION_TYPE::IDDCX_TRANSMISSION_TYPE_WIRED_OTHER,

                pEndPointFriendlyName: u16cstr!("Virtual Display").as_ptr(),
                pEndPointManufacturerName: u16cstr!("Cherry Tech").as_ptr(),
                pEndPointModelName: u16cstr!("VirtuDisplay Pro").as_ptr(),

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
        let mut status =
            unsafe { IddCxAdapterInitAsync(&adapter_init, &mut adapter_init_out) }.into_status();

        if status.is_success() {
            self.adapter = Some(adapter_init_out.AdapterObject);

            status = unsafe { self.clone_into(adapter_init_out.AdapterObject as WDFOBJECT) }
                .into_status();
        }

        status
    }

    pub fn finish_init() -> NTSTATUS {
        // start the socket listener to listen for messages from the client
        startup();

        NTSTATUS::STATUS_SUCCESS
    }

    pub fn create_monitor(&mut self, index: u32) -> NTSTATUS {
        let mut attr =
            WDF_OBJECT_ATTRIBUTES::init_context_type(unsafe { MonitorContext::get_type_info() });

        // use the edid serial number to represent the monitor index for later identification
        let mut edid = generate_edid_with(index);

        let mut monitor_info = IDDCX_MONITOR_INFO {
            Size: u32::try_from(size_of::<IDDCX_MONITOR_INFO>()).unwrap(),
            // SAFETY: windows-rs + generated _GUID types are same size, with same fields, and repr C
            // see: https://microsoft.github.io/windows-docs-rs/doc/windows/core/struct.GUID.html
            // and: wmdf_umdf_sys::_GUID
            MonitorContainerId: unsafe { mem::transmute(GUID::new().unwrap()) },
            MonitorType:
                DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI,

            ConnectorIndex: index,
            MonitorDescription: IDDCX_MONITOR_DESCRIPTION {
                Size: u32::try_from(size_of::<IDDCX_MONITOR_DESCRIPTION>()).unwrap(),
                Type: IDDCX_MONITOR_DESCRIPTION_TYPE::IDDCX_MONITOR_DESCRIPTION_TYPE_EDID,
                DataSize: u32::try_from(edid.len()).unwrap(),
                pData: addr_of_mut!(edid).cast(),
            },
        };

        let monitor_create = IDARG_IN_MONITORCREATE {
            ObjectAttributes: &mut attr,
            pMonitorInfo: &mut monitor_info,
        };

        let mut monitor_create_out = IDARG_OUT_MONITORCREATE::default();
        let mut status = unsafe {
            IddCxMonitorCreate(
                self.adapter.unwrap(),
                &monitor_create,
                &mut monitor_create_out,
            )
        }
        .into_status();

        if status.is_success() {
            // store monitor object for later
            {
                let mut lock = MONITOR_MODES.get().unwrap().lock().unwrap();

                for monitor in &mut *lock {
                    if monitor.monitor.id == index {
                        monitor.monitor_object =
                            Some(NonNull::new(monitor_create_out.MonitorObject).unwrap());
                    }
                }
            }

            unsafe {
                let context = MonitorContext::new(monitor_create_out.MonitorObject);
                context
                    .init(monitor_create_out.MonitorObject as WDFOBJECT)
                    .into_status();
            }

            // tell os monitor is plugged in
            if status.is_success() {
                let mut arrival_out = IDARG_OUT_MONITORARRIVAL::default();

                status = unsafe {
                    IddCxMonitorArrival(monitor_create_out.MonitorObject, &mut arrival_out)
                        .into_status()
                };
            }
        }

        status
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
}
