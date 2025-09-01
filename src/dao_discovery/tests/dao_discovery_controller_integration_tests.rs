#[cfg(test)]
mod dao_discovery_controller_integration_tests {
    use candid::{decode_one, encode_args, Principal};
    use pocket_ic::PocketIc;

    fn setup_canister() -> (PocketIc, Principal) {
        let pic = PocketIc::new();
        let canister_id = pic.create_canister();
        pic.add_cycles(canister_id, 2_000_000_000_000);

        // Load your compiled WASM - adjust path as needed
        let wasm = std::fs::read("../../target/wasm32-unknown-unknown/release/dao_discovery.wasm")
            .expect("Could not read WASM file");

        pic.install_canister(canister_id, wasm, vec![], None);

        (pic, canister_id)
    }

    fn create_test_principals() -> (Principal, Principal, Principal) {
        let user_id = Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap();
        let dao_id1 = Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap();
        let dao_id2 = Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap();
        (user_id, dao_id1, dao_id2)
    }

    #[test]
    fn test_save_user_dao() {
        let (pic, canister_id) = setup_canister();
        let (user_id, dao_id, _) = create_test_principals();

        // Test save_user_dao method
        let response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((user_id, dao_id)).unwrap(),
        );

        assert!(response.is_ok());

        let user_daos: Vec<Principal> = decode_one(&response.unwrap()).unwrap();

