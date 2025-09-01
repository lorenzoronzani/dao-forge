
use super::*;
use candid::{Decode, Encode, Principal};
use ic_stable_structures::storable::Bound;
use std::borrow::Cow;

fn sample_config() -> Configuration {
    let p_mgmt = Principal::from_text("aaaaa-aa").unwrap(); // management canister
    let p_anon = Principal::anonymous(); // 2vxsx-fae
    let p_a = Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap(); // common test principal
    let p_b = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(); // common test principal

    Configuration::new(
        Some(p_mgmt),
        Some(p_anon),
        None,
        Some(p_a),
        None,
        Some(p_b),
        None,
    )
}

#[test]
fn default_is_all_none() {
    let d = Configuration::default();
    assert!(d.dao_agency_canister_id.is_none());
    assert!(d.sogc_publication_canister_id.is_none());
    assert!(d.dao_discovery_canister_id.is_none());
    assert!(d.documents_storage_canister_id.is_none());
    assert!(d.voting_canister_id.is_none());
    assert!(d.network_call_canister_id.is_none());
    assert!(d.dao_platform_canister_id.is_none());
}

#[test]
fn new_sets_all_fields() {
    let cfg = sample_config();

    assert_eq!(
        cfg.dao_agency_canister_id,
        Some(Principal::from_text("aaaaa-aa").unwrap())
    );
    assert_eq!(
        cfg.sogc_publication_canister_id,
        Some(Principal::anonymous())
    );
    assert_eq!(cfg.dao_discovery_canister_id, None);
    assert_eq!(
        cfg.documents_storage_canister_id,
        Some(Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap())
    );
    assert_eq!(cfg.voting_canister_id, None);
    assert_eq!(
        cfg.network_call_canister_id,
        Some(Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap())
    );
    assert_eq!(cfg.dao_platform_canister_id, None);
}

#[test]
fn candid_encode_decode_roundtrip() {
    let original = sample_config();
    let bytes = Encode!(&original).expect("candid encode");
    let decoded: Configuration = Decode!(&bytes, Configuration).expect("candid decode");

    // Field-by-field equality (Configuration doesn't derive PartialEq)
    assert_eq!(
        decoded.dao_agency_canister_id,
        original.dao_agency_canister_id
    );
    assert_eq!(
        decoded.sogc_publication_canister_id,
        original.sogc_publication_canister_id
    );
    assert_eq!(
        decoded.dao_discovery_canister_id,
        original.dao_discovery_canister_id
    );
    assert_eq!(
        decoded.documents_storage_canister_id,
        original.documents_storage_canister_id
    );
    assert_eq!(decoded.voting_canister_id, original.voting_canister_id);
    assert_eq!(
        decoded.network_call_canister_id,
        original.network_call_canister_id
    );
    assert_eq!(
        decoded.dao_platform_canister_id,
        original.dao_platform_canister_id
    );
}

#[test]
fn storable_roundtrip_and_bound() {
    let original = sample_config();

    // to_bytes produces candid; from_bytes must decode the same
    let cow = original.to_bytes();
    assert!(!cow.is_empty(), "encoded bytes should not be empty");

    let decoded = Configuration::from_bytes(Cow::Borrowed(cow.as_ref()));

    // Field-by-field equality
    assert_eq!(
        decoded.dao_agency_canister_id,
        original.dao_agency_canister_id
    );
    assert_eq!(
        decoded.sogc_publication_canister_id,
        original.sogc_publication_canister_id
    );
    assert_eq!(
        decoded.dao_discovery_canister_id,
        original.dao_discovery_canister_id
    );
    assert_eq!(
        decoded.documents_storage_canister_id,
        original.documents_storage_canister_id
    );
    assert_eq!(decoded.voting_canister_id, original.voting_canister_id);
    assert_eq!(
        decoded.network_call_canister_id,
        original.network_call_canister_id
    );
    assert_eq!(
        decoded.dao_platform_canister_id,
        original.dao_platform_canister_id
    );

    // Bound is explicitly Unbounded (important for stable-structures)
    assert!(matches!(Configuration::BOUND, Bound::Unbounded));
}

#[test]
#[should_panic] // because to_bytes/from_bytes use unwrap()
fn storable_from_bytes_panics_on_garbage() {
    let garbage = Cow::Borrowed(&[0xff, 0x00, 0x13, 0x37][..]);
    let _ = Configuration::from_bytes(garbage);
}
