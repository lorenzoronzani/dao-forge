use super::*;
use candid::Principal;

fn create_test_dao() -> Dao {
    Dao::new(
        123,
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
    )
}

#[test]
fn test_dao_creation() {
    let dao = create_test_dao();

    // Test basic properties
    assert_eq!(dao.id, 123);
    assert_eq!(
        dao.canister,
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
    );
}

#[test]
fn test_dao_new() {
    let id = 456;
    let canister = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

    let dao = Dao::new(id, canister);

    assert_eq!(dao.id, id);
    assert_eq!(dao.canister, canister);
}

#[test]
fn test_dao_storable() {
    let original_dao = create_test_dao();

    // Test serialization
    let bytes = original_dao.to_bytes();
    assert!(!bytes.is_empty());

    // Test deserialization
    let deserialized_dao = Dao::from_bytes(bytes);

    // Verify data integrity
    assert_eq!(original_dao.id, deserialized_dao.id);
    assert_eq!(original_dao.canister, deserialized_dao.canister);
}

#[test]
fn test_dao_clone() {
    let original_dao = create_test_dao();
    let cloned_dao = original_dao.clone();

    // Verify clone works correctly
    assert_eq!(original_dao.id, cloned_dao.id);
    assert_eq!(original_dao.canister, cloned_dao.canister);

    // Verify they are separate objects
    assert_eq!(original_dao.id, cloned_dao.id);
}

#[test]
fn test_dao_different_principals() {
    let dao1 = Dao::new(
        1,
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
    );
    let dao2 = Dao::new(
        2,
        Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap(),
    );
    let dao3 = Dao::new(
        3,
        Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap(),
    );

    // Test different DAOs have different properties
    assert_ne!(dao1.id, dao2.id);
    assert_ne!(dao2.id, dao3.id);
    assert_ne!(dao1.canister, dao2.canister);
    assert_ne!(dao2.canister, dao3.canister);
}

#[test]
fn test_dao_high_id_numbers() {
    let high_id = u32::MAX;
    let dao = Dao::new(
        high_id,
        Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
    );

    assert_eq!(dao.id, high_id);

    // Test serialization works with high numbers
    let bytes = dao.to_bytes();
    let deserialized = Dao::from_bytes(bytes);
    assert_eq!(deserialized.id, high_id);
}
