use windows::{
    core::Error,
    Win32::{
        Foundation::{HMODULE, LUID},
        Graphics::{
            Direct3D::D3D_DRIVER_TYPE_UNKNOWN,
            Direct3D11::{
                D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            },
            Dxgi::{CreateDXGIFactory2, IDXGIAdapter1, IDXGIFactory5},
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

pub struct Direct3DDevice {
    adapter_luid: LUID,
    dxgi_factory: IDXGIFactory5,
    adapter: IDXGIAdapter1,
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
}

impl Direct3DDevice {
    pub fn new(adapter_luid: LUID) -> Result<Self, Direct3DError> {
        let dxgi_factory = unsafe { CreateDXGIFactory2::<IDXGIFactory5>(0)? };

        let adapter = unsafe { dxgi_factory.EnumAdapterByLuid::<IDXGIAdapter1>(adapter_luid)? };

        let mut device = None;
        let mut device_context = None;

        unsafe {
            D3D11CreateDevice(
                &adapter,
                D3D_DRIVER_TYPE_UNKNOWN,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                None,
                0,
                Some(&mut device),
                None,
                Some(&mut device_context),
            )?;
        }

        let device = device.ok_or("ID3D11Device not found")?;
        let device_context = device_context.ok_or("ID3D11DeviceContext not found")?;

        Ok(Self {
            adapter_luid,
            dxgi_factory,
            adapter,
            device,
            device_context,
        })
    }
}
