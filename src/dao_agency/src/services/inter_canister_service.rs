use candid::Principal;
use ic_cdk::api::call::call;

pub struct InterCanisterService;

impl InterCanisterService {
    pub async fn call(
        canister_id: Principal,
        method: String,
        args: (Principal, Principal),
    ) -> Result<String, String> {
        let result: Result<(u64,), _> = call(canister_id, &method, args).await;

        println!("Result: {:?}", result);

        Ok("Done".to_string())
    }
}
