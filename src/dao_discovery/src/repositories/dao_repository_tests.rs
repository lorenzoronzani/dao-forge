use super::*;
use crate::models::Dao;
use candid::Principal;

fn create_test_dao(id: u32) -> Dao {
    Dao::new(
        id,
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
    )
}

#[test]
fn test_dao_repository_save_and_get() {
    let test_dao = create_test_dao(123);

    // Test save
    let saved_dao = DaoRepository::save(test_dao.clone());

    // Verify save returns the same dao
    assert_eq!(saved_dao.id, test_dao.id);
    assert_eq!(saved_dao.canister, test_dao.canister);

    // Test get
    let retrieved_dao = DaoRepository::get(123);

    // Verify get returns the saved dao
    assert!(retrieved_dao.is_some());
    let retrieved = retrieved_dao.unwrap();
    assert_eq!(retrieved.id, test_dao.id);
    assert_eq!(retrieved.canister, test_dao.canister);
}

#[test]
fn test_dao_repository_get_nonexistent() {
    // Test getting a DAO that doesn't exist
    let result = DaoRepository::get(999);
    assert!(result.is_none());
}

#[test]
fn test_dao_repository_save_multiple() {
    let dao1 = create_test_dao(100);
    let dao2 = Dao::new(
        200,
        Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap(),
    );
    let dao3 = Dao::new(
        300,
        Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(),
    );

    // Save multiple DAOs
    DaoRepository::save(dao1.clone());
    DaoRepository::save(dao2.clone());
    DaoRepository::save(dao3.clone());

    // Verify all can be retrieved
    let retrieved1 = DaoRepository::get(100).unwrap();
    let retrieved2 = DaoRepository::get(200).unwrap();
    let retrieved3 = DaoRepository::get(300).unwrap();

    assert_eq!(retrieved1.id, dao1.id);
    assert_eq!(retrieved2.id, dao2.id);
    assert_eq!(retrieved3.id, dao3.id);
    assert_eq!(retrieved1.canister, dao1.canister);
    assert_eq!(retrieved2.canister, dao2.canister);
    assert_eq!(retrieved3.canister, dao3.canister);
}

#[test]
fn test_dao_repository_get_all() {
    // Save some test DAOs
    let dao1 = create_test_dao(400);
    let dao2 = create_test_dao(500);
    let dao3 = create_test_dao(600);

    DaoRepository::save(dao1.clone());
    DaoRepository::save(dao2.clone());
    DaoRepository::save(dao3.clone());

    // Get all DAOs
    let all_daos = DaoRepository::get_all();

    // Verify all DAOs are returned (note: might contain DAOs from other tests)
    assert!(all_daos.len() >= 3);
    assert!(all_daos.iter().any(|d| d.id == 400));
    assert!(all_daos.iter().any(|d| d.id == 500));
    assert!(all_daos.iter().any(|d| d.id == 600));
}

#[test]
fn test_dao_repository_size() {
    // Get initial size
    let initial_size = DaoRepository::size();

    // Add some DAOs
    let dao1 = create_test_dao(700);
    let dao2 = create_test_dao(800);

    DaoRepository::save(dao1);
    let size_after_one = DaoRepository::size();
    assert_eq!(size_after_one, initial_size + 1);

    DaoRepository::save(dao2);
    let size_after_two = DaoRepository::size();
    assert_eq!(size_after_two, initial_size + 2);
}

#[test]
fn test_dao_repository_overwrite() {
    let original_dao = create_test_dao(900);

    // Save original DAO
    DaoRepository::save(original_dao.clone());

    // Create updated DAO with same ID but different canister
    let updated_dao = Dao::new(
        900,
        Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(),
    );

    // Save updated DAO (should overwrite)
    DaoRepository::save(updated_dao.clone());

    // Verify the DAO was updated, not duplicated
    let retrieved = DaoRepository::get(900).unwrap();
    assert_eq!(retrieved.id, 900);
    assert_eq!(retrieved.canister, updated_dao.canister);
    assert_ne!(retrieved.canister, original_dao.canister);
}

#[test]
fn test_dao_repository_workflow() {
    let dao_id = 1000;
    let dao = create_test_dao(dao_id);

    // Initial state: DAO doesn't exist
    assert!(DaoRepository::get(dao_id).is_none());

    // Save DAO
    let saved = DaoRepository::save(dao.clone());
    assert_eq!(saved.id, dao.id);

    // Verify it exists
    let retrieved = DaoRepository::get(dao_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, dao_id);

    // Verify it appears in get_all
    let all_daos = DaoRepository::get_all();
    assert!(all_daos.iter().any(|d| d.id == dao_id));

    // Verify size increased
    assert!(DaoRepository::size() > 0);
}

#[test]
fn test_dao_repository_edge_cases() {
    // Test with ID 0
    let dao_zero = create_test_dao(0);
    DaoRepository::save(dao_zero.clone());
    let retrieved_zero = DaoRepository::get(0);
    assert!(retrieved_zero.is_some());
    assert_eq!(retrieved_zero.unwrap().id, 0);

    // Test with maximum u32
    let dao_max = create_test_dao(u32::MAX);
    DaoRepository::save(dao_max.clone());
    let retrieved_max = DaoRepository::get(u32::MAX);
    assert!(retrieved_max.is_some());
    assert_eq!(retrieved_max.unwrap().id, u32::MAX);
}
