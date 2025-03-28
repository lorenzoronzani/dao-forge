use candid::Principal;
use ic_cdk::api::management_canister::main::{
    canister_status, create_canister, install_code, update_settings, CanisterIdRecord,
    CanisterInstallMode, CanisterSettings, CanisterStatusResponse, CreateCanisterArgument,
    InstallCodeArgument, UpdateSettingsArgument, WasmModule,
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

    async fn get_canister_status(canister_id: Principal) -> Result<CanisterStatusResponse, String> {
        match canister_status(CanisterIdRecord { canister_id }).await {
            Ok((status,)) => Ok(status),
            Err(e) => Err(format!("Failed to get canister status: {:?}", e)),
        }
    }

    async fn update_settings(
        canister_id: Principal,
        settings: CanisterSettings,
    ) -> Result<(), String> {
        match update_settings(UpdateSettingsArgument {
            canister_id,
            settings,
        })
        .await
        {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Failed to update canister settings: {:?}", e)),
        }
    }

    pub async fn remove_controller(
        canister_id: Principal,
        controller: Principal,
    ) -> Result<(), String> {
        let status = CanisterManagementService::get_canister_status(canister_id).await?;
        let controllers = status.settings.controllers;

        let updated_controllers = controllers
            .into_iter()
            .filter(|c| *c != controller)
            .collect();

        let settings = CanisterSettings {
            controllers: Some(updated_controllers),
            compute_allocation: Some(status.settings.compute_allocation),
            memory_allocation: Some(status.settings.memory_allocation),
            freezing_threshold: Some(status.settings.freezing_threshold),
            reserved_cycles_limit: Some(status.settings.reserved_cycles_limit),
            log_visibility: Some(status.settings.log_visibility),
            wasm_memory_limit: Some(status.settings.wasm_memory_limit),
        };

        CanisterManagementService::update_settings(canister_id, settings).await
    }
}
