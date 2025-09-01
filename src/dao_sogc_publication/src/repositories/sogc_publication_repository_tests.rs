use super::*;
use crate::models::SogcPublication;
use common::types::Mutation;

fn create_test_sogc_publication(sogc_id: u32) -> SogcPublication {
    SogcPublication::new(
        sogc_id,
        1640995200000, // 2022-01-01
        1,
        1641081600000, // 2022-01-02
        vec![Mutation::ChangeOfCompany],
        format!("Test publication {}", sogc_id),
    )
}

#[test]
fn test_sogc_publication_repository_save_and_get() {
    let test_sogc = create_test_sogc_publication(123);

    // Test save
    let saved_sogc = SogcPubblicationRepository::save(test_sogc.clone());

    // Verify save returns the same publication
    assert_eq!(saved_sogc.sogc_id, test_sogc.sogc_id);
    assert_eq!(saved_sogc.description, test_sogc.description);

    // Test get
    let retrieved_sogc = SogcPubblicationRepository::get(123);

    // Verify get returns the saved publication
    assert!(retrieved_sogc.is_some());
    let retrieved = retrieved_sogc.unwrap();
    assert_eq!(retrieved.sogc_id, test_sogc.sogc_id);
    assert_eq!(retrieved.publication_date, test_sogc.publication_date);
    assert_eq!(retrieved.description, test_sogc.description);
}

#[test]
fn test_sogc_publication_repository_get_nonexistent() {
    // Test getting a publication that doesn't exist
    let result = SogcPubblicationRepository::get(99999);
    assert!(result.is_none());
}

#[test]
fn test_sogc_publication_repository_save_multiple() {
    let sogc1 = create_test_sogc_publication(200);
    let sogc2 = SogcPublication::new(
        201,
        1672531200000, // Different date
        2,
        1672617600000,
        vec![Mutation::ChangeOfAddress],
        "Different publication".to_string(),
    );

    // Save multiple publications
    SogcPubblicationRepository::save(sogc1.clone());
    SogcPubblicationRepository::save(sogc2.clone());

    // Verify both can be retrieved
    let retrieved1 = SogcPubblicationRepository::get(200).unwrap();
    let retrieved2 = SogcPubblicationRepository::get(201).unwrap();

    assert_eq!(retrieved1.sogc_id, sogc1.sogc_id);
    assert_eq!(retrieved2.sogc_id, sogc2.sogc_id);
    assert_eq!(retrieved1.description, sogc1.description);
    assert_eq!(retrieved2.description, sogc2.description);
}

#[test]
fn test_sogc_publication_repository_size() {
    // Get initial size
    let initial_size = SogcPubblicationRepository::size();

    // Add some publications
    let sogc1 = create_test_sogc_publication(300);
    let sogc2 = create_test_sogc_publication(301);

    SogcPubblicationRepository::save(sogc1);
    let size_after_one = SogcPubblicationRepository::size();
    assert_eq!(size_after_one, initial_size + 1);

    SogcPubblicationRepository::save(sogc2);
    let size_after_two = SogcPubblicationRepository::size();
    assert_eq!(size_after_two, initial_size + 2);
}

#[test]
fn test_sogc_publication_repository_overwrite() {
    let original_sogc = create_test_sogc_publication(400);

    // Save original publication
    SogcPubblicationRepository::save(original_sogc.clone());

    // Create updated publication with same ID but different content
    let updated_sogc = SogcPublication::new(
        400,
        1704067200000, // Different date
        5,
        1704153600000,
        vec![Mutation::ChangeOfAddress],
        "Updated publication".to_string(),
    );

    // Save updated publication (should overwrite)
    SogcPubblicationRepository::save(updated_sogc.clone());

    // Verify the publication was updated, not duplicated
    let retrieved = SogcPubblicationRepository::get(400).unwrap();
    assert_eq!(retrieved.sogc_id, 400);
    assert_eq!(retrieved.description, "Updated publication");
    assert_ne!(retrieved.description, original_sogc.description);
}

#[test]
fn test_sogc_publication_repository_workflow() {
    let sogc_id = 500;
    let sogc = create_test_sogc_publication(sogc_id);

    // Initial state: publication doesn't exist
    assert!(SogcPubblicationRepository::get(sogc_id).is_none());

    // Save publication
    let saved = SogcPubblicationRepository::save(sogc.clone());
    assert_eq!(saved.sogc_id, sogc.sogc_id);

    // Verify it exists
    let retrieved = SogcPubblicationRepository::get(sogc_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().sogc_id, sogc_id);

    // Verify size increased
    assert!(SogcPubblicationRepository::size() > 0);
}

#[test]
fn test_sogc_publication_repository_edge_cases() {
    // Test with ID 0
    let sogc_zero = create_test_sogc_publication(0);
    SogcPubblicationRepository::save(sogc_zero.clone());
    let retrieved_zero = SogcPubblicationRepository::get(0);
    assert!(retrieved_zero.is_some());
    assert_eq!(retrieved_zero.unwrap().sogc_id, 0);

    // Test with maximum u32
    let sogc_max = create_test_sogc_publication(u32::MAX);
    SogcPubblicationRepository::save(sogc_max.clone());
    let retrieved_max = SogcPubblicationRepository::get(u32::MAX);
    assert!(retrieved_max.is_some());
    assert_eq!(retrieved_max.unwrap().sogc_id, u32::MAX);
}
