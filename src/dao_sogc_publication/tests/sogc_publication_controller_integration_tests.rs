#[cfg(test)]
mod sogc_publication_controller_integration_tests {
    use candid::{decode_one, encode_args, Principal};
    use common::types::{Mutation, SogcPublicationArgs};
    use dao_sogc_publication::SogcPublication;
    use pocket_ic::PocketIc;

    fn setup_canister() -> (PocketIc, Principal) {
        let pic = PocketIc::new();
        let canister_id = pic.create_canister();
        pic.add_cycles(canister_id, 2_000_000_000_000);

        // Load your compiled WASM - adjust path as needed
        let wasm =
            std::fs::read("../../target/wasm32-unknown-unknown/release/dao_sogc_publication.wasm")
                .expect("Could not read WASM file");

        pic.install_canister(canister_id, wasm, vec![], None);

        (pic, canister_id)
    }

    fn create_test_sogc_args() -> SogcPublicationArgs {
        SogcPublicationArgs {
            daily_number: 5,
            publication_date: 1641081600000, // 2022-01-02
            mutations: vec![Mutation::ChangeOfCompany],
            description: "Test SOGC publication".to_string(),
        }
    }

    #[test]
    fn test_create_sogc_publication() {
        let (pic, canister_id) = setup_canister();
        let sogc_args = create_test_sogc_args();

        // Test create_sogc_publication method
        let response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "create_sogc_publication",
            encode_args((sogc_args.clone(),)).unwrap(),
        );

        assert!(response.is_ok());

        let sogc_id: u32 = decode_one(&response.unwrap()).unwrap();

