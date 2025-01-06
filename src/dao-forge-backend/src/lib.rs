use candid::Principal;
use ic_cdk::api::management_canister::main::{CanisterSettings, create_canister, install_code, CreateCanisterArgument, InstallCodeArgument, CanisterInstallMode};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
async fn deploy_my_canister() -> Result<Principal, String> {
    let wasm = include_bytes!("../../../target/wasm32-unknown-unknown/release/dao_association.wasm").to_vec();
    deploy_canister(wasm).await
}

#[ic_cdk::update]
async fn deploy_canister(wasm_module: Vec<u8>) -> Result<Principal, String> {
    // First create an empty canister
    let settings = CanisterSettings {
        controllers: Some(vec![ic_cdk::api::id()]),  // Set the current canister as controller
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
        reserved_cycles_limit: None,
        log_visibility: None,
        wasm_memory_limit: None,
    };

    let create_result = create_canister(
        CreateCanisterArgument { settings: Some(settings) },
        1_000_000_000_000,
    ).await;

    let canister_id = match create_result {
        Ok((canister_id,)) => canister_id.canister_id,
        Err(e) => return Err(format!("Failed to create canister: {:?}", e)),
    };

    // Install the WASM module
    let install_result = install_code(
        InstallCodeArgument {
            mode: CanisterInstallMode::Install,
            canister_id,
            wasm_module: wasm_module.clone(),
            arg: vec![],
        },
    ).await;

    match install_result {
        Ok(()) => Ok(canister_id),
        Err(e) => Err(format!("Failed to install WASM: {:?}", e)),
    }
}

