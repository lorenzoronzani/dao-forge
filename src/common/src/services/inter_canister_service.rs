use candid::utils::ArgumentEncoder;
use candid::{decode_one, CandidType, Principal};
use ic_cdk::api::call::{call, call_raw};
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

    pub async fn call_raw<O: CandidType + DeserializeOwned + Debug>(
        canister_id: Principal,
        method: &str,
        args_encoded: Vec<u8>,
    ) -> Result<O, String> {
        let result: Result<Vec<u8>, _> = call_raw(canister_id, method, args_encoded, 0).await;

        match result {
            Ok(response) => {
                let decoded_response = decode_one(&response).map_err(|e| e.to_string())?;
                println!("Success: {:?}", decoded_response);
                Ok(decoded_response)
            }
            Err((code, msg)) => {
                let error = format!("Call failed with code {:?}: {}", code, msg);
                println!("Error: {}", error);
                Err(error)
            }
        }
    }
}

#[cfg(test)]
#[path = "inter_canister_service_tests.rs"]
mod inter_canister_service_tests;
