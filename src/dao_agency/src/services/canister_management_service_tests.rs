use super::*;
use candid::{encode_args, Principal};
use ic_cdk::api::management_canister::main::{CanisterStatusResponse, WasmModule};

fn assert_future_result<T, F: core::future::Future<Output = Result<T, String>>>(_f: F) {}

#[test]
fn deploy_canister_signature_is_future_result_principal() {
    let wasm: WasmModule = vec![0x00, 0x61, 0x73, 0x6D]; // "\0asm"
    let init_args = encode_args(()).unwrap();
    let controllers = vec![Principal::from_text("aaaaa-aa").unwrap()];
    let fut = CanisterManagementService::deploy_canister(wasm, init_args, controllers);
    assert_future_result::<Principal, _>(fut);
}

#[test]
fn deploy_canister_accepts_multiple_controllers() {
    let wasm: WasmModule = vec![0x00, 0x61, 0x73, 0x6D];
    let init_args = encode_args((42u32,)).unwrap();
    let controllers = vec![
        Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap(),
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
    ];
    let fut = CanisterManagementService::deploy_canister(wasm, init_args, controllers);
    assert_future_result::<Principal, _>(fut);
}

#[test]
fn canister_status_signature_is_future_result_status() {
    let fut = CanisterManagementService::canister_status(Principal::from_text("aaaaa-aa").unwrap());
    assert_future_result::<CanisterStatusResponse, _>(fut);
}
