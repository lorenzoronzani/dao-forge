use candid::Principal;
use ic_cdk::api::management_canister::main::{
    canister_status, create_canister, install_code, CanisterIdRecord, CanisterInstallMode,
    CanisterSettings, CanisterStatusResponse, CreateCanisterArgument, InstallCodeArgument,
    WasmModule,
};

pub struct CanisterManagementService;

impl CanisterManagementService {
    pub async fn deploy_canister(
        wasm_module: WasmModule,
        encoded_init_args: Vec<u8>,
        controllers: Vec<Principal>,
    ) -> Result<Principal, String> {
        let settings = CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            log_visibility: None,
            wasm_memory_limit: None,
        };

        let canister_id =
            CanisterManagementService::create_canister(settings, 1_000_000_000_000).await?;

        CanisterManagementService::install_code(canister_id, wasm_module, encoded_init_args)
            .await?;

        Ok(canister_id)
    }

    async fn create_canister(
        settings: CanisterSettings,
        cycles: u128,
    ) -> Result<Principal, String> {
        match create_canister(
            CreateCanisterArgument {
                settings: Some(settings),
            },
            cycles,
        )
        .await
        {
            Ok((canister_id,)) => Ok(canister_id.canister_id),
            Err(e) => Err(format!("Failed to create canister: {:?}", e)),
        }
    }

    async fn install_code(
        canister_id: Principal,
        wasm_module: WasmModule,
        encoded_args: Vec<u8>,
    ) -> Result<(), String> {
        match install_code(InstallCodeArgument {
            mode: CanisterInstallMode::Install,
            canister_id,
            wasm_module,
            arg: encoded_args,
        })
        .await
        {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Failed to install WASM: {:?}", e)),
        }
    }

    pub async fn canister_status(canister_id: Principal) -> Result<CanisterStatusResponse, String> {
        match canister_status(CanisterIdRecord { canister_id }).await {
            Ok((status,)) => Ok(status),
            Err(e) => Err(format!("Failed to get canister status: {:?}", e)),
        }
    }
}

#[cfg(test)]
#[path = "canister_management_service_tests.rs"]
mod canister_management_service_tests;
