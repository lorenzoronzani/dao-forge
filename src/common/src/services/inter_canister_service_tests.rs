use super::*;
use candid::{decode_one, encode_args, encode_one, Principal};

fn assert_future_result<T, F: core::future::Future<Output = Result<T, String>>>(_f: F) {}

#[test]
fn call_signature_vec_principal() {
    let can = Principal::from_text("aaaaa-aa").unwrap();
    let user = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
    let dao = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let fut = InterCanisterService::call::<_, Vec<Principal>>(can, "method", (user, dao));
    assert_future_result::<Vec<Principal>, _>(fut);
}

#[test]
fn call_raw_signature_u32() {
    let can = Principal::from_text("aaaaa-aa").unwrap();
    let bytes = encode_one(7u32).unwrap();
    let fut = InterCanisterService::call_raw::<u32>(can, "store", bytes);
    assert_future_result::<u32, _>(fut);
}

#[test]
fn candid_tuple_encoding_is_ordered() {
    let a = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
    let b = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let ab = encode_args((a, b)).unwrap();
    let ba = encode_args((b, a)).unwrap();
    assert!(!ab.is_empty());
    assert_ne!(ab, ba);
}

#[test]
fn candid_decode_one_roundtrip_u32() {
    let e = encode_one(123u32).unwrap();
    let d: u32 = decode_one(&e).unwrap();
    assert_eq!(d, 123);
}
