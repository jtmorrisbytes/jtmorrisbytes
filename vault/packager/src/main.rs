// bundle the service and its dependencies to prepare it for installation or distrobution
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "This command will compile the local source code and prepare it for installation or distrobution"
    );
    let metadata = cargo_metadata::MetadataCommand::new().exec()?;
    // the name of the package 'this software'
    const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
    let package_metadata = metadata
        .packages
        .iter()
        .find(|p| p.name == PACKAGE_NAME)
        .ok_or("Expected to find the package in the current directory tree")?;
    // package_metadata.manifest_path;

    // std::process::Command::new("rustup target add x86_64-pc-windows-msvc")
    #[cfg(target_os = "windows")]
    {
        println!("running tests");
        let s = std::process::Command::new("cargo")
            .args(&[
                "test",
                "--target",
                "x86_64-pc-windows-msvc",
                "--target",
                "i686-pc-windows-msvc",
                "--release",
                "--manifest-path",
                package_metadata.manifest_path.as_str(),
                "-p",
                package_metadata.name.as_str(),
            ])
            .status()?
            .success();
        if !s {
            return Err(format!("Test failed").into());
        }
        std::process::Command::new("cargo")
            .args(&[
                "build",
                "--target",
                "x86_64-pc-windows-msvc",
                "--target",
                "i686-pc-windows-msvc",
                "--release",
                "--manifest-path",
                package_metadata.manifest_path.as_str(),
            ])
            .status()
            .ok();
    }

    let artifact_dir = metadata.workspace_root.join("out");
    std::fs::remove_dir_all(&artifact_dir).ok();
    std::fs::create_dir_all(&artifact_dir).ok();
    // build the ui wasm for the cranelift compiler and wasmer
    let s = std::process::Command::new("cargo")
        .args(&[
            "build",
            "--target",
            "wasm32-wasip1",
            "--target",
            "wasm32-unknown-unknown",
            "-p",
            "vault_wasm_ui",
        ])
        .status()?;
    if !s.success() {
        return Err("Failed to compile webassembly".into());
    }
    // optimize the web assembly before passing it to the cranelift compiler
    let mut optimization_options = wasm_opt::OptimizationOptions::new_opt_level_4();
    optimization_options.run(
        metadata
            .target_directory
            .join("wasm32-wasip1")
            .join("release")
            .join("libvault_wasm_ui.wasm"),
        artifact_dir.join("libvault_wasm_ui.wasm"),
    )?;

        optimization_options.run(
        metadata
            .target_directory
            .join("wasm32_unknown_unknown")
            .join("release")
            .join("libvault_wasm_ui.wasm"),
        artifact_dir.join("libvault_wasm_ui_wasm32.wasm"),
    )?;


    Ok(())
}
