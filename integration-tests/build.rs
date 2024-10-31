use std::{env, path::Path, process::Command};

// make sure these are in sync with the values in tests/upgrade.rs
const OLD_CONTRACT_VERSION: &str = "0.1.0";
const NEW_CONTRACT_VERSION: &str = "0.1.1";

// generates two versions of the gateway contract at the root of the crate under `_artefacts/`
fn main() {
    let out = env!("CARGO_MANIFEST_DIR");
    let dir_name = "_artefacts/";
    let out_dir = Path::new(&out).join(dir_name);

    let old_contract_path = out_dir.join("axelar_gateway_old.wasm");
    let new_contract_path = out_dir.join("axelar_gateway_new.wasm");
    let axelar_gatway_path = out_dir.join("axelar_gateway.wasm");

    let mut command_name = Command::new("stellar");
    let command = command_name.args([
        "contract",
        "build",
        "--package",
        "axelar-gateway",
        "--no-cache",
        "--out-dir",
        dir_name,
    ]);

    env::set_var("GATEWAY_CONTRACT_TEST_VERSION", OLD_CONTRACT_VERSION);
    let _status = command.status().unwrap();

    std::fs::rename(&axelar_gatway_path, old_contract_path).unwrap();

    env::set_var("GATEWAY_CONTRACT_TEST_VERSION", NEW_CONTRACT_VERSION);
    let _status = command.status().unwrap();

    std::fs::rename(&axelar_gatway_path, new_contract_path).unwrap();
}
