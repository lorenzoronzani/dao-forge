use candid::Principal;

use crate::services::InterCanisterService;

pub struct DaoDiscoveryService;

impl DaoDiscoveryService {
    pub async fn save_user_dao(user: Principal, dao: Principal) {
        let _: Result<Vec<Principal>, String> = InterCanisterService::call(
            Principal::from_text("ggppn-oqaaa-aaaaa-aaaaq-azy").unwrap(),
            &"save_user_dao",
            (user, dao),
        )
        .await;
    }

    pub async fn remove_user_dao(user: Principal, dao: Principal) {
        let _: Result<Vec<Principal>, String> = InterCanisterService::call(
            Principal::from_text("ggppn-oqaaa-aaaaa-aaaaq-azy").unwrap(),
            &"remove_user_dao",
            (user, dao),
        )
        .await;
    }
}
