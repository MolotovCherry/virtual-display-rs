use flutter_rust_bridge::frb;

pub struct AnotherTestRustApi {}

impl AnotherTestRustApi {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn test(&self) {
        println!("AnotherTestRustApi.test");
    }
}
