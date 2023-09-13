use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex, atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};

use wdf_umdf::{
    IddCxSwapChainFinishedProcessingFrame, IddCxSwapChainReleaseAndAcquireBuffer,
    IddCxSwapChainSetDevice, IntoHelper, WdfObjectDelete,
};
use wdf_umdf_sys::{
    HANDLE, IDARG_IN_SWAPCHAINSETDEVICE, IDARG_OUT_RELEASEANDACQUIREBUFFER, IDDCX_SWAPCHAIN,
    WAIT_TIMEOUT, WDFOBJECT,
};
use windows::{
    core::{w, ComInterface},
    Win32::{
        Foundation::HANDLE as WHANDLE,
        Graphics::Direct3D11::ID3D11Device,
        System::Threading::{
            AvRevertMmThreadCharacteristics, AvSetMmThreadCharacteristicsW, WaitForSingleObject,
        },
    },
};

use crate::direct_3d_device::Direct3DDevice;

pub struct SwapChainProcessor {
    swap_chain: IDDCX_SWAPCHAIN,
    device: Direct3DDevice,
    available_buffer_event: HANDLE,
    terminate_event: Mutex<Option<Sender<()>>>,
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
            terminate_event: Mutex::new(None),
            thread: Mutex::new(None),
            dropped: AtomicBool::new(false),
        })
    }

    pub fn run(self: Arc<Self>) {
        let (terminate_s, terminate_r) = channel();
        {
            let mut term = self.terminate_event.lock().unwrap();
            *term = Some(terminate_s);
        }

        struct Sendable<T>(T);
        unsafe impl<T> Send for Sendable<T> {}
        unsafe impl<T> Sync for Sendable<T> {}

        let swap_chain_ptr = Sendable(self.swap_chain);
        let thread_self = self.clone();
        let join_handle = thread::spawn(move || {
            // It is very important to prioritize this thread by making use of the Multimedia Scheduler Service.
            // It will intelligently prioritize the thread for improved throughput in high CPU-load scenarios.
            let task_handle = std::ptr::null_mut();
            unsafe {
                AvSetMmThreadCharacteristicsW(w!("DisplayPostProcessing"), task_handle).unwrap();
            }

            thread_self.run_core(terminate_r);

            let swap_chain = swap_chain_ptr;
            unsafe {
                WdfObjectDelete(swap_chain.0 as WDFOBJECT).unwrap();
            }

            // Revert the thread to normal once it's done
            unsafe {
                AvRevertMmThreadCharacteristics(WHANDLE(task_handle as _)).unwrap();
            }
        });

        let mut handle = self.thread.lock().unwrap();
        *handle = Some(join_handle);
    }

    fn run_core(&self, terminate_r: Receiver<()>) {
        let Ok(dxgi_device) = self.device.device.cast::<ID3D11Device>() else {
            return;
        };

        let set_device = IDARG_IN_SWAPCHAINSETDEVICE {
            pDevice: dxgi_device.as_unknown() as *const _ as *mut _,
        };

        if unsafe { IddCxSwapChainSetDevice(self.swap_chain, &set_device) }.is_err() {
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
                if terminate_r.try_recv().is_ok() {
                    break;
                }

                // WAIT_OBJECT_) | WAIT_TIMEOUT
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

    /// Let it drop OR
    pub fn terminate(&self) {
        let dropped = self.dropped.load(Ordering::Relaxed);
        if !dropped {
            // send signal to end thread
            self.terminate_event
                .lock()
                .unwrap()
                .take()
                .unwrap()
                .send(())
                .unwrap();

            // wait until thread is finished
            self.thread.lock().unwrap().take().unwrap().join().unwrap();

            self.dropped.store(true, Ordering::Relaxed);
        }
    }
}

impl Drop for SwapChainProcessor {
    fn drop(&mut self) {
        self.terminate();
    }
}
