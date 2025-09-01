use super::*;

use candid::Principal;
use common::models::Configuration;

fn create_test_configuration() -> Configuration {
    use candid::Principal;

    Configuration::new(
        Some(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()),
        Some(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()),
        Some(Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap()),
        Some(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()),
        Some(Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap()),
        Some(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()),
        Some(Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap()),
    )
}

#[test]
fn test_configuration_save_and_get() {
    let repo = ConfigurationRepository::new();
    let test_config = create_test_configuration();

    // Test save
    let saved_config = repo.save(test_config.clone());

    // Verify save returns the same configuration
    assert_eq!(
        saved_config.dao_agency_canister_id,
        test_config.dao_agency_canister_id
    );
    assert_eq!(
        saved_config.sogc_publication_canister_id,
        test_config.sogc_publication_canister_id
    );
    assert_eq!(
        saved_config.dao_discovery_canister_id,
        test_config.dao_discovery_canister_id
    );

    // Test get
    let retrieved_config = repo.get();

    // Verify get returns the saved configuration
    assert_eq!(
        retrieved_config.dao_agency_canister_id,
        test_config.dao_agency_canister_id
    );
    assert_eq!(
        retrieved_config.voting_canister_id,
        test_config.voting_canister_id
    );
    assert_eq!(
        retrieved_config.dao_platform_canister_id,
        test_config.dao_platform_canister_id
    );
}

#[test]
fn test_configuration_overwrite() {
    let repo = ConfigurationRepository::new();

    // Save first configuration
    let first_config = create_test_configuration();
    repo.save(first_config.clone());

    // Create and save second configuration with different values
    let second_config = Configuration::new(
        None, // Different from first_config
        Some(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()),
        None,
        Some(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()),
        None,
        Some(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()),
        None,
    );
    repo.save(second_config.clone());

    // Verify the second configuration overwrote the first
    let retrieved_config = repo.get();
    assert_eq!(retrieved_config.dao_agency_canister_id, None);
    assert_eq!(
        retrieved_config.sogc_publication_canister_id,
        second_config.sogc_publication_canister_id
    );
    assert_ne!(
        retrieved_config.dao_agency_canister_id,
        first_config.dao_agency_canister_id
    );
}

#[test]
fn test_configuration_get_default() {
    let repo = ConfigurationRepository::new();

    // Test getting configuration when none has been saved
    // This should return the default Configuration (all None values)
    let config = repo.get();

    // Verify it returns a valid Configuration with default values (all None)
    assert_eq!(config.dao_agency_canister_id, None);
    assert_eq!(config.sogc_publication_canister_id, None);
    assert_eq!(config.dao_discovery_canister_id, None);
    assert_eq!(config.documents_storage_canister_id, None);
    assert_eq!(config.voting_canister_id, None);
    assert_eq!(config.network_call_canister_id, None);
    assert_eq!(config.dao_platform_canister_id, None);
}

#[test]
fn test_configuration_partial_values() {
    let repo = ConfigurationRepository::new();

    // Test with some None and some Some values
    let partial_config = Configuration::new(
        Some(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()),
        None,
        Some(Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap()),
        None,
        Some(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()),
        None,
        None,
    );

    repo.save(partial_config.clone());
    let retrieved = repo.get();

    assert_eq!(
        retrieved.dao_agency_canister_id,
        partial_config.dao_agency_canister_id
    );
    assert_eq!(retrieved.sogc_publication_canister_id, None);
    assert_eq!(
        retrieved.dao_discovery_canister_id,
        partial_config.dao_discovery_canister_id
    );
    assert_eq!(
        retrieved.voting_canister_id,
        partial_config.voting_canister_id
    );
    assert_eq!(retrieved.dao_platform_canister_id, None);
}
