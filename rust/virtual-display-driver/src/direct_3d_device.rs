use windows::{
    core::Error,
    Win32::{
        Foundation::LUID,
        Graphics::{
            Direct3D::D3D_DRIVER_TYPE_UNKNOWN,
            Direct3D11::{
                D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY,
                D3D11_CREATE_DEVICE_SINGLETHREADED, D3D11_SDK_VERSION,
            },
            Dxgi::{CreateDXGIFactory2, IDXGIAdapter1, IDXGIFactory5, DXGI_CREATE_FACTORY_FLAGS},
        },
    },
};

#[derive(thiserror::Error, Debug)]
pub enum Direct3DError {
    #[error("Direct3DError({0:?})")]
    Win32(#[from] Error),
    #[error("Direct3DError(\"{0}\")")]
    Other(&'static str),
}

impl From<&'static str> for Direct3DError {
    fn from(value: &'static str) -> Self {
        Direct3DError::Other(value)
    }
}

#[derive(Debug)]
pub struct Direct3DDevice {
    // The following are already refcounted, so they're safe to use directly without additional drop impls
    _dxgi_factory: IDXGIFactory5,
    _adapter: IDXGIAdapter1,
    pub device: ID3D11Device,
    pub ctx: ID3D11DeviceContext,
}

impl Direct3DDevice {
    pub fn init(adapter_luid: LUID) -> Result<Self, Direct3DError> {
        let dxgi_factory =
            unsafe { CreateDXGIFactory2::<IDXGIFactory5>(DXGI_CREATE_FACTORY_FLAGS(0))? };

        let adapter = unsafe { dxgi_factory.EnumAdapterByLuid::<IDXGIAdapter1>(adapter_luid)? };

        let mut device = None;
        let mut device_context = None;

        unsafe {
            D3D11CreateDevice(
                &adapter,
                D3D_DRIVER_TYPE_UNKNOWN,
                None,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT
                    | D3D11_CREATE_DEVICE_SINGLETHREADED
                    | D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY,
                None,
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut device_context),
            )?;
        }

        let device = device.ok_or("ID3D11Device not found")?;
        let device_context = device_context.ok_or("ID3D11DeviceContext not found")?;

        let slf = Self {
            _dxgi_factory: dxgi_factory,
            _adapter: adapter,
            device,
            ctx: device_context,
        };

        Ok(slf)
    }
}
