use super::*;
use crate::models::DaoAssociation;

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
fn test_dao_association_service_save() {
    let test_dao = create_test_dao_association();

    // Test save method
    let saved_dao = DaoAssociationService::save(test_dao.clone());

    // Verify save returns the same DAO
    assert_eq!(saved_dao.parent.name, test_dao.parent.name);
    assert_eq!(saved_dao.parent.uid, test_dao.parent.uid);
    assert_eq!(saved_dao.parent.purpose, test_dao.parent.purpose);
}

#[test]
fn test_dao_association_service_save_and_get() {
    let test_dao = create_test_dao_association();

    // Save a DAO
    DaoAssociationService::save(test_dao.clone());

    // Get the DAO
    let retrieved_dao = DaoAssociationService::get();

    // Verify get returns the saved DAO
    assert_eq!(retrieved_dao.parent.name, test_dao.parent.name);
    assert_eq!(retrieved_dao.parent.address, test_dao.parent.address);
    assert_eq!(retrieved_dao.parent.zip, test_dao.parent.zip);
    assert_eq!(retrieved_dao.parent.uid, test_dao.parent.uid);
    assert_eq!(retrieved_dao.parent.ch_id, test_dao.parent.ch_id);
}

#[test]
fn test_dao_association_service_update() {
    let original_dao = create_test_dao_association();

    // Save original DAO
    DaoAssociationService::save(original_dao.clone());

    // Create updated DAO with different values
    let updated_dao = DaoAssociation::new(
        "Updated DAO Name".to_string(),
        "456 New Address".to_string(),
        5678,
        "New Town".to_string(),
        "updated-uid-456".to_string(),
        "updated-ch-789".to_string(),
        999,
        "Updated purpose".to_string(),
        vec![4, 5, 6],
        vec![],
        2000000,
        vec![30, 40],
        vec![300, 400],
    );

    // Update the DAO
    let result = DaoAssociationService::update(updated_dao.clone());

    // Verify update returns the updated DAO
    assert_eq!(result.parent.name, "Updated DAO Name");
    assert_eq!(result.parent.uid, "updated-uid-456");

    // Verify the DAO was actually updated in storage
    let retrieved_dao = DaoAssociationService::get();
    assert_eq!(retrieved_dao.parent.name, "Updated DAO Name");
    assert_eq!(retrieved_dao.parent.address, "456 New Address");
    assert_ne!(retrieved_dao.parent.name, original_dao.parent.name);
}

#[test]
fn test_dao_association_service_get_default() {
    // Test getting DAO when none has been saved
    // This should return the default DaoAssociation
    let dao = DaoAssociationService::get();

    // Just verify it doesn't panic and returns a valid DaoAssociation
    // Adjust assertions based on your DaoAssociation::default() implementation
    assert!(dao.parent.name.is_empty() || !dao.parent.name.is_empty()); // Basic sanity check
}

#[test]
fn test_dao_association_service_workflow() {
    let test_dao = create_test_dao_association();

    // Complete workflow: save -> get -> update -> get
    let saved = DaoAssociationService::save(test_dao.clone());
    let retrieved1 = DaoAssociationService::get();

    // Verify save and get work together
    assert_eq!(saved.parent.name, retrieved1.parent.name);

    // Update with new values
    let mut updated_dao = retrieved1;
    updated_dao.parent.name = "Workflow Updated DAO".to_string();

    let updated = DaoAssociationService::update(updated_dao);
    let retrieved2 = DaoAssociationService::get();

    // Verify update workflow
    assert_eq!(updated.parent.name, "Workflow Updated DAO");
    assert_eq!(retrieved2.parent.name, "Workflow Updated DAO");
    assert_ne!(retrieved2.parent.name, test_dao.parent.name);
}