        // Verify a valid ID was returned
        assert!(sogc_id > 0);
    }

    #[test]
    fn test_create_and_get_sogc_publication() {
        let (pic, canister_id) = setup_canister();
        let sogc_args = create_test_sogc_args();

        // Create a publication
        let create_response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "create_sogc_publication",
            encode_args((sogc_args.clone(),)).unwrap(),
        );

        let sogc_id: u32 = decode_one(&create_response.unwrap()).unwrap();

        // Get the created publication
        let get_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_sogc_publication",
            encode_args((sogc_id,)).unwrap(),
        );

        assert!(get_response.is_ok());

        let retrieved_publication: SogcPublication = decode_one(&get_response.unwrap()).unwrap();

        // Verify the publication data
        assert_eq!(retrieved_publication.sogc_id, sogc_id);
        assert_eq!(retrieved_publication.daily_number, sogc_args.daily_number);
        assert_eq!(
            retrieved_publication.publication_date,
            sogc_args.publication_date
        );
        assert_eq!(retrieved_publication.mutations, sogc_args.mutations);
        assert_eq!(retrieved_publication.description, sogc_args.description);

        // Verify publication_sogc_date was set (should be current time)
        assert!(retrieved_publication.publication_sogc_date > 0);
    }

    #[test]
    fn test_create_multiple_sogc_publications() {
        let (pic, canister_id) = setup_canister();

        // Create first publication
        let sogc_args1 = SogcPublicationArgs {
            daily_number: 1,
            publication_date: 1640995200000, // 2022-01-01
            mutations: vec![Mutation::ChangeOfCompany],
            description: "First publication".to_string(),
        };

        let response1 = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "create_sogc_publication",
            encode_args((sogc_args1,)).unwrap(),
        );
        let sogc_id1: u32 = decode_one(&response1.unwrap()).unwrap();

        // Create second publication
        let sogc_args2 = SogcPublicationArgs {
            daily_number: 2,
            publication_date: 1641081600000, // 2022-01-02
            mutations: vec![Mutation::ChangeOfAddress],
            description: "Second publication".to_string(),
        };

        let response2 = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "create_sogc_publication",
            encode_args((sogc_args2,)).unwrap(),
        );
        let sogc_id2: u32 = decode_one(&response2.unwrap()).unwrap();

        // Verify different IDs
        assert_ne!(sogc_id1, sogc_id2);
        assert!(sogc_id2 > sogc_id1); // Auto-increment behavior

        // Verify both can be retrieved
        let get_response1 = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_sogc_publication",
            encode_args((sogc_id1,)).unwrap(),
        );
        let publication1: SogcPublication = decode_one(&get_response1.unwrap()).unwrap();
        assert_eq!(publication1.description, "First publication");

        let get_response2 = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_sogc_publication",
            encode_args((sogc_id2,)).unwrap(),
        );
        let publication2: SogcPublication = decode_one(&get_response2.unwrap()).unwrap();
        assert_eq!(publication2.description, "Second publication");
    }

    #[test]
    fn test_get_nonexistent_sogc_publication() {
        let (pic, canister_id) = setup_canister();

        // Try to get a publication that doesn't exist
        let response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_sogc_publication",
            encode_args((99999u32,)).unwrap(),
        );

        // Should fail because publication doesn't exist (expect panics)
        assert!(response.is_err());
    }

    #[test]
    fn test_create_sogc_publication_with_empty_mutations() {
        let (pic, canister_id) = setup_canister();

        let sogc_args = SogcPublicationArgs {
            daily_number: 10,
            publication_date: 1672531200000, // 2023-01-01
            mutations: vec![],               // Empty mutations
            description: "Publication with no mutations".to_string(),
        };

        // Create publication
        let create_response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "create_sogc_publication",
            encode_args((sogc_args,)).unwrap(),
        );

        let sogc_id: u32 = decode_one(&create_response.unwrap()).unwrap();

        // Get and verify
        let get_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_sogc_publication",
            encode_args((sogc_id,)).unwrap(),
        );

        let publication: SogcPublication = decode_one(&get_response.unwrap()).unwrap();
        assert!(publication.mutations.is_empty());
        assert_eq!(publication.description, "Publication with no mutations");
    }

    #[test]
    fn test_create_sogc_publication_with_multiple_mutations() {
        let (pic, canister_id) = setup_canister();

        let sogc_args = SogcPublicationArgs {
            daily_number: 15,
            publication_date: 1672617600000, // 2023-01-02
            mutations: vec![Mutation::ChangeOfCompany, Mutation::ChangeOfAddress],
            description: "Publication with multiple mutations".to_string(),
        };

        // Create publication
        let create_response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "create_sogc_publication",
            encode_args((sogc_args.clone(),)).unwrap(),
        );

        let sogc_id: u32 = decode_one(&create_response.unwrap()).unwrap();

        // Get and verify
        let get_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_sogc_publication",
            encode_args((sogc_id,)).unwrap(),
        );

        let publication: SogcPublication = decode_one(&get_response.unwrap()).unwrap();
        assert_eq!(publication.mutations.len(), 2);
        assert!(publication.mutations.contains(&Mutation::ChangeOfCompany));
        assert!(publication.mutations.contains(&Mutation::ChangeOfAddress));
    }

    #[test]
    fn test_complete_workflow() {
        let (pic, canister_id) = setup_canister();

        // Step 1: Create first publication
        let sogc_args1 = SogcPublicationArgs {
            daily_number: 100,
            publication_date: 1640995200000,
            mutations: vec![Mutation::ChangeOfCompany],
            description: "Workflow test publication 1".to_string(),
        };

        let response1 = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "create_sogc_publication",
            encode_args((sogc_args1.clone(),)).unwrap(),
        );
        let sogc_id1: u32 = decode_one(&response1.unwrap()).unwrap();

        // Step 2: Create second publication
        let sogc_args2 = SogcPublicationArgs {
            daily_number: 101,
            publication_date: 1641081600000,
            mutations: vec![Mutation::ChangeOfAddress],
            description: "Workflow test publication 2".to_string(),
        };

        let response2 = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "create_sogc_publication",
            encode_args((sogc_args2.clone(),)).unwrap(),
        );
        let sogc_id2: u32 = decode_one(&response2.unwrap()).unwrap();

        // Step 3: Verify both publications exist and have correct data
        let get1_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_sogc_publication",
            encode_args((sogc_id1,)).unwrap(),
        );
        let publication1: SogcPublication = decode_one(&get1_response.unwrap()).unwrap();

        let get2_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_sogc_publication",
            encode_args((sogc_id2,)).unwrap(),
        );
        let publication2: SogcPublication = decode_one(&get2_response.unwrap()).unwrap();

        // Final verifications
        assert_eq!(publication1.daily_number, 100);
        assert_eq!(publication2.daily_number, 101);
        assert_eq!(publication1.description, "Workflow test publication 1");
        assert_eq!(publication2.description, "Workflow test publication 2");
        assert!(publication2.sogc_id > publication1.sogc_id); // Auto-increment
    }
}
