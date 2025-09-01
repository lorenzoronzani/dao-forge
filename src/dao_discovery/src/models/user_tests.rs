use super::*;
use candid::Principal;

fn create_test_user() -> User {
    User::new(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap())
}

#[test]
fn test_user_creation() {
    let user = create_test_user();

    // Test basic properties
    assert_eq!(
        user.id,
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
    );
    assert!(user.dao_ids.is_empty());
}

#[test]
fn test_user_new() {
    let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let user = User::new(principal);

    assert_eq!(user.id, principal);
    assert_eq!(user.dao_ids.len(), 0);
}

#[test]
fn test_user_add_dao() {
    let mut user = create_test_user();

    // Add first DAO
    user.add_dao(123);
    assert_eq!(user.dao_ids.len(), 1);
    assert!(user.dao_ids.contains(&123));

    // Add second DAO
    user.add_dao(456);
    assert_eq!(user.dao_ids.len(), 2);
    assert!(user.dao_ids.contains(&123));
    assert!(user.dao_ids.contains(&456));

    // Try to add duplicate DAO - should not be added
    user.add_dao(123);
    assert_eq!(user.dao_ids.len(), 2); // Still 2, not 3
    assert!(user.dao_ids.contains(&123));
}

#[test]
fn test_user_remove_dao() {
    let mut user = create_test_user();

    // Add some DAOs
    user.add_dao(123);
    user.add_dao(456);
    user.add_dao(789);
    assert_eq!(user.dao_ids.len(), 3);

    // Remove existing DAO
    user.remove_dao(456);
    assert_eq!(user.dao_ids.len(), 2);
    assert!(user.dao_ids.contains(&123));
    assert!(user.dao_ids.contains(&789));
    assert!(!user.dao_ids.contains(&456));

    // Try to remove non-existing DAO - should not change anything
    user.remove_dao(999);
    assert_eq!(user.dao_ids.len(), 2);

    // Remove all remaining DAOs
    user.remove_dao(123);
    user.remove_dao(789);
    assert!(user.dao_ids.is_empty());
}

#[test]
fn test_user_storable() {
    let mut original_user = create_test_user();
    original_user.add_dao(123);
    original_user.add_dao(456);

    // Test serialization
    let bytes = original_user.to_bytes();
    assert!(!bytes.is_empty());

    // Test deserialization
    let deserialized_user = User::from_bytes(bytes);

    // Verify data integrity
    assert_eq!(original_user.id, deserialized_user.id);
    assert_eq!(original_user.dao_ids.len(), deserialized_user.dao_ids.len());
    assert_eq!(original_user.dao_ids, deserialized_user.dao_ids);
}

#[test]
fn test_user_clone() {
    let mut original_user = create_test_user();
    original_user.add_dao(123);
    original_user.add_dao(456);

    let cloned_user = original_user.clone();

    // Verify clone works correctly
    assert_eq!(original_user.id, cloned_user.id);
    assert_eq!(original_user.dao_ids, cloned_user.dao_ids);
}

#[test]
fn test_user_dao_operations_workflow() {
    let mut user = create_test_user();

    // Complete workflow: add multiple, remove some, add more
    user.add_dao(100);
    user.add_dao(200);
    user.add_dao(300);
    assert_eq!(user.dao_ids.len(), 3);

    user.remove_dao(200);
    assert_eq!(user.dao_ids.len(), 2);
    assert!(user.dao_ids.contains(&100));
    assert!(user.dao_ids.contains(&300));

    user.add_dao(400);
    user.add_dao(500);
    assert_eq!(user.dao_ids.len(), 4);
    assert!(user.dao_ids.contains(&100));
    assert!(user.dao_ids.contains(&300));
    assert!(user.dao_ids.contains(&400));
    assert!(user.dao_ids.contains(&500));
}

#[test]
fn test_user_different_principals() {
    let user1 = User::new(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap());
    let user2 = User::new(Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap());
    let user3 = User::new(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap());

    // Test different users have different principals
    assert_ne!(user1.id, user2.id);
    assert_ne!(user2.id, user3.id);
    assert_ne!(user1.id, user3.id);

    // All should start with empty dao_ids
    assert!(user1.dao_ids.is_empty());
    assert!(user2.dao_ids.is_empty());
    assert!(user3.dao_ids.is_empty());
}

#[test]
fn test_user_edge_cases() {
    let mut user = create_test_user();

    // Test with ID 0
    user.add_dao(0);
    assert!(user.dao_ids.contains(&0));

    // Test with maximum u32
    user.add_dao(u32::MAX);
    assert!(user.dao_ids.contains(&u32::MAX));

    // Test removing from empty list (already tested but explicit)
    let mut empty_user = create_test_user();
    empty_user.remove_dao(123);
    assert!(empty_user.dao_ids.is_empty());
}
