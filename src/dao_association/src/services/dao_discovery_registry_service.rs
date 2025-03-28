use crate::services::InterCanisterService;
use candid::Principal;

pub struct DaoDiscoveryRegistryService {
    pub canister_id: Principal,
}

impl DaoDiscoveryRegistryService {
    pub async fn save(user: Principal) -> String {
        InterCanisterService::call(
            Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap(),
            "save".to_string(),
        )
        .await
        .unwrap()
    }
}
