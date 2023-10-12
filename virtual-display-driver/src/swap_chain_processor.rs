use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use log::error;
use wdf_umdf::{
    IddCxSwapChainFinishedProcessingFrame, IddCxSwapChainReleaseAndAcquireBuffer,
    IddCxSwapChainSetDevice, IntoHelper, WdfObjectDelete,
};
use wdf_umdf_sys::{
    HANDLE, IDARG_IN_SWAPCHAINSETDEVICE, IDARG_OUT_RELEASEANDACQUIREBUFFER, IDDCX_SWAPCHAIN,
    WAIT_TIMEOUT, WDFOBJECT,
};
use windows::{
    core::{ComInterface, Interface},
    Win32::{
        Foundation::HANDLE as WHANDLE, Graphics::Dxgi::IDXGIDevice,
        System::Threading::WaitForSingleObject,
    },
};

use crate::direct_3d_device::Direct3DDevice;

pub struct SwapChainProcessor {
    swap_chain: IDDCX_SWAPCHAIN,
    device: Direct3DDevice,
    available_buffer_event: HANDLE,
    terminate_event: AtomicBool,
    thread: Mutex<Option<JoinHandle<()>>>,
    dropped: AtomicBool,
}

unsafe impl Send for SwapChainProcessor {}
unsafe impl Sync for SwapChainProcessor {}

impl SwapChainProcessor {
    pub fn new(
        swap_chain: IDDCX_SWAPCHAIN,
        device: Direct3DDevice,
        new_frame_event: HANDLE,
    ) -> Arc<Self> {
        Arc::new(Self {
            swap_chain,
            device,
            available_buffer_event: new_frame_event,
            terminate_event: AtomicBool::new(false),
            thread: Mutex::new(None),
            dropped: AtomicBool::new(false),
        })
    }

    pub fn run(self: Arc<Self>) {
        struct Sendable<T>(T);
        unsafe impl<T> Send for Sendable<T> {}
        unsafe impl<T> Sync for Sendable<T> {}

        let swap_chain_ptr = Sendable(self.swap_chain);
        let thread_self = self.clone();

        let join_handle = thread::spawn(move || {
            // It is very important to prioritize this thread by making use of the Multimedia Scheduler Service.
            // It will intelligently prioritize the thread for improved throughput in high CPU-load scenarios.
            // let mut task_handle = 0u32;
            // let res = unsafe {
            //     AvSetMmThreadCharacteristicsW(w!("DisplayPostProcessing"), &mut task_handle)
            // };
            // if let Err(e) = res {
            //     error!("Failed to prioritize thread: {e}");
            //     return;
            // }

            thread_self.run_core();

            let swap_chain = swap_chain_ptr;
            unsafe {
                WdfObjectDelete(swap_chain.0 as WDFOBJECT).unwrap();
            }

            // Revert the thread to normal once it's done
            // let res = unsafe { AvRevertMmThreadCharacteristics(WHANDLE(task_handle as _)) };
            // if let Err(e) = res {
            //     error!("Failed to prioritize thread: {e}");
            // }
        });

        let mut handle = self.thread.lock().unwrap();
        *handle = Some(join_handle);
    }

    fn run_core(&self) {
        let dxgi_device = self.device.device.cast::<IDXGIDevice>();
        let Ok(dxgi_device) = dxgi_device else {
            error!(
                "Failed to cast ID3D11Device to IDXGIDevice: {}",
                dxgi_device.unwrap_err()
            );

            return;
        };

        let set_device = IDARG_IN_SWAPCHAINSETDEVICE {
            pDevice: dxgi_device.into_raw() as *mut _,
        };

        if let Err(e) = unsafe { IddCxSwapChainSetDevice(self.swap_chain, &set_device) } {
            error!("Failed to set up IddcxSwapChainDevice: {e}");
            return;
        }

        loop {
            let mut buffer = IDARG_OUT_RELEASEANDACQUIREBUFFER::default();
            let hr = unsafe { IddCxSwapChainReleaseAndAcquireBuffer(self.swap_chain, &mut buffer) }
                .into_status();

            const E_PENDING: u32 = 0x8000000A;
            if u32::from(hr) == E_PENDING {
                let wait_result =
                    unsafe { WaitForSingleObject(WHANDLE(self.available_buffer_event as _), 16).0 };

                // thread requested an end
                let terminate = self.terminate_event.load(Ordering::Relaxed);
                if terminate {
                    break;
                }

                // WAIT_OBJECT_0 | WAIT_TIMEOUT
                if matches!(wait_result, 0 | WAIT_TIMEOUT) {
                    // We have a new buffer, so try the AcquireBuffer again
                    continue;
                } else {
                    // The wait was cancelled or something unexpected happened
                    break;
                }
            } else if hr.is_success() {
                // This is the most performance-critical section of code in an IddCx driver. It's important that whatever
                // is done with the acquired surface be finished as quickly as possible.
                let hr = unsafe { IddCxSwapChainFinishedProcessingFrame(self.swap_chain) };

                if hr.is_err() {
                    break;
                }
            } else {
                // The swap-chain was likely abandoned (e.g. DXGI_ERROR_ACCESS_LOST), so exit the processing loop
                break;
            }
        }
    }

    /// Terminate swap chain if it hasn't already been
    pub fn terminate(&self) {
        let dropped = self.dropped.load(Ordering::Relaxed);
        if !dropped {
            self.dropped.store(true, Ordering::Relaxed);

            // send signal to end thread
            self.terminate_event.store(true, Ordering::Relaxed);

            // wait until thread is finished
            self.thread.lock().unwrap().take().unwrap().join().unwrap();
        }
    }
}

impl Drop for SwapChainProcessor {
    fn drop(&mut self) {
        self.terminate();
    }
}
