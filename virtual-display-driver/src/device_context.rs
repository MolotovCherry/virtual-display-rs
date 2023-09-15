use std::{
    mem,
    ptr::NonNull,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use wdf_umdf::{
    IddCxAdapterInitAsync, IddCxMonitorArrival, IddCxMonitorCreate, IntoHelper, WdfObjectDelete,
    WDF_DECLARE_CONTEXT_TYPE,
};
use wdf_umdf_sys::{
    DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY, HANDLE, IDARG_IN_ADAPTER_INIT, IDARG_IN_MONITORCREATE,
    IDARG_OUT_ADAPTER_INIT, IDARG_OUT_MONITORARRIVAL, IDARG_OUT_MONITORCREATE, IDDCX_ADAPTER,
    IDDCX_ADAPTER_CAPS, IDDCX_ENDPOINT_DIAGNOSTIC_INFO, IDDCX_ENDPOINT_VERSION,
    IDDCX_FEATURE_IMPLEMENTATION, IDDCX_MONITOR_DESCRIPTION, IDDCX_MONITOR_DESCRIPTION_TYPE,
    IDDCX_MONITOR_INFO, IDDCX_SWAPCHAIN, IDDCX_TRANSMISSION_TYPE, LUID, NTSTATUS, WDFDEVICE,
    WDFOBJECT, WDF_OBJECT_ATTRIBUTES,
};
use widestring::u16cstr;
use windows::core::GUID;

use crate::{
    direct_3d_device::Direct3DDevice, edid::generate_edid_with, monitor_listener::MONITOR_MODES,
    swap_chain_processor::SwapChainProcessor,
};

pub static FINISHED_INIT: AtomicBool = AtomicBool::new(false);

// Maximum amount of monitors that can be connected
pub const MAX_MONITORS: u8 = 10;

pub struct DeviceContext {
    pub device: WDFDEVICE,
    adapter: Option<IDDCX_ADAPTER>,
    swap_chain_processor: Option<Arc<SwapChainProcessor>>,
}

WDF_DECLARE_CONTEXT_TYPE!(pub DeviceContext);

// SAFETY: Raw ptr is managed by external library
unsafe impl Send for DeviceContext {}
unsafe impl Sync for DeviceContext {}

impl DeviceContext {
    pub fn new(device: WDFDEVICE) -> Self {
        Self {
            device,
            adapter: None,
            swap_chain_processor: None,
        }
    }

    pub fn init_adapter(&mut self) -> NTSTATUS {
        let version = IDDCX_ENDPOINT_VERSION {
            Size: mem::size_of::<IDDCX_ENDPOINT_VERSION>() as u32,
            MajorVer: env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap(),
            MinorVer: concat!(
                env!("CARGO_PKG_VERSION_MINOR"),
                env!("CARGO_PKG_VERSION_PATCH")
            )
            .parse::<u32>()
            .unwrap(),
            ..Default::default()
        };

        let adapter_caps = IDDCX_ADAPTER_CAPS {
            Size: mem::size_of::<IDDCX_ADAPTER_CAPS>() as u32,
            MaxMonitorsSupported: MAX_MONITORS as u32,

            EndPointDiagnostics: IDDCX_ENDPOINT_DIAGNOSTIC_INFO {
                Size: mem::size_of::<IDDCX_ENDPOINT_DIAGNOSTIC_INFO>() as u32,
                GammaSupport: IDDCX_FEATURE_IMPLEMENTATION::IDDCX_FEATURE_IMPLEMENTATION_NONE,
                TransmissionType: IDDCX_TRANSMISSION_TYPE::IDDCX_TRANSMISSION_TYPE_WIRED_OTHER,

                pEndPointFriendlyName: u16cstr!("Virtual Display").as_ptr(),
                pEndPointManufacturerName: u16cstr!("Cherry Tech").as_ptr(),
                pEndPointModelName: u16cstr!("VirtuDisplay Pro").as_ptr(),

                pFirmwareVersion: &version as *const _ as *mut _,
                pHardwareVersion: &version as *const _ as *mut _,
            },

            ..Default::default()
        };

        let attr = WDF_OBJECT_ATTRIBUTES::init_context_type(unsafe { Self::get_type_info() });

        let adapter_init = IDARG_IN_ADAPTER_INIT {
            // this is WdfDevice because that's what we set last
            WdfDevice: self.device,
            pCaps: &adapter_caps as *const _ as *mut _,
            ObjectAttributes: &attr as *const _ as *mut _,
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

    pub fn finish_init(&mut self) -> NTSTATUS {
        FINISHED_INIT.store(true, Ordering::Release);

        NTSTATUS::STATUS_SUCCESS
    }

    pub fn create_monitor(&mut self, index: u32) -> NTSTATUS {
        let mut attr =
            WDF_OBJECT_ATTRIBUTES::init_context_type(unsafe { DeviceContext::get_type_info() });

        // use the edid serial number to represent the monitor index for later identification
        let edid = generate_edid_with(index);

        let mut monitor_info = IDDCX_MONITOR_INFO {
            Size: mem::size_of::<IDDCX_MONITOR_INFO>() as u32,
            // SAFETY: windows-rs + generated _GUID types are same size, with same fields, and repr C
            // see: https://microsoft.github.io/windows-docs-rs/doc/windows/core/struct.GUID.html
            // and: wmdf_umdf_sys::_GUID
            MonitorContainerId: unsafe { mem::transmute(GUID::new().unwrap()) },
            MonitorType:
                DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI,

            ConnectorIndex: index,
            MonitorDescription: IDDCX_MONITOR_DESCRIPTION {
                Size: mem::size_of::<IDDCX_MONITOR_DESCRIPTION>() as u32,
                Type: IDDCX_MONITOR_DESCRIPTION_TYPE::IDDCX_MONITOR_DESCRIPTION_TYPE_EDID,
                DataSize: edid.len() as u32,
                pData: edid.as_ptr() as *const _ as *mut _,
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
                let Some(lock) = MONITOR_MODES.get() else {
                    // not initted yet
                    panic!("MONITOR_MODES not initialized");
                };

                let Ok(mut lock) = lock.lock() else {
                    panic!("Failed to lock MONITOR_MODES");
                };

                for monitor in &mut *lock {
                    if monitor.monitor.id == index {
                        monitor.monitor_object =
                            Some(NonNull::new(monitor_create_out.MonitorObject).unwrap());
                    }
                }
            }

            unsafe {
                status = self
                    .clone_into(monitor_create_out.MonitorObject as *mut _)
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

    pub fn assign_swap_chain(
        &mut self,
        swap_chain: IDDCX_SWAPCHAIN,
        render_adapter: LUID,
        new_frame_event: HANDLE,
    ) {
        // drop processing thread
        if let Some(processor) = self.swap_chain_processor.take() {
            processor.terminate();
        }

        // Safety: wmd_umdf_sys::_LUID and windows::LUID both are repr c and both with the same layouts
        let device = Direct3DDevice::init(unsafe { std::mem::transmute(render_adapter) });
        if let Ok(device) = device {
            let processor = SwapChainProcessor::new(swap_chain, device, new_frame_event);

            processor.clone().run();

            self.swap_chain_processor = Some(processor);
        } else {
            // It's important to delete the swap-chain if D3D initialization fails, so that the OS knows to generate a new
            // swap-chain and try again.

            unsafe {
                let _ = WdfObjectDelete(swap_chain as *mut _);
            }
        }
    }

    pub fn unassign_swap_chain(&mut self) {
        if let Some(processor) = self.swap_chain_processor.take() {
            processor.terminate();
        }
    }
}
