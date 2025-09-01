use super::*;
use candid::{encode_args, encode_one, Principal};

fn assert_future<F: core::future::Future<Output = Result<String, String>>>(_f: F) {}

fn mk_args() -> crate::types::EmailArgs {
    crate::types::EmailArgs {
        to: "to@example.com".into(),
        subject: "Hello".into(),
        message: "Hi there".into(),
        action_url: "https://example.com".into(),
    }
}

#[test]
fn signature_is_future_result_string() {
    let fut = NetworkCallService::send_email(
        Principal::from_text("aaaaa-aa").unwrap(),
        "to@example.com".into(),
        "Hello".into(),
        "Hi".into(),
        "https://example.com".into(),
    );
    assert_future(fut);
}

#[test]
fn candid_encoding_uses_single_record_arg_in_tuple() {
    let args = mk_args();
    let as_single = encode_args((args.clone(),)).unwrap();
    let as_tuple = encode_args((
        args.to.clone(),
        args.subject.clone(),
        args.message.clone(),
        args.action_url.clone(),
    ))
    .unwrap();
    assert!(!as_single.is_empty());
    assert_ne!(as_single, as_tuple);
    let via_one = encode_one(args).unwrap();
    assert_eq!(as_single, via_one);
}

#[test]
fn encoding_changes_when_fields_change() {
    let mut a = mk_args();
    let e1 = encode_args((a.clone(),)).unwrap();
    a.subject.push_str("!");
    let e2 = encode_args((a.clone(),)).unwrap();
    assert_ne!(e1, e2);
    a.message.push_str(" More.");
    let e3 = encode_args((a,)).unwrap();
    assert_ne!(e2, e3);
}
