pub mod sub_module;

use flutter_rust_bridge::frb;
pub use sub_module::*;

pub struct TestRustApi {}

impl TestRustApi {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn test(&self) {
        println!("TestRustApi.test");
    }
}
