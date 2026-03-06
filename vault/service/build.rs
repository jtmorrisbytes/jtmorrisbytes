use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=\"build.rs\"");
    
    let metadata = cargo_metadata::MetadataCommand::new().exec()?;
    let package = metadata.packages.iter().find(|p| p.name == "vault_wasm_ui");
    if package.is_none() {
        println!("cargo:error=\"Failed to find package vault_wasm_ui in the current workspace\"");
        return Err("Failed to find package vault_wasm_ui in the current workspace".into());
    }
    // all bin targets in this crate are assumed to be webassembly binaries with a main function
    let package = package.unwrap();

    println!("cargo:rerun-if-changed={}",package.manifest_path);
    // 'collect' all the binaries into multiple 'bin' flags
    let bins: Vec<&str> = package
        .targets
        .iter()
        .filter(|p| p.kind.contains(&cargo_metadata::TargetKind::Bin))
        .map(|t| t.name.as_str())
        .collect();

    let mut command = std::process::Command::new("cargo");
    let mut c: &mut Command = &mut command;
    c = c.args(&["build","--target","wasm32-wasip1", "--release","-p",package.name.as_str()]);
    for bin in bins {
        c = c.args(&["--bin",bin]);
    }
    let status = c.status()?;
    if !status.success() {
        println!("cargo::error=\"Build Failed\"");
        return Err("Build Failed".into());
    }



    Ok(())
}
