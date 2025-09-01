use super::*;
use candid::{Decode, Encode};

#[test]
fn role_default_is_member() {
    assert_eq!(Role::default(), Role::Member);
}

#[test]
fn user_default_is_sane() {
    let u = User::default();
    assert_eq!(u.id, "");
    assert_eq!(u.role, Role::Member);
}

#[test]
fn user_new_sets_fields() {
    let u1 = User::new("alice".into(), Role::Member);
    assert_eq!(u1.id, "alice");
    assert_eq!(u1.role, Role::Member);

    let u2 = User::new("bob".into(), Role::Board);
    assert_eq!(u2.id, "bob");
    assert_eq!(u2.role, Role::Board);
}

#[test]
fn candid_encode_decode_roundtrip() {
    let original = User::new("carol".into(), Role::Board);
    let bytes = Encode!(&original).expect("candid encode");
    let decoded: User = Decode!(&bytes, User).expect("candid decode");
    assert_eq!(decoded.id, original.id);
    assert_eq!(decoded.role, original.role);
}
