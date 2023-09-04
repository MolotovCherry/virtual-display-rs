use std::sync::mpsc::sync_channel;
use std::thread::JoinHandle;

use wdf_umdf::WDF_DECLARE_CONTEXT_TYPE;

pub struct SwapChainProcessor {
    thread: JoinHandle<()>,
}

impl SwapChainProcessor {}

impl Drop for SwapChainProcessor {
    fn drop(&mut self) {
        todo!()
    }
}
