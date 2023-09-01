use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "cargo:rustc-link-search=C:/Program Files (x86)/Windows Kits/10/Lib/wdf/umdf/x64/2.33"
    );

    // need linked c runtime for umdf includes
    println!("cargo:rustc-link-lib=static=ucrt");
    println!("cargo:rustc-link-lib=static=vcruntime");

    // need to link to umdf library too
    println!("cargo:rustc-link-lib=static=WdfDriverStubUm");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .detect_include_paths(true)
        // add umdf include path
        .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/wdf/umdf/2.33")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_default(true)
        // just some type that generated improperly
        .blocklist_type("_IMAGE_TLS_DIRECTORY64")
        .blocklist_type("IMAGE_TLS_DIRECTORY64")
        .blocklist_type("IMAGE_TLS_DIRECTORY")
        .blocklist_type("PIMAGE_TLS_DIRECTORY64")
        .blocklist_type("PIMAGE_TLS_DIRECTORY")
        // Finish the builder and generate the bindings.
        .generate()?;

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("umdf.rs"))?;

    Ok(())
}
