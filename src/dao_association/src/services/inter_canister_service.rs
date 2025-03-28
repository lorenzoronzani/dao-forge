use candid::Principal;
use ic_cdk::api::call::call;

pub struct InterCanisterService;

impl InterCanisterService {
    pub async fn call(canister_id: Principal, method: String) -> Result<String, String> {
        let result: Result<(u64,), _> = call(canister_id, &method, ()).await;

        Ok("Done".to_string())
    }
}
