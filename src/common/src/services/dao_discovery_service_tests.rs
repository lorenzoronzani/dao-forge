use super::*;
use candid::{encode_args, Principal};

fn assert_future_unit<F: core::future::Future<Output = ()>>(_f: F) {}

#[test]
fn save_user_dao_signature_is_future_unit() {
    let canister = Principal::from_text("aaaaa-aa").unwrap();
    let user = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
    let dao = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let fut = DaoDiscoveryService::save_user_dao(canister, user, dao);
    assert_future_unit(fut);
}

#[test]
fn remove_user_dao_signature_is_future_unit() {
    let canister = Principal::from_text("aaaaa-aa").unwrap();
    let user = Principal::anonymous();
    let dao = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    let fut = DaoDiscoveryService::remove_user_dao(canister, user, dao);
    assert_future_unit(fut);
}

#[test]
fn candid_tuple_encoding_is_nonempty_and_order_sensitive() {
    let user = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
    let dao = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();

    let enc_user_dao = encode_args((user, dao)).unwrap();
    let enc_dao_user = encode_args((dao, user)).unwrap();

    assert!(!enc_user_dao.is_empty());
    assert_ne!(enc_user_dao, enc_dao_user);
}
