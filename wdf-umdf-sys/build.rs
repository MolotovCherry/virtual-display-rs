use std::path::PathBuf;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("cannot find the directory")]
    DirectoryNotFound,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    generate();
}

/// Retrieves the path to the Windows Kits directory. The default should be
/// `C:\Program Files (x86)\Windows Kits\10`.
pub fn get_windows_kits_dir() -> Result<PathBuf, Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";
    let dir: String = hklm.open_subkey(key)?.get_value("KitsRoot10")?;

    Ok(dir.into())
}

#[derive(Clone, Copy)]
pub enum DirectoryType {
    Include,
    Library,
}

/// Retrieves the path to the user mode libraries. The path may look something like:
/// `C:\Program Files (x86)\Windows Kits\10\lib\10.0.18362.0\um`.
pub fn get_um_dir(dir_type: DirectoryType) -> Result<PathBuf, Error> {
    // We first append lib to the path and read the directory..
    let dir = get_windows_kits_dir()?
        .join(match dir_type {
            DirectoryType::Include => "Include",
            DirectoryType::Library => "Lib",
        })
        .read_dir()?;

    // In the lib directory we may have one or more directories named after the version of Windows,
    // we will be looking for the highest version number.
    let dir = dir
        .filter_map(Result::ok)
        .map(|dir| dir.path())
        .filter(|dir| {
            dir.components()
                .last()
                .and_then(|c| c.as_os_str().to_str())
                .map_or(false, |c| c.starts_with("10.") && dir.join("um").is_dir())
        })
        .max()
        .ok_or_else(|| Error::DirectoryNotFound)?;

    // Finally append um to the path to get the path to the user mode libraries.
    Ok(dir.join("um"))
}

pub fn get_umdf_dir(dir_type: DirectoryType) -> Result<PathBuf, Error> {
    Ok(get_windows_kits_dir()?.join(match dir_type {
        DirectoryType::Include => PathBuf::from_iter(["Include", "wdf", "umdf", "2.33"]),
        DirectoryType::Library => PathBuf::from_iter(["Lib", "wdf", "umdf", "x64", "2.33"]),
    }))
}

fn build_dir() -> PathBuf {
    PathBuf::from(
        std::env::var_os("OUT_DIR").expect("the environment variable OUT_DIR is undefined"),
    )
}

fn generate() {
    // Tell Cargo to re-run this if src/wrapper.h gets changed.
    println!("cargo:rerun-if-changed=c/wrapper.h");

    // Find the include directory containing the user headers.
    let include_dir = get_um_dir(DirectoryType::Include).unwrap();
    let wdf_include_dir = get_umdf_dir(DirectoryType::Include).unwrap();

    let umdf_lib_dir = get_umdf_dir(DirectoryType::Library).unwrap();

    println!("cargo:rustc-link-search={}", umdf_lib_dir.display());

    // need to link to umdf library too
    println!("cargo:rustc-link-lib=static=WdfDriverStubUm");

    // Get the build directory.
    let out_path = build_dir();

    // Generate the bindings
    let umdf = bindgen::Builder::default()
        .derive_debug(false)
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .merge_extern_blocks(true)
        .header("c/wrapper.h")
        // important since we're using the stub
        .clang_arg("-DWDF_STUB")
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg(format!("-I{}", wdf_include_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        // generate
        .generate()
        .unwrap();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    umdf.write_to_file(out_path.join("umdf.rs")).unwrap();
}
