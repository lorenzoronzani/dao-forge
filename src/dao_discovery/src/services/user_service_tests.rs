use super::*;
use candid::Principal;

fn create_test_principal(id: u8) -> Principal {
    match id {
        1 => Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
        2 => Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
        3 => Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
        4 => Principal::from_text("ggppn-oqaaa-aaaaa-aaaaq-azy").unwrap(),
        5 => Principal::from_text("ueq6w-kyaaa-aaaaa-aaaaq-a2a").unwrap(),
        _ => Principal::anonymous(),
    }
}

#[test]
fn test_user_service_save() {
    let principal = create_test_principal(1);
    let dao_ids = vec![123, 456, 789];

    // Test save method
    let saved_user = UserService::save(principal, dao_ids.clone());

    // Verify user was created with correct data
    assert_eq!(saved_user.id, principal);
    assert_eq!(saved_user.dao_ids, dao_ids);
}

#[test]
fn test_user_service_save_and_get() {
    let principal = create_test_principal(2);
    let dao_ids = vec![100, 200];

    // Save a user
    let _saved_user = UserService::save(principal, dao_ids.clone());

    // Get the user
    let retrieved_user = UserService::get(principal);

    // Verify get returns the saved user
    assert!(retrieved_user.is_some());
    let retrieved = retrieved_user.unwrap();
    assert_eq!(retrieved.id, principal);
    assert_eq!(retrieved.dao_ids, dao_ids);
}

#[test]
fn test_user_service_get_nonexistent() {
    let nonexistent = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

    // Test getting a user that doesn't exist
    let result = UserService::get(nonexistent);
    assert!(result.is_none());
}

#[test]
fn test_user_service_save_empty_dao_ids() {
    let principal = create_test_principal(3);

    // Save user with empty dao_ids
    let saved_user = UserService::save(principal, vec![]);

    assert_eq!(saved_user.id, principal);
    assert!(saved_user.dao_ids.is_empty());

    // Verify it's stored correctly
    let retrieved = UserService::get(principal).unwrap();
    assert!(retrieved.dao_ids.is_empty());
}

#[test]
fn test_user_service_update_add_dao() {
    let principal = create_test_principal(4);
    let initial_dao_ids = vec![111, 222];

    // Save initial user
    UserService::save(principal, initial_dao_ids.clone());

    // Add a DAO
    let updated_user = UserService::update(principal, 333, true);

    // Verify DAO was added
    assert_eq!(updated_user.dao_ids.len(), 3);
    assert!(updated_user.dao_ids.contains(&111));
    assert!(updated_user.dao_ids.contains(&222));
    assert!(updated_user.dao_ids.contains(&333));

    // Verify it's persisted
    let retrieved = UserService::get(principal).unwrap();
    assert!(retrieved.dao_ids.contains(&333));
}

#[test]
fn test_user_service_update_remove_dao() {
    let principal = create_test_principal(5);
    let initial_dao_ids = vec![444, 555, 666];

    // Save initial user
    UserService::save(principal, initial_dao_ids.clone());

    // Remove a DAO
    let updated_user = UserService::update(principal, 555, false);

    // Verify DAO was removed
    assert_eq!(updated_user.dao_ids.len(), 2);
    assert!(updated_user.dao_ids.contains(&444));
    assert!(updated_user.dao_ids.contains(&666));
    assert!(!updated_user.dao_ids.contains(&555));

    // Verify it's persisted
    let retrieved = UserService::get(principal).unwrap();
    assert!(!retrieved.dao_ids.contains(&555));
}

#[test]
fn test_user_service_update_add_duplicate() {
    let principal = create_test_principal(1);
    let initial_dao_ids = vec![777, 888];

    // Save initial user
    UserService::save(principal, initial_dao_ids.clone());

    // Try to add existing DAO (should not create duplicate)
    let updated_user = UserService::update(principal, 777, true);

    // Verify no duplicate was added
    assert_eq!(updated_user.dao_ids.len(), 2);
    assert!(updated_user.dao_ids.contains(&777));
    assert!(updated_user.dao_ids.contains(&888));
}

#[test]
fn test_user_service_update_remove_nonexistent() {
    let principal = create_test_principal(2);
    let initial_dao_ids = vec![999];

    // Save initial user
    UserService::save(principal, initial_dao_ids.clone());

    // Try to remove non-existent DAO
    let updated_user = UserService::update(principal, 123456, false);

    // Verify original DAOs are unchanged
    assert_eq!(updated_user.dao_ids.len(), 1);
    assert!(updated_user.dao_ids.contains(&999));
}

#[test]
fn test_user_service_multiple_updates() {
    let principal = create_test_principal(3);

    // Save initial user with empty dao_ids
    UserService::save(principal, vec![]);

    // Add multiple DAOs
    UserService::update(principal, 1, true);
    UserService::update(principal, 2, true);
    UserService::update(principal, 3, true);

    let user_after_adds = UserService::get(principal).unwrap();
    assert_eq!(user_after_adds.dao_ids.len(), 3);

    // Remove some DAOs
    UserService::update(principal, 2, false);

    let final_user = UserService::get(principal).unwrap();
    assert_eq!(final_user.dao_ids.len(), 2);
    assert!(final_user.dao_ids.contains(&1));
    assert!(final_user.dao_ids.contains(&3));
    assert!(!final_user.dao_ids.contains(&2));
}

#[test]
fn test_user_service_workflow() {
    let principal = create_test_principal(4);
    let initial_daos = vec![10, 20, 30];

    // Complete workflow: save -> get -> update -> get
    let saved_user = UserService::save(principal, initial_daos.clone());
    assert_eq!(saved_user.dao_ids, initial_daos);

    let retrieved1 = UserService::get(principal).unwrap();
    assert_eq!(retrieved1.dao_ids, initial_daos);

    // Add and remove DAOs
    UserService::update(principal, 40, true); // Add 40
    UserService::update(principal, 20, false); // Remove 20

    let final_user = UserService::get(principal).unwrap();
    assert_eq!(final_user.dao_ids.len(), 3);
    assert!(final_user.dao_ids.contains(&10));
    assert!(final_user.dao_ids.contains(&30));
    assert!(final_user.dao_ids.contains(&40));
    assert!(!final_user.dao_ids.contains(&20));
}

#[test]
fn test_user_service_overwrite_save() {
    let principal = create_test_principal(5);

    // Save initial user
    UserService::save(principal, vec![1, 2, 3]);

    // Save again with different dao_ids (should overwrite)
    let updated_user = UserService::save(principal, vec![4, 5]);

    assert_eq!(updated_user.dao_ids, vec![4, 5]);

    // Verify overwrite persisted
    let retrieved = UserService::get(principal).unwrap();
    assert_eq!(retrieved.dao_ids, vec![4, 5]);
    assert!(!retrieved.dao_ids.contains(&1));
}
