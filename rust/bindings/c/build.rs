use std::env;

use cbindgen::Language;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let builder = cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_parse_deps(true)
        .with_parse_include(&["driver-ipc"]);

    // c++
    builder
        .clone()
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings/cpp/bindings.h");

    // c
    builder
        .with_language(Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings/c/bindings.h");
}
