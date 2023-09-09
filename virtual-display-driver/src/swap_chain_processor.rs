use std::thread::JoinHandle;

pub struct SwapChainProcessor {
    thread: JoinHandle<()>,
}

impl SwapChainProcessor {}

impl Drop for SwapChainProcessor {
    fn drop(&mut self) {
        todo!()
    }
}
