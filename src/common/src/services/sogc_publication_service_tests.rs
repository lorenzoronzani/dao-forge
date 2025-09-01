use super::*;
use candid::{encode_args, encode_one, Principal};

fn assert_future<F: core::future::Future<Output = Result<u32, String>>>(_f: F) {}

fn mk_args() -> crate::types::SogcPublicationArgs {
    crate::types::SogcPublicationArgs {
        daily_number: 101,
        publication_date: 1_700_000_000,
        mutations: Vec::<crate::types::Mutation>::new(),
        description: "desc".into(),
    }
}

#[test]
fn signature_is_future_result_u32() {
    let fut = SogcPublicationService::publish(
        Principal::from_text("aaaaa-aa").unwrap(),
        101,
        1_700_000_000,
        Vec::<crate::types::Mutation>::new(),
        "desc".into(),
    );
    assert_future(fut);
}

#[test]
fn candid_encoding_uses_single_record_arg_in_tuple() {
    let rec_via_tuple = encode_args((mk_args(),)).unwrap();
    let rec_via_one = encode_one(mk_args()).unwrap();
    assert_eq!(rec_via_tuple, rec_via_one);

    let a = mk_args();
    let triple = encode_args((
        a.daily_number,
        a.publication_date,
        Vec::<crate::types::Mutation>::new(),
        a.description,
    ))
    .unwrap();
    assert_ne!(rec_via_tuple, triple);
}

#[test]
fn encoding_changes_when_fields_change() {
    let e1 = encode_one(mk_args()).unwrap();

    let changed = crate::types::SogcPublicationArgs {
        daily_number: 102,
        publication_date: 1_700_000_000,
        mutations: Vec::<crate::types::Mutation>::new(),
        description: "desc".into(),
    };
    let e2 = encode_one(changed).unwrap();
    assert_ne!(e1, e2);

    let changed2 = crate::types::SogcPublicationArgs {
        daily_number: 101,
        publication_date: 1_700_000_000,
        mutations: Vec::<crate::types::Mutation>::new(),
        description: "desc v2".into(),
    };
    let e3 = encode_one(changed2).unwrap();
    assert_ne!(e1, e3);
}
