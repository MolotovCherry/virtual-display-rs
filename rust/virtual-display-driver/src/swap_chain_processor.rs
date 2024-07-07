use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use log::{debug, error};
use wdf_umdf::{
    IddCxSwapChainFinishedProcessingFrame, IddCxSwapChainReleaseAndAcquireBuffer,
    IddCxSwapChainSetDevice, WdfObjectDelete,
};
use wdf_umdf_sys::{
    HANDLE, IDARG_IN_SWAPCHAINSETDEVICE, IDARG_OUT_RELEASEANDACQUIREBUFFER, IDDCX_SWAPCHAIN,
    NTSTATUS, WAIT_TIMEOUT, WDFOBJECT,
};
use windows::{
    core::{w, Interface},
    Win32::{
        Foundation::HANDLE as WHANDLE,
        Graphics::Dxgi::IDXGIDevice,
        System::Threading::{
            AvRevertMmThreadCharacteristics, AvSetMmThreadCharacteristicsW, WaitForSingleObject,
        },
    },
};

use crate::{direct_3d_device::Direct3DDevice, helpers::Sendable};

pub struct SwapChainProcessor {
    terminate: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

unsafe impl Send for SwapChainProcessor {}
unsafe impl Sync for SwapChainProcessor {}

impl SwapChainProcessor {
    pub fn new() -> Self {
        Self {
            terminate: Arc::new(AtomicBool::new(false)),
            thread: None,
        }
    }

    pub fn run(
        &mut self,
        swap_chain: IDDCX_SWAPCHAIN,
        device: Direct3DDevice,
        available_buffer_event: HANDLE,
    ) {
        let available_buffer_event = unsafe { Sendable::new(available_buffer_event) };
        let swap_chain = unsafe { Sendable::new(swap_chain) };
        let terminate = self.terminate.clone();

        let join_handle = thread::spawn(move || {
            // It is very important to prioritize this thread by making use of the Multimedia Scheduler Service.
            // It will intelligently prioritize the thread for improved throughput in high CPU-load scenarios.
            let mut av_task = 0u32;
            let res = unsafe { AvSetMmThreadCharacteristicsW(w!("Distribution"), &mut av_task) };
            let Ok(av_handle) = res else {
                error!("Failed to prioritize thread: {res:?}");
                return;
            };

            Self::run_core(*swap_chain, &device, *available_buffer_event, &terminate);

            let res = unsafe { WdfObjectDelete(*swap_chain as WDFOBJECT) };
            if let Err(e) = res {
                error!("Failed to delete wdf object: {e:?}");
                return;
            }

            // Revert the thread to normal once it's done
            let res = unsafe { AvRevertMmThreadCharacteristics(av_handle) };
            if let Err(e) = res {
                error!("Failed to revert prioritize thread: {e:?}");
            }
        });

        self.thread = Some(join_handle);
    }

    fn run_core(
        swap_chain: IDDCX_SWAPCHAIN,
        device: &Direct3DDevice,
        available_buffer_event: HANDLE,
        terminate: &AtomicBool,
    ) {
        let dxgi_device = device.device.cast::<IDXGIDevice>();
        let Ok(dxgi_device) = dxgi_device else {
            error!("Failed to cast ID3D11Device to IDXGIDevice: {dxgi_device:?}");
            return;
        };

        let set_device = IDARG_IN_SWAPCHAINSETDEVICE {
            pDevice: dxgi_device.into_raw().cast(),
        };

        let res = unsafe { IddCxSwapChainSetDevice(swap_chain, &set_device) };
        if res.is_err() {
            debug!("Failed to set swapchain device: {res:?}");
            return;
        }

        loop {
            let mut buffer = IDARG_OUT_RELEASEANDACQUIREBUFFER::default();
            let hr: NTSTATUS =
                unsafe { IddCxSwapChainReleaseAndAcquireBuffer(swap_chain, &mut buffer).into() };

            #[allow(clippy::items_after_statements)]
            const E_PENDING: u32 = 0x8000_000A;
            if u32::from(hr) == E_PENDING {
                let wait_result =
                    unsafe { WaitForSingleObject(WHANDLE(available_buffer_event.cast()), 16).0 };

                // thread requested an end
                let should_terminate = terminate.load(Ordering::Relaxed);
                if should_terminate {
                    break;
                }

                // WAIT_OBJECT_0 | WAIT_TIMEOUT
                if matches!(wait_result, 0 | WAIT_TIMEOUT) {
                    // We have a new buffer, so try the AcquireBuffer again
                    continue;
                }

                // The wait was cancelled or something unexpected happened
                break;
            } else if hr.is_success() {
                // This is the most performance-critical section of code in an IddCx driver. It's important that whatever
                // is done with the acquired surface be finished as quickly as possible.
                let hr = unsafe { IddCxSwapChainFinishedProcessingFrame(swap_chain) };

                if hr.is_err() {
                    break;
                }
            } else {
                // The swap-chain was likely abandoned (e.g. DXGI_ERROR_ACCESS_LOST), so exit the processing loop
                break;
            }
        }
    }
}

impl Drop for SwapChainProcessor {
    fn drop(&mut self) {
        if let Some(handle) = self.thread.take() {
            // send signal to end thread
            self.terminate.store(true, Ordering::Relaxed);

            // wait until thread is finished
            _ = handle.join();
        }
    }
}
