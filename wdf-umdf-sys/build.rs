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

/// Retrieves the path to the Windows Kits directory. The default should be
/// `C:\Program Files (x86)\Windows Kits\10`.
pub fn get_windows_kits_dir() -> Result<PathBuf, Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";
    let dir: String = hklm.open_subkey(key)?.get_value("KitsRoot10")?;

    Ok(dir.into())
}

#[derive(Clone, Copy, PartialEq)]
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
    let mut dir = dir
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

    dir.push("um");

    if DirectoryType::Library == dir_type {
        dir.push("x64");
    }

    // Finally append um to the path to get the path to the user mode libraries.
    Ok(dir)
}

pub fn get_umdf_dir(dir_type: DirectoryType) -> Result<PathBuf, Error> {
    Ok(get_windows_kits_dir()?.join(match dir_type {
        DirectoryType::Include => PathBuf::from_iter(["Include", "wdf", "umdf", "2.33"]),
        DirectoryType::Library => PathBuf::from_iter(["Lib", "wdf", "umdf", "x64", "2.33"]),
    }))
}

/// Retrieves the path to the shared headers. The path may look something like:
/// `C:\Program Files (x86)\Windows Kits\10\lib\10.0.18362.0\shared`.
pub fn get_shared_dir() -> Result<PathBuf, Error> {
    // We first append lib to the path and read the directory..
    let dir = get_windows_kits_dir()?.join("Include").read_dir()?;

    // In the lib directory we may have one or more directories named after the version of Windows,
    // we will be looking for the highest version number.
    let dir = dir
        .filter_map(Result::ok)
        .map(|dir| dir.path())
        .filter(|dir| {
            dir.components()
                .last()
                .and_then(|c| c.as_os_str().to_str())
                .map_or(false, |c| {
                    c.starts_with("10.") && dir.join("shared").is_dir()
                })
        })
        .max()
        .ok_or_else(|| Error::DirectoryNotFound)?;

    // Finally append shared to the path to get the path to the shared headers.
    Ok(dir.join("shared"))
}

fn build_dir() -> PathBuf {
    PathBuf::from(
        std::env::var_os("OUT_DIR").expect("the environment variable OUT_DIR is undefined"),
    )
}

fn generate() {
    // Find the include directory containing the user headers.
    let include_um_dir = get_um_dir(DirectoryType::Include).unwrap();
    let lib_um_dir = get_um_dir(DirectoryType::Library).unwrap();
    let shared = get_shared_dir().unwrap();

    println!("cargo:rustc-link-search={}", lib_um_dir.display());

    // Tell Cargo to re-run this if src/wrapper.h gets changed.
    println!("cargo:rerun-if-changed=c/wrapper.h");

    //
    // UMDF
    //

    let umdf_lib_dir = get_umdf_dir(DirectoryType::Library).unwrap();

    println!("cargo:rustc-link-search={}", umdf_lib_dir.display());

    let wdf_include_dir = get_umdf_dir(DirectoryType::Include).unwrap();

    // need to link to umdf lib
    println!("cargo:rustc-link-lib=static=WdfDriverStubUm");

    //
    // IDDCX
    //

    let mut iddcx_lib_dir = lib_um_dir.clone();
    iddcx_lib_dir.push("iddcx");
    iddcx_lib_dir.push("1.9");

    println!("cargo:rustc-link-search={}", iddcx_lib_dir.display());

    // need to link to iddcx lib
    println!("cargo:rustc-link-lib=static=IddCxStub");

    //
    // REST
    //

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
        // general um includes
        .clang_arg(format!("-I{}", include_um_dir.display()))
        // umdf includes
        .clang_arg(format!("-I{}", wdf_include_dir.display()))
        .clang_arg(format!("-I{}", shared.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        // we will use our own custom type
        .blocklist_item("NTSTATUS")
        .clang_arg("--language=c++")
        // generate
        .generate()
        .unwrap();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    umdf.write_to_file(out_path.join("umdf.rs")).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    generate();
}
