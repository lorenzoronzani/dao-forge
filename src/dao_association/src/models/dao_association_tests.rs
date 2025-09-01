use super::*;

fn create_test_dao_association() -> DaoAssociation {
    DaoAssociation::new(
        "Test DAO Association".to_string(),
        "123 Test Street".to_string(),
        1234,
        "Test Town".to_string(),
        "test-uid-123".to_string(),
        "test-ch-456".to_string(),
        789,
        "Testing purpose".to_string(),
        vec![1, 2, 3],
        vec![],
        1000000,
        vec![10, 20],
        vec![100, 200],
    )
}

#[test]
fn test_dao_association_creation() {
    let dao = create_test_dao_association();

    // Test basic properties
    assert_eq!(dao.parent.name, "Test DAO Association");
    assert_eq!(dao.parent.address, "123 Test Street");
    assert_eq!(dao.parent.zip, 1234);
    assert_eq!(dao.parent.town, "Test Town");
    assert_eq!(dao.parent.uid, "test-uid-123");
    assert_eq!(dao.parent.ch_id, "test-ch-456");
    assert_eq!(dao.parent.frc_id, 789);
    assert_eq!(dao.parent.purpose, "Testing purpose");

    // Test collections
    assert_eq!(dao.parent.sogc_publications.len(), 3);
    assert_eq!(dao.parent.documents.len(), 2);
    assert_eq!(dao.parent.pools.len(), 2);

    // Test enums
    assert_eq!(dao.parent.legal_form, LegalForm::Association);
    assert_eq!(dao.parent.status, OrganizationStatus::Active);
}

#[test]
fn test_dao_association_default() {
    let default_dao = DaoAssociation::default();

    assert_eq!(default_dao.parent.legal_form, LegalForm::Corporation);
    assert_eq!(default_dao.parent.status, OrganizationStatus::Active);
}

#[test]
fn test_dao_association_storable() {
    let original_dao = create_test_dao_association();

    // Test serialization
    let bytes = original_dao.to_bytes();
    assert!(!bytes.is_empty());

    // Test deserialization
    let deserialized_dao = DaoAssociation::from_bytes(bytes);

    // Verify data integrity
    assert_eq!(original_dao.parent.name, deserialized_dao.parent.name);
    assert_eq!(original_dao.parent.address, deserialized_dao.parent.address);
    assert_eq!(original_dao.parent.zip, deserialized_dao.parent.zip);
    assert_eq!(original_dao.parent.uid, deserialized_dao.parent.uid);
    assert_eq!(original_dao.parent.purpose, deserialized_dao.parent.purpose);
}

#[test]
fn test_dao_association_clone() {
    let original_dao = create_test_dao_association();
    let cloned_dao = original_dao.clone();

    // Verify clone works correctly
    assert_eq!(original_dao.parent.name, cloned_dao.parent.name);
    assert_eq!(original_dao.parent.uid, cloned_dao.parent.uid);
    assert_eq!(
        original_dao.parent.sogc_publications,
        cloned_dao.parent.sogc_publications
    );
}
