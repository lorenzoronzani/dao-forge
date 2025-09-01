use super::*;
use crate::models::DaoAssociation;
use common::models::{Role, User};

fn create_test_users() -> Vec<User> {
    vec![
        User::new("user1".to_string(), Role::Member),
        User::new("user2".to_string(), Role::Board),
    ]
}

fn create_test_dao_association() -> DaoAssociation {
    DaoAssociation::new(
        "Test DAO".to_string(),
        "123 Test Street".to_string(),
        12345,
        "Testville".to_string(),
        "UID123".to_string(),
        "CHID456".to_string(),
        789,
        "To test things".to_string(),
        vec![1, 2, 3],
        create_test_users(),
        1672531200,
        vec![4, 5, 6],
        vec![7, 8, 9],
    )
}

#[test]
fn test_dao_association_save_and_get() {
    let test_dao = create_test_dao_association();

    // Test save
    let saved_dao = DaoAssociationRepository::save(test_dao.clone());

    // Verify save returns the same DAO
    assert_eq!(saved_dao.parent.name, test_dao.parent.name);
    assert_eq!(
        saved_dao.parent.members.len(),
        test_dao.parent.members.len()
    );

    // Test get
    let retrieved_dao = DaoAssociationRepository::get();

    // Verify get returns the saved DAO
    assert_eq!(retrieved_dao.parent.name, test_dao.parent.name);
    assert_eq!(retrieved_dao.parent.address, test_dao.parent.address);
    assert_eq!(retrieved_dao.parent.purpose, test_dao.parent.purpose);
    assert_eq!(retrieved_dao.parent.members.len(), 2);
}

#[test]
fn test_dao_association_overwrite() {
    // Save first DAO
    let first_dao = create_test_dao_association();
    DaoAssociationRepository::save(first_dao.clone());

    // Create and save second DAO with different values
    let second_dao = DaoAssociation::new(
        "Test DAO 2".to_string(),
        "456 Other Avenue".to_string(),
        54321,
        "Othertown".to_string(),
        "UID123-2".to_string(),
        "CHID456-2".to_string(),
        987,
        "To test overwriting".to_string(),
        vec![10],
        vec![],
        1672531201,
        vec![11],
        vec![12],
    );
    DaoAssociationRepository::save(second_dao.clone());

    // Verify the second DAO overwrote the first
    let retrieved_dao = DaoAssociationRepository::get();
    assert_eq!(retrieved_dao.parent.name, second_dao.parent.name);
    assert_ne!(retrieved_dao.parent.name, first_dao.parent.name);
    assert_eq!(retrieved_dao.parent.members.len(), 0);
}

#[test]
fn test_dao_association_get_default() {
    let dao = DaoAssociationRepository::get();
    let default_dao = DaoAssociation::default();

    // Verify it returns a valid DaoAssociation with default values
    assert_eq!(dao.parent.name, default_dao.parent.name);
    assert_eq!(dao.parent.address, default_dao.parent.address);
    assert_eq!(dao.parent.zip, default_dao.parent.zip);
    assert_eq!(dao.parent.town, default_dao.parent.town);
    assert_eq!(dao.parent.legal_form, default_dao.parent.legal_form);
    assert_eq!(dao.parent.status, default_dao.parent.status);
    assert_eq!(dao.parent.uid, default_dao.parent.uid);
    assert_eq!(dao.parent.ch_id, default_dao.parent.ch_id);
    assert_eq!(dao.parent.frc_id, default_dao.parent.frc_id);
    assert_eq!(dao.parent.purpose, default_dao.parent.purpose);
    assert!(dao.parent.sogc_publications.is_empty());
    assert!(dao.parent.members.is_empty());
    assert_eq!(dao.parent.created_at, default_dao.parent.created_at);
    assert!(dao.parent.documents.is_empty());
    assert!(dao.parent.pools.is_empty());
}
