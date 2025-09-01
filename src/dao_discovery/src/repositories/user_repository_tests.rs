use super::*;
use crate::models::User;
use candid::Principal;

fn create_test_user() -> User {
    User::new(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap())
}

fn create_test_user_with_principal(principal_text: &str) -> User {
    User::new(Principal::from_text(principal_text).unwrap())
}

#[test]
fn test_user_repository_save_and_get() {
    let test_user = create_test_user();
    let user_id = test_user.id;

    // Test save
    let saved_user = UserRepository::save(test_user.clone());

    // Verify save returns the same user
    assert_eq!(saved_user.id, test_user.id);
    assert_eq!(saved_user.dao_ids, test_user.dao_ids);

    // Test get
    let retrieved_user = UserRepository::get(user_id);

    // Verify get returns the saved user
    assert!(retrieved_user.is_some());
    let retrieved = retrieved_user.unwrap();
    assert_eq!(retrieved.id, test_user.id);
    assert_eq!(retrieved.dao_ids, test_user.dao_ids);
}

#[test]
fn test_user_repository_get_nonexistent() {
    let nonexistent_id = Principal::from_text("aaaaa-aa").unwrap();

    // Test getting a user that doesn't exist
    let result = UserRepository::get(nonexistent_id);
    assert!(result.is_none());
}

#[test]
fn test_user_repository_save_multiple() {
    let user1 = create_test_user_with_principal("rrkah-fqaaa-aaaaa-aaaaq-cai");
    let user2 = create_test_user_with_principal("rdmx6-jaaaa-aaaaa-aaadq-cai");
    let user3 = create_test_user_with_principal("be2us-64aaa-aaaaa-qaabq-cai");

    // Save multiple users
    UserRepository::save(user1.clone());
    UserRepository::save(user2.clone());
    UserRepository::save(user3.clone());

    // Verify all can be retrieved
    let retrieved1 = UserRepository::get(user1.id).unwrap();
    let retrieved2 = UserRepository::get(user2.id).unwrap();
    let retrieved3 = UserRepository::get(user3.id).unwrap();

    assert_eq!(retrieved1.id, user1.id);
    assert_eq!(retrieved2.id, user2.id);
    assert_eq!(retrieved3.id, user3.id);
    assert_eq!(retrieved1.dao_ids, user1.dao_ids);
    assert_eq!(retrieved2.dao_ids, user2.dao_ids);
    assert_eq!(retrieved3.dao_ids, user3.dao_ids);
}

#[test]
fn test_user_repository_update() {
    let mut test_user = create_test_user();
    let user_id = test_user.id;

    // Save initial user
    UserRepository::save(test_user.clone());

    // Modify the user
    test_user.add_dao(123);
    test_user.add_dao(456);

    // Update the user
    let updated_user = UserRepository::update(test_user.clone());

    // Verify update returns the updated user
    assert_eq!(updated_user.id, test_user.id);
    assert_eq!(updated_user.dao_ids, test_user.dao_ids);
    assert!(updated_user.dao_ids.contains(&123));
    assert!(updated_user.dao_ids.contains(&456));

    // Verify the user was actually updated in storage
    let retrieved_user = UserRepository::get(user_id).unwrap();
    assert_eq!(retrieved_user.dao_ids.len(), 2);
    assert!(retrieved_user.dao_ids.contains(&123));
    assert!(retrieved_user.dao_ids.contains(&456));
}

#[test]
fn test_user_repository_save_vs_update() {
    let mut test_user = create_test_user();
    let user_id = test_user.id;

    // Save initial user
    UserRepository::save(test_user.clone());

    // Modify and save (should work same as update)
    test_user.add_dao(789);
    let saved_again = UserRepository::save(test_user.clone());

    // Verify save and update have same behavior
    assert_eq!(saved_again.dao_ids.len(), 1);
    assert!(saved_again.dao_ids.contains(&789));

    // Modify and update
    test_user.add_dao(101112);
    let _updated = UserRepository::update(test_user.clone());

    // Verify both methods result in same storage behavior
    let final_user = UserRepository::get(user_id).unwrap();
    assert_eq!(final_user.dao_ids.len(), 2);
    assert!(final_user.dao_ids.contains(&789));
    assert!(final_user.dao_ids.contains(&101112));
}

#[test]
fn test_user_repository_overwrite() {
    let original_user = create_test_user();
    let user_id = original_user.id;

    // Save original user
    UserRepository::save(original_user.clone());

    // Create updated user with same ID but different dao_ids
    let mut updated_user = User::new(user_id);
    updated_user.add_dao(999);
    updated_user.add_dao(888);

    // Save updated user (should overwrite)
    UserRepository::save(updated_user.clone());

    // Verify the user was updated, not duplicated
    let retrieved = UserRepository::get(user_id).unwrap();
    assert_eq!(retrieved.id, user_id);
    assert_eq!(retrieved.dao_ids.len(), 2);
    assert!(retrieved.dao_ids.contains(&999));
    assert!(retrieved.dao_ids.contains(&888));
    assert_ne!(retrieved.dao_ids, original_user.dao_ids);
}

#[test]
fn test_user_repository_workflow() {
    let user_principal = Principal::from_text("2vxsx-fae").unwrap();
    let mut user = User::new(user_principal);

    // Initial state: user doesn't exist
    assert!(UserRepository::get(user_principal).is_none());

    // Save user
    let saved = UserRepository::save(user.clone());
    assert_eq!(saved.id, user.id);
    assert!(saved.dao_ids.is_empty());

    // Verify it exists
    let retrieved = UserRepository::get(user_principal);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, user_principal);

    // Update user with DAOs
    user.add_dao(100);
    user.add_dao(200);
    let updated = UserRepository::update(user.clone());
    assert_eq!(updated.dao_ids.len(), 2);

    // Verify updates are persisted
    let final_user = UserRepository::get(user_principal).unwrap();
    assert!(final_user.dao_ids.contains(&100));
    assert!(final_user.dao_ids.contains(&200));
}

#[test]
fn test_user_repository_with_complex_dao_operations() {
    let mut user = create_test_user();
    let user_id = user.id;

    // Save initial user
    UserRepository::save(user.clone());

    // Add multiple DAOs
    user.add_dao(1);
    user.add_dao(2);
    user.add_dao(3);
    user.add_dao(4);
    UserRepository::update(user.clone());

    // Verify all DAOs are saved
    let retrieved = UserRepository::get(user_id).unwrap();
    assert_eq!(retrieved.dao_ids.len(), 4);

    // Remove some DAOs
    user.remove_dao(2);
    user.remove_dao(4);
    UserRepository::update(user.clone());

    // Verify removals are persisted
    let final_user = UserRepository::get(user_id).unwrap();
    assert_eq!(final_user.dao_ids.len(), 2);
    assert!(final_user.dao_ids.contains(&1));
    assert!(final_user.dao_ids.contains(&3));
    assert!(!final_user.dao_ids.contains(&2));
    assert!(!final_user.dao_ids.contains(&4));
}
