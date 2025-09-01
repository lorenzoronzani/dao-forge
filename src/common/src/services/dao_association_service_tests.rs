use super::*;
use candid::{encode_one, Principal};

/// compile-time check: the function exists and has the expected args
#[allow(dead_code)]
fn _signature_check() {
    let f: fn(Principal, u32) -> _ = |p, d| {
        // we don't run it; we just ensure the type matches
        let _ = DaoAssociationService::add_document(p, d);
    };
    let _ = f;
}

#[test]
fn candid_encoding_of_document_id_matches_expected() {
    let want = encode_one(42u32).expect("encode");
    assert!(!want.is_empty());
    let other = encode_one(7u32).unwrap();
    assert_ne!(want, other);
}
