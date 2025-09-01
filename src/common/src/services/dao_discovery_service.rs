use candid::Principal;

use crate::services::InterCanisterService;

pub struct DaoDiscoveryService;

impl DaoDiscoveryService {
    pub async fn save_user_dao(canister_id: Principal, user: Principal, dao: Principal) {
        let _: Result<Vec<Principal>, String> =
            InterCanisterService::call(canister_id, &"save_user_dao", (user, dao)).await;
    }

    pub async fn remove_user_dao(canister_id: Principal, user: Principal, dao: Principal) {
        let _: Result<Vec<Principal>, String> =
            InterCanisterService::call(canister_id, &"remove_user_dao", (user, dao)).await;
    }
}

#[cfg(test)]
#[path = "dao_discovery_service_tests.rs"]
mod dao_discovery_service_tests;
