use super::*;
use common::types::Mutation;

#[test]
fn test_sogc_publication_service_save() {
    let publication_sogc_date = 1640995200000; // 2022-01-01
    let daily_number = 5;
    let publication_date = 1641081600000; // 2022-01-02
    let mutations = vec![Mutation::ChangeOfCompany];
    let description = "Test publication".to_string();

    // Test save method
    let saved_sogc = SogcPubblicationService::save(
        publication_sogc_date,
        daily_number,
        publication_date,
        mutations.clone(),
        description.clone(),
    );

    // Verify publication was created with auto-generated ID
    assert!(saved_sogc.sogc_id > 0);
    assert_eq!(saved_sogc.publication_sogc_date, publication_sogc_date);
    assert_eq!(saved_sogc.daily_number, daily_number);
    assert_eq!(saved_sogc.publication_date, publication_date);
    assert_eq!(saved_sogc.mutations, mutations);
    assert_eq!(saved_sogc.description, description);
}

#[test]
fn test_sogc_publication_service_save_and_get() {
    let publication_sogc_date = 1672531200000; // 2023-01-01
    let daily_number = 10;
    let publication_date = 1672617600000; // 2023-01-02
    let mutations = vec![Mutation::ChangeOfAddress];
    let description = "Save and get test".to_string();

    // Save a publication
    let saved_sogc = SogcPubblicationService::save(
        publication_sogc_date,
        daily_number,
        publication_date,
        mutations.clone(),
        description.clone(),
    );
    let sogc_id = saved_sogc.sogc_id;

    // Get the publication by ID
    let retrieved_sogc = SogcPubblicationService::get(sogc_id);

    // Verify get returns the saved publication
    assert!(retrieved_sogc.is_some());
    let retrieved = retrieved_sogc.unwrap();
    assert_eq!(retrieved.sogc_id, sogc_id);
    assert_eq!(retrieved.publication_sogc_date, publication_sogc_date);
    assert_eq!(retrieved.daily_number, daily_number);
    assert_eq!(retrieved.description, description);
}

#[test]
fn test_sogc_publication_service_get_nonexistent() {
    // Test getting a publication that doesn't exist
    let result = SogcPubblicationService::get(99999);
    assert!(result.is_none());
}

#[test]
fn test_sogc_publication_service_auto_increment_ids() {
    let mutations = vec![Mutation::ChangeOfCompany];

    // Save multiple publications
    let sogc1 = SogcPubblicationService::save(
        1640995200000,
        1,
        1641081600000,
        mutations.clone(),
        "First publication".to_string(),
    );

    let sogc2 = SogcPubblicationService::save(
        1672531200000,
        2,
        1672617600000,
        mutations.clone(),
        "Second publication".to_string(),
    );

    let sogc3 = SogcPubblicationService::save(
        1704067200000,
        3,
        1704153600000,
        mutations,
        "Third publication".to_string(),
    );

    // Verify IDs are auto-incremented
    assert!(sogc2.sogc_id > sogc1.sogc_id);
    assert!(sogc3.sogc_id > sogc2.sogc_id);
    assert_eq!(sogc1.description, "First publication");
    assert_eq!(sogc2.description, "Second publication");
    assert_eq!(sogc3.description, "Third publication");
}

#[test]
fn test_sogc_publication_service_empty_mutations() {
    let sogc = SogcPubblicationService::save(
        1640995200000,
        7,
        1641081600000,
        vec![], // Empty mutations
        "No mutations publication".to_string(),
    );

    assert!(sogc.mutations.is_empty());
    assert_eq!(sogc.description, "No mutations publication");
}

#[test]
fn test_sogc_publication_service_multiple_mutations() {
    let mutations = vec![Mutation::ChangeOfCompany, Mutation::ChangeOfAddress];

    let sogc = SogcPubblicationService::save(
        1672531200000,
        8,
        1672617600000,
        mutations.clone(),
        "Multiple mutations test".to_string(),
    );

    assert_eq!(sogc.mutations.len(), 2);
    assert!(sogc.mutations.contains(&Mutation::ChangeOfCompany));
    assert!(sogc.mutations.contains(&Mutation::ChangeOfAddress));
}

#[test]
fn test_sogc_publication_service_workflow() {
    let mutations = vec![Mutation::ChangeOfCompany];
    let description = "Workflow test".to_string();

    // Complete workflow: save -> get
    let saved_sogc = SogcPubblicationService::save(
        1640995200000,
        15,
        1641081600000,
        mutations.clone(),
        description.clone(),
    );
    let sogc_id = saved_sogc.sogc_id;

    // Verify can get by ID
    let by_id = SogcPubblicationService::get(sogc_id);
    assert!(by_id.is_some());
    assert_eq!(by_id.unwrap().description, description);

    // Verify auto-generated ID is positive
    assert!(sogc_id > 0);
}

#[test]
fn test_sogc_publication_service_edge_values() {
    // Test with zero values (except ID which is auto-generated)
    let sogc_zero = SogcPubblicationService::save(
        0, // Zero timestamp
        0, // Zero daily number
        0, // Zero publication date
        vec![],
        String::new(), // Empty description
    );

    assert!(sogc_zero.sogc_id > 0); // ID should still be auto-generated
    assert_eq!(sogc_zero.publication_sogc_date, 0);
    assert_eq!(sogc_zero.daily_number, 0);
    assert_eq!(sogc_zero.publication_date, 0);
    assert!(sogc_zero.mutations.is_empty());
    assert!(sogc_zero.description.is_empty());

    // Test with maximum values
    let sogc_max = SogcPubblicationService::save(
        u64::MAX,
        u32::MAX,
        u64::MAX,
        vec![Mutation::ChangeOfCompany],
        "Maximum values test".to_string(),
    );

    assert_eq!(sogc_max.publication_sogc_date, u64::MAX);
    assert_eq!(sogc_max.daily_number, u32::MAX);
    assert_eq!(sogc_max.publication_date, u64::MAX);
}
