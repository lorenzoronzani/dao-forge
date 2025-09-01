use super::*;
use candid::{encode_args, encode_one, Principal};

fn assert_future<F: core::future::Future<Output = Result<u32, String>>>(_f: F) {}

fn mk_args() -> crate::types::DocumentArgs {
    crate::types::DocumentArgs {
        name: "report.pdf".into(),
        content_type: "application/pdf".into(),
        content: (0u8..=15).collect(),
    }
}

#[test]
fn signature_is_future_result_u32() {
    let fut = DocumentsStorageService::store_document(
        Principal::from_text("aaaaa-aa").unwrap(),
        "report.pdf".into(),
        "application/pdf".into(),
        vec![1, 2, 3],
    );
    assert_future(fut);
}

#[test]
fn candid_encoding_uses_single_record_arg() {
    let args = mk_args();
    let rec = encode_args((args.clone(),)).unwrap();
    let triple = encode_args((
        args.name.clone(),
        args.content_type.clone(),
        args.content.clone(),
    ))
    .unwrap();
    assert!(!rec.is_empty());
    assert_ne!(rec, triple);
    let rec_via_one = encode_one(args).unwrap();
    assert_eq!(rec, rec_via_one);
}

#[test]
fn encoding_changes_when_fields_change() {
    let mut a = mk_args();
    let e1 = encode_args((a.clone(),)).unwrap();
    a.name.push_str("-v2");
    let e2 = encode_args((a.clone(),)).unwrap();
    assert_ne!(e1, e2);
    a.content[0] ^= 0xff;
    let e3 = encode_args((a,)).unwrap();
    assert_ne!(e2, e3);
}
