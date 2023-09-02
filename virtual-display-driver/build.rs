fn main() {
    winres::WindowsResource::new().compile().unwrap();

    // need linked c runtime for umdf includes
    println!("cargo:rustc-link-lib=static=ucrt");
    println!("cargo:rustc-link-lib=static=vcruntime");
}
