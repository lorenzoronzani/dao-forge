use candid::Principal;

use super::InterCanisterService;

pub struct DaoDiscoveryService;

impl DaoDiscoveryService {
    pub async fn save(user: Principal, dao: Principal) -> String {
        InterCanisterService::call(
            Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(),
            "save".to_string(),
            (user, dao),
        )
        .await
        .unwrap();

        "Done".to_string()
    }
}
