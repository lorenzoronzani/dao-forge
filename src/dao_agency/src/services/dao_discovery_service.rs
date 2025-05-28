use candid::Principal;

use common::services::InterCanisterService;

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
}
