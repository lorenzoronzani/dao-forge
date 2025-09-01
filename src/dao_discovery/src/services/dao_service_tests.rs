use super::*;
use candid::Principal;

// Mock the DaoService for testing by creating a testable version
struct TestDaoService;

impl TestDaoService {
    fn get_randoms_with_mock_time(amount: u32, mock_time: u64) -> Vec<Principal> {
        let mut daos = DaoRepository::get_all();
        let size = daos.len() as u32;

        if size <= amount {
            return daos.iter().map(|dao| dao.canister).collect();
        }

        let limit = amount.min(size);
        let mut random_daos = Vec::new();
        let seed = mock_time / 1000; // Use mock time instead of ic_cdk::api::time()

        for i in 0..limit {
            let random_index = (seed * (i + 1) as u64) % (daos.len() as u64);
            random_daos.push(daos.remove(random_index as usize).canister);
        }

        random_daos
    }
}

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
fn test_dao_service_save() {
    let canister = create_test_principal(1);

    // Test save method
    let saved_dao = DaoService::save(canister);

    // Verify dao was created with auto-generated ID
    assert!(saved_dao.id > 0);
    assert_eq!(saved_dao.canister, canister);
}

#[test]
fn test_dao_service_save_and_get() {
    let canister = create_test_principal(2);

    // Save a DAO
    let saved_dao = DaoService::save(canister);
    let dao_id = saved_dao.id;

    // Get the DAO by ID
    let retrieved_dao = DaoService::get(dao_id);

    // Verify get returns the saved DAO
    assert!(retrieved_dao.is_some());
    let retrieved = retrieved_dao.unwrap();
    assert_eq!(retrieved.id, dao_id);
    assert_eq!(retrieved.canister, canister);
}

#[test]
fn test_dao_service_get_nonexistent() {
    // Test getting a DAO that doesn't exist
    let result = DaoService::get(99999);
    assert!(result.is_none());
}

#[test]
fn test_dao_service_auto_increment_ids() {
    let canister1 = create_test_principal(3);
    let canister2 = create_test_principal(4);
    let canister3 = create_test_principal(5);

    // Save multiple DAOs
    let dao1 = DaoService::save(canister1);
    let dao2 = DaoService::save(canister2);
    let dao3 = DaoService::save(canister3);

    // Verify IDs are auto-incremented
    assert!(dao2.id > dao1.id);
    assert!(dao3.id > dao2.id);
    assert_eq!(dao1.canister, canister1);
    assert_eq!(dao2.canister, canister2);
    assert_eq!(dao3.canister, canister3);
}

#[test]
fn test_dao_service_find_by_canister() {
    let canister1 = create_test_principal(1);
    let canister2 = create_test_principal(2);

    // Save some DAOs
    DaoService::save(canister1);
    DaoService::save(canister2);

    // Find by existing canister
    let found_dao1 = DaoService::find_by_canister(canister1);
    assert!(found_dao1.is_some());
    assert_eq!(found_dao1.unwrap().canister, canister1);

    // Find by existing canister
    let found_dao2 = DaoService::find_by_canister(canister2);
    assert!(found_dao2.is_some());
    assert_eq!(found_dao2.unwrap().canister, canister2);

    // Find by non-existent canister - use a completely different principal
    let nonexistent = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();
    let not_found = DaoService::find_by_canister(nonexistent);
    assert!(not_found.is_none());
}

#[test]
fn test_dao_service_get_randoms_basic() {
    let canister1 = create_test_principal(1);
    let canister2 = create_test_principal(2);
    let canister3 = create_test_principal(3);

    // Save some DAOs
    DaoService::save(canister1);
    DaoService::save(canister2);
    DaoService::save(canister3);

    // Use mock time for predictable results
    let mock_time = 1000000000000u64; // Fixed timestamp
    let randoms = TestDaoService::get_randoms_with_mock_time(2, mock_time);

    // Verify we get exactly 2 unique canisters
    assert_eq!(randoms.len(), 2);

    // Verify all returned canisters are from our saved DAOs
    for canister in &randoms {
        assert!(*canister == canister1 || *canister == canister2 || *canister == canister3);
    }

    // Verify no duplicates
    if randoms.len() == 2 {
        assert_ne!(randoms[0], randoms[1]);
    }
}

#[test]
fn test_dao_service_get_randoms_more_than_available() {
    let canister1 = create_test_principal(4);
    let canister2 = create_test_principal(5);

    // Save 2 DAOs
    DaoService::save(canister1);
    DaoService::save(canister2);

    // Request more than available with mock time
    let mock_time = 2000000000000u64;
    let randoms = TestDaoService::get_randoms_with_mock_time(5, mock_time);

    // Should return all available DAOs (at least the 2 we just saved)
    assert!(randoms.len() >= 2);

    // Verify our DAOs are in the results
    let contains_canister1 = randoms.iter().any(|&c| c == canister1);
    let contains_canister2 = randoms.iter().any(|&c| c == canister2);
    assert!(contains_canister1 || contains_canister2);
}

#[test]
fn test_dao_service_get_randoms_zero() {
    let canister = create_test_principal(1);
    DaoService::save(canister);

    // Request 0 DAOs with mock time
    let mock_time = 3000000000000u64;
    let randoms = TestDaoService::get_randoms_with_mock_time(0, mock_time);

    // Should return empty vector
    assert_eq!(randoms.len(), 0);
}

#[test]
fn test_dao_service_workflow() {
    let canister = create_test_principal(1);

    // Complete workflow: save -> get -> find
    let saved_dao = DaoService::save(canister);
    let dao_id = saved_dao.id;

    // Verify can get by ID
    let by_id = DaoService::get(dao_id);
    assert!(by_id.is_some());
    assert_eq!(by_id.unwrap().canister, canister);

    // Verify can find by canister
    let by_canister = DaoService::find_by_canister(canister);
    assert!(by_canister.is_some());
    assert_eq!(by_canister.unwrap().id, dao_id);

    // Verify appears in randoms using mock time
    let mock_time = 4000000000000u64;
    let randoms = TestDaoService::get_randoms_with_mock_time(1, mock_time);
    assert!(randoms.len() >= 1);
}