        // Verify the DAO was saved for the user
        assert_eq!(user_daos.len(), 1);
        assert_eq!(user_daos[0], dao_id);
    }

    #[test]
    fn test_get_user_daos_with_specific_user() {
        let (pic, canister_id) = setup_canister();
        let (user_id, dao_id, _) = create_test_principals();

        // First save a DAO for the user
        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((user_id, dao_id)).unwrap(),
        )
        .unwrap();

        // Test get_user_daos with specific user
        let response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_user_daos",
            encode_args((Some(user_id),)).unwrap(),
        );

        assert!(response.is_ok());

        let user_daos: Vec<Principal> = decode_one(&response.unwrap()).unwrap();
        assert_eq!(user_daos.len(), 1);
        assert_eq!(user_daos[0], dao_id);
    }

    #[test]
    fn test_get_user_daos_with_none() {
        let (pic, canister_id) = setup_canister();

        // Test get_user_daos with None (should use caller())
        let response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_user_daos",
            encode_args((None::<Principal>,)).unwrap(),
        );

        assert!(response.is_ok());

        let user_daos: Vec<Principal> = decode_one(&response.unwrap()).unwrap();
        // Should be empty since anonymous user has no DAOs
        assert_eq!(user_daos.len(), 0);
    }

    #[test]
    fn test_get_random_daos_with_default() {
        let (pic, canister_id) = setup_canister();
        let (_, dao_id1, dao_id2) = create_test_principals();

        // Save some DAOs first
        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((Principal::anonymous(), dao_id1)).unwrap(),
        )
        .unwrap();

        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((Principal::anonymous(), dao_id2)).unwrap(),
        )
        .unwrap();

        // Test get_random_daos with default amount (6)
        let response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_random_daos",
            encode_args((None::<u32>,)).unwrap(),
        );

        assert!(response.is_ok());

        let random_daos: Vec<Principal> = decode_one(&response.unwrap()).unwrap();

        // Should return the DAOs we saved (2 in this case, less than default 6)
        assert_eq!(random_daos.len(), 2);
        assert!(random_daos.contains(&dao_id1));
        assert!(random_daos.contains(&dao_id2));
    }

    #[test]
    fn test_get_random_daos_with_specific_amount() {
        let (pic, canister_id) = setup_canister();
        let (_, dao_id1, dao_id2) = create_test_principals();

        // Save some DAOs first
        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((Principal::anonymous(), dao_id1)).unwrap(),
        )
        .unwrap();

        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((Principal::anonymous(), dao_id2)).unwrap(),
        )
        .unwrap();

        // Test get_random_daos with specific amount
        let response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_random_daos",
            encode_args((Some(1u32),)).unwrap(),
        );

        assert!(response.is_ok());

        let random_daos: Vec<Principal> = decode_one(&response.unwrap()).unwrap();

        // Should return exactly 1 DAO
        assert_eq!(random_daos.len(), 1);
        assert!(random_daos[0] == dao_id1 || random_daos[0] == dao_id2);
    }

    #[test]
    fn test_remove_user_dao() {
        let (pic, canister_id) = setup_canister();
        let (user_id, dao_id1, dao_id2) = create_test_principals();

        // First save two DAOs for the user
        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((user_id, dao_id1)).unwrap(),
        )
        .unwrap();

        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((user_id, dao_id2)).unwrap(),
        )
        .unwrap();

        // Verify user has 2 DAOs
        let check_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_user_daos",
            encode_args((Some(user_id),)).unwrap(),
        );
        let user_daos: Vec<Principal> = decode_one(&check_response.unwrap()).unwrap();
        assert_eq!(user_daos.len(), 2);

        // Remove one DAO
        let response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "remove_user_dao",
            encode_args((user_id, dao_id1)).unwrap(),
        );

        assert!(response.is_ok());

        let remaining_daos: Vec<Principal> = decode_one(&response.unwrap()).unwrap();

        // Should have only 1 DAO remaining
        assert_eq!(remaining_daos.len(), 1);
        assert_eq!(remaining_daos[0], dao_id2);
        assert!(!remaining_daos.contains(&dao_id1));
    }

    #[test]
    fn test_save_user_dao_multiple_times() {
        let (pic, canister_id) = setup_canister();
        let (user_id, dao_id1, dao_id2) = create_test_principals();

        // Save first DAO
        let response1 = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((user_id, dao_id1)).unwrap(),
        );
        let user_daos1: Vec<Principal> = decode_one(&response1.unwrap()).unwrap();
        assert_eq!(user_daos1.len(), 1);

        // Save second DAO for same user
        let response2 = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((user_id, dao_id2)).unwrap(),
        );
        let user_daos2: Vec<Principal> = decode_one(&response2.unwrap()).unwrap();
        assert_eq!(user_daos2.len(), 2);
        assert!(user_daos2.contains(&dao_id1));
        assert!(user_daos2.contains(&dao_id2));
    }

    #[test]
    fn test_complete_workflow() {
        let (pic, canister_id) = setup_canister();
        let (user_id, dao_id1, dao_id2) = create_test_principals();

        // Step 1: Save DAOs for user
        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((user_id, dao_id1)).unwrap(),
        )
        .unwrap();

        pic.update_call(
            canister_id,
            Principal::anonymous(),
            "save_user_dao",
            encode_args((user_id, dao_id2)).unwrap(),
        )
        .unwrap();

        // Step 2: Verify user has DAOs
        let get_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_user_daos",
            encode_args((Some(user_id),)).unwrap(),
        );
        let user_daos: Vec<Principal> = decode_one(&get_response.unwrap()).unwrap();
        assert_eq!(user_daos.len(), 2);

        // Step 3: Check random DAOs include our saved ones
        let random_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_random_daos",
            encode_args((Some(2u32),)).unwrap(),
        );
        let random_daos: Vec<Principal> = decode_one(&random_response.unwrap()).unwrap();
        assert_eq!(random_daos.len(), 2);

        // Step 4: Remove one DAO
        let remove_response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "remove_user_dao",
            encode_args((user_id, dao_id1)).unwrap(),
        );
        let remaining_daos: Vec<Principal> = decode_one(&remove_response.unwrap()).unwrap();
        assert_eq!(remaining_daos.len(), 1);
        assert_eq!(remaining_daos[0], dao_id2);

        // Step 5: Final verification
        let final_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_user_daos",
            encode_args((Some(user_id),)).unwrap(),
        );
        let final_daos: Vec<Principal> = decode_one(&final_response.unwrap()).unwrap();
        assert_eq!(final_daos.len(), 1);
        assert_eq!(final_daos[0], dao_id2);
    }
}
