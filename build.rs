fn main() {
    if let Err(e) = link_lib() {
        eprintln!("cargo:error={e}");
        panic!();
    }
}

fn link_lib() -> Result<(), Box<dyn std::error::Error>> {
    let visa_path = std::env::var("VISA_DIR")?;
    let visa_path = std::path::PathBuf::from(visa_path);

    // Get arch
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_lib = if target_arch != "x86" && target_arch != "i686" {
        "visa64"
    } else {
        "visa32"
    };

    let search_dir = visa_path.join("Lib_x64").join("msc");

    // Check if the library exists
    if !search_dir.join(format!("{target_lib}.lib")).exists()
        && !visa_path.join(format!("{target_lib}.lib")).exists()
    {
        return Err(format!("The library {target_lib}.lib does not exist in the expected directories: {search_dir:?} or {visa_path:?}").into());
    }

    // link to the VISA library
    let search_dir = visa_path.join("Lib_x64").join("msc");
    println!("cargo:rustc-link-search=native={}", visa_path.display());
    println!("cargo:rustc-link-search=native={}", search_dir.display());
    println!("cargo:rustc-link-lib=static={target_lib}");

    Ok(())
}
