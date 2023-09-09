use std::mem;

use wdf_umdf::{IddCxAdapterInitAsync, IntoHelper, WDF_DECLARE_CONTEXT_TYPE};
use wdf_umdf_sys::{
    IDARG_IN_ADAPTER_INIT, IDARG_OUT_ADAPTER_INIT, IDDCX_ADAPTER, IDDCX_ADAPTER_CAPS,
    IDDCX_ENDPOINT_DIAGNOSTIC_INFO, IDDCX_ENDPOINT_VERSION, IDDCX_FEATURE_IMPLEMENTATION,
    IDDCX_MONITOR, IDDCX_TRANSMISSION_TYPE, NTSTATUS, WDFDEVICE, WDFOBJECT, WDF_OBJECT_ATTRIBUTES,
};
use widestring::u16cstr;

// Taken from
// https://github.com/ge9/IddSampleDriver/blob/fe98ccff703b5c1e578a0d627aeac2fa77ac58e2/IddSampleDriver/Driver.cpp#L403
static MONITOR_EDID: &[u8] = &[
    0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x31, 0xD8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x05, 0x16, 0x01, 0x03, 0x6D, 0x32, 0x1C, 0x78, 0xEA, 0x5E, 0xC0, 0xA4, 0x59, 0x4A, 0x98, 0x25,
    0x20, 0x50, 0x54, 0x00, 0x00, 0x00, 0xD1, 0xC0, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x3A, 0x80, 0x18, 0x71, 0x38, 0x2D, 0x40, 0x58, 0x2C,
    0x45, 0x00, 0xF4, 0x19, 0x11, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x4C, 0x69, 0x6E,
    0x75, 0x78, 0x20, 0x23, 0x30, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0xFD, 0x00, 0x3B,
    0x3D, 0x42, 0x44, 0x0F, 0x00, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0xFC,
    0x00, 0x4C, 0x69, 0x6E, 0x75, 0x78, 0x20, 0x46, 0x48, 0x44, 0x0A, 0x20, 0x20, 0x20, 0x00, 0x05,
];

pub struct DeviceContext {
    pub device: WDFDEVICE,
    adapter: Option<IDDCX_ADAPTER>,
    monitor: Option<IDDCX_MONITOR>,
}

WDF_DECLARE_CONTEXT_TYPE!(pub DeviceContext);

// SAFETY: Raw ptr is managed by external library
unsafe impl Sync for DeviceContext {}

impl DeviceContext {
    pub fn new(device: WDFDEVICE) -> Self {
        Self {
            device,
            adapter: None,
            monitor: None,
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
            MaxMonitorsSupported: 1,

            EndPointDiagnostics: IDDCX_ENDPOINT_DIAGNOSTIC_INFO {
                Size: mem::size_of::<IDDCX_ENDPOINT_DIAGNOSTIC_INFO>() as u32,
                GammaSupport: IDDCX_FEATURE_IMPLEMENTATION::IDDCX_FEATURE_IMPLEMENTATION_NONE,
                TransmissionType: IDDCX_TRANSMISSION_TYPE::IDDCX_TRANSMISSION_TYPE_WIRED_OTHER,

                pEndPointFriendlyName: u16cstr!("Virtual Display").as_ptr(),
                pEndPointManufacturerName: u16cstr!("Cherry Tech").as_ptr(),
                pEndPointModelName: u16cstr!("VirtuDisplay Pro").as_ptr(),

                pFirmwareVersion: &version as *const _ as *mut _,
                pHardwareVersion: &version as *const _ as *mut _,
                ..Default::default()
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

            status = unsafe {
                DeviceContext::init_from(
                    self.device as WDFOBJECT,
                    adapter_init_out.AdapterObject as WDFOBJECT,
                )
            }
            .into_status();
        }

        status
    }
}
