use candid::utils::ArgumentEncoder;
use candid::{CandidType, Principal};
use ic_cdk::api::call::call;
use ic_cdk::println;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub struct InterCanisterService;

impl InterCanisterService {
    pub async fn call<I: ArgumentEncoder, O: CandidType + DeserializeOwned + Debug>(
        canister_id: Principal,
        method: &str,
        args: I,
    ) -> Result<O, String> {
        let result: Result<(O,), _> = call(canister_id, method, args).await;

        match result {
            Ok((response,)) => {
                println!("Success: {:?}", response);
                Ok(response)
            }
            Err((code, msg)) => {
                let error = format!("Call failed with code {:?}: {}", code, msg);
                println!("Error: {}", error);
                Err(error)
            }
        }
    }
}
