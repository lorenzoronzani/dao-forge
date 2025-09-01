use super::*;
use candid::{Decode, Encode};

fn sample_dao() -> Dao {
    Dao::new(
        "Acme DAO".to_string(),
        "42 Blockchain Ave".to_string(),
        8000,
        "Zürich".to_string(),
        LegalForm::LimitedLiabilityCompany,
        OrganizationStatus::Active,
        "CHE-123.456.789".to_string(),
        "CH-020.3.012.345-6".to_string(),
        777_001,
        "Advance decentralized governance".to_string(),
        vec![1001, 1002, 1003],
        vec![],
        1_690_000_000,
        vec![11, 12],
        vec![21, 22, 23],
    )
}

#[test]
fn legal_form_default_is_corporation() {
    assert_eq!(LegalForm::default(), LegalForm::Corporation);
}

#[test]
fn organization_status_default_is_active() {
    assert_eq!(OrganizationStatus::default(), OrganizationStatus::Active);
}

#[test]
fn dao_default_is_sane() {
    let d = Dao::default();
    assert_eq!(d.name, "");
    assert_eq!(d.address, "");
    assert_eq!(d.zip, 0);
    assert_eq!(d.town, "");
    assert_eq!(d.legal_form, LegalForm::Corporation);
    assert_eq!(d.status, OrganizationStatus::Active);
    assert_eq!(d.uid, "");
    assert_eq!(d.ch_id, "");
    assert_eq!(d.frc_id, 0);
    assert_eq!(d.purpose, "");
    assert!(d.sogc_publications.is_empty());
    assert!(d.members.is_empty());
    assert_eq!(d.created_at, 0);
    assert!(d.documents.is_empty());
    assert!(d.pools.is_empty());
}

#[test]
fn dao_new_sets_all_fields() {
    let d = sample_dao();

    assert_eq!(d.name, "Acme DAO");
    assert_eq!(d.address, "42 Blockchain Ave");
    assert_eq!(d.zip, 8000);
    assert_eq!(d.town, "Zürich");
    assert_eq!(d.legal_form, LegalForm::LimitedLiabilityCompany);
    assert_eq!(d.status, OrganizationStatus::Active);
    assert_eq!(d.uid, "CHE-123.456.789");
    assert_eq!(d.ch_id, "CH-020.3.012.345-6");
    assert_eq!(d.frc_id, 777_001);
    assert_eq!(d.purpose, "Advance decentralized governance");
    assert_eq!(d.sogc_publications, vec![1001, 1002, 1003]);
    assert!(d.members.is_empty());
    assert_eq!(d.created_at, 1_690_000_000);
    assert_eq!(d.documents, vec![11, 12]);
    assert_eq!(d.pools, vec![21, 22, 23]);
}

#[test]
fn candid_encode_decode_roundtrip() {
    let original = sample_dao();

    let bytes = Encode!(&original).expect("candid encode");
    let decoded: Dao = Decode!(&bytes, Dao).expect("candid decode");

    // Field-by-field equality (Dao doesn't derive PartialEq)
    assert_eq!(decoded.name, original.name);
    assert_eq!(decoded.address, original.address);
    assert_eq!(decoded.zip, original.zip);
    assert_eq!(decoded.town, original.town);
    assert_eq!(decoded.legal_form, original.legal_form);
    assert_eq!(decoded.status, original.status);
    assert_eq!(decoded.uid, original.uid);
    assert_eq!(decoded.ch_id, original.ch_id);
    assert_eq!(decoded.frc_id, original.frc_id);
    assert_eq!(decoded.purpose, original.purpose);
    assert_eq!(decoded.sogc_publications, original.sogc_publications);
    assert_eq!(decoded.members.len(), original.members.len());
    assert_eq!(decoded.created_at, original.created_at);
    assert_eq!(decoded.documents, original.documents);
    assert_eq!(decoded.pools, original.pools);
}
