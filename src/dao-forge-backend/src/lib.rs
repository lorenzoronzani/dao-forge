use candid::{encode_args, CandidType, Principal};
use ic_cdk::api::management_canister::main::{
    create_canister, install_code, CanisterInstallMode, CanisterSettings, CreateCanisterArgument,
    InstallCodeArgument,
};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
async fn deploy_my_canister() -> Result<Principal, String> {
    let wasm =
        include_bytes!("../../../target/wasm32-unknown-unknown/release/dao_association.wasm")
            .to_vec();
    deploy_canister(wasm).await
}

#[ic_cdk::update]
async fn deploy_canister(wasm_module: Vec<u8>) -> Result<Principal, String> {
    // First create an empty canister
    let settings = CanisterSettings {
        controllers: Some(vec![ic_cdk::api::id()]), // Set the current canister as controller
        compute_allocation: None,
        memory_allocation: None,
        freezing_threshold: None,
        reserved_cycles_limit: None,
        log_visibility: None,
        wasm_memory_limit: None,
    };

    let create_result = create_canister(
        CreateCanisterArgument {
            settings: Some(settings),
        },
        1_000_000_000_000,
    )
    .await;

    let canister_id = match create_result {
        Ok((canister_id,)) => canister_id.canister_id,
        Err(e) => return Err(format!("Failed to create canister: {:?}", e)),
    };

    #[derive(CandidType)]
    struct TempInitArgs {
        name: String,
        members: Vec<String>,
    }

    let init_args = TempInitArgs {
        name: "My DAO".to_string(),
        members: vec!["member1".to_string(), "member2".to_string()],
    };

    // Encode the arguments using candid
    let encoded_args = encode_args((init_args,)).unwrap();

    // Install the WASM module
    let install_result = install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: wasm_module.clone(),
        arg: encoded_args,
    })
    .await;

    match install_result {
        Ok(()) => Ok(canister_id),
        Err(e) => Err(format!("Failed to install WASM: {:?}", e)),
    }
}
