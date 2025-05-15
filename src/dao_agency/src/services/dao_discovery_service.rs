use candid::Principal;

use super::InterCanisterService;

pub struct DaoDiscoveryService;

impl DaoDiscoveryService {
    pub async fn save_user_dao(user: Principal, dao: Principal) {
        let _: Result<Vec<Principal>, String> = InterCanisterService::call(
            Principal::from_text("uzt4z-lp777-77774-qaabq-cai").unwrap(),
            &"save_user_dao",
            (user, dao),
        )
        .await;
    }
}
