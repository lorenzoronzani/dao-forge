use super::*;
use common::types::Mutation;

fn create_test_sogc_publication() -> SogcPublication {
    SogcPublication::new(
        123,
        1640995200000, // 2022-01-01 timestamp in milliseconds
        1,
        1641081600000, // 2022-01-02 timestamp in milliseconds
        vec![Mutation::ChangeOfCompany, Mutation::ChangeOfAddress],
        "Test SOGC publication description".to_string(),
    )
}

#[test]
fn test_sogc_publication_creation() {
    let sogc = create_test_sogc_publication();

    // Test basic properties
    assert_eq!(sogc.sogc_id, 123);
    assert_eq!(sogc.publication_sogc_date, 1640995200000);
    assert_eq!(sogc.daily_number, 1);
    assert_eq!(sogc.publication_date, 1641081600000);
    assert_eq!(sogc.mutations.len(), 2);
    assert_eq!(sogc.description, "Test SOGC publication description");

    // Test mutations
    assert!(sogc.mutations.contains(&Mutation::ChangeOfCompany));
    assert!(sogc.mutations.contains(&Mutation::ChangeOfAddress));
}

#[test]
fn test_sogc_publication_new() {
    let sogc_id = 456;
    let pub_sogc_date = 1672531200000; // 2023-01-01
    let daily_num = 5;
    let pub_date = 1672617600000; // 2023-01-02
    let mutations = vec![Mutation::ChangeOfAddress];
    let description = "Custom publication".to_string();

    let sogc = SogcPublication::new(
        sogc_id,
        pub_sogc_date,
        daily_num,
        pub_date,
        mutations.clone(),
        description.clone(),
    );

    assert_eq!(sogc.sogc_id, sogc_id);
    assert_eq!(sogc.publication_sogc_date, pub_sogc_date);
    assert_eq!(sogc.daily_number, daily_num);
    assert_eq!(sogc.publication_date, pub_date);
    assert_eq!(sogc.mutations, mutations);
    assert_eq!(sogc.description, description);
}

#[test]
fn test_sogc_publication_storable() {
    let original_sogc = create_test_sogc_publication();

    // Test serialization
    let bytes = original_sogc.to_bytes();
    assert!(!bytes.is_empty());

    // Test deserialization
    let deserialized_sogc = SogcPublication::from_bytes(bytes);

    // Verify data integrity
    assert_eq!(original_sogc.sogc_id, deserialized_sogc.sogc_id);
    assert_eq!(
        original_sogc.publication_sogc_date,
        deserialized_sogc.publication_sogc_date
    );
    assert_eq!(original_sogc.daily_number, deserialized_sogc.daily_number);
    assert_eq!(
        original_sogc.publication_date,
        deserialized_sogc.publication_date
    );
    assert_eq!(original_sogc.mutations, deserialized_sogc.mutations);
    assert_eq!(original_sogc.description, deserialized_sogc.description);
}

#[test]
fn test_sogc_publication_clone() {
    let original_sogc = create_test_sogc_publication();
    let cloned_sogc = original_sogc.clone();

    // Verify clone works correctly
    assert_eq!(original_sogc.sogc_id, cloned_sogc.sogc_id);
    assert_eq!(
        original_sogc.publication_sogc_date,
        cloned_sogc.publication_sogc_date
    );
    assert_eq!(original_sogc.daily_number, cloned_sogc.daily_number);
    assert_eq!(original_sogc.publication_date, cloned_sogc.publication_date);
    assert_eq!(original_sogc.mutations, cloned_sogc.mutations);
    assert_eq!(original_sogc.description, cloned_sogc.description);
}

#[test]
fn test_sogc_publication_empty_mutations() {
    let sogc = SogcPublication::new(
        789,
        1672531200000,
        2,
        1672617600000,
        vec![], // Empty mutations
        "No mutations publication".to_string(),
    );

    assert_eq!(sogc.sogc_id, 789);
    assert!(sogc.mutations.is_empty());
    assert_eq!(sogc.description, "No mutations publication");
}

#[test]
fn test_sogc_publication_single_mutation() {
    let sogc = SogcPublication::new(
        101112,
        1672531200000,
        3,
        1672617600000,
        vec![Mutation::ChangeOfCompany],
        "Single mutation test".to_string(),
    );

    assert_eq!(sogc.mutations.len(), 1);
    assert_eq!(sogc.mutations[0], Mutation::ChangeOfCompany);
}

#[test]
fn test_sogc_publication_multiple_mutations() {
    let mutations = vec![
        Mutation::ChangeOfCompany,
        Mutation::ChangeOfAddress,
        // Add more mutation types as they exist in your Mutation enum
    ];

    let sogc = SogcPublication::new(
        131415,
        1672531200000,
        4,
        1672617600000,
        mutations.clone(),
        "Multiple mutations test".to_string(),
    );

    assert_eq!(sogc.mutations.len(), mutations.len());
    for mutation in &mutations {
        assert!(sogc.mutations.contains(mutation));
    }
}

#[test]
fn test_sogc_publication_empty_description() {
    let sogc = SogcPublication::new(
        161718,
        1672531200000,
        5,
        1672617600000,
        vec![Mutation::ChangeOfAddress],
        String::new(), // Empty description
    );

    assert_eq!(sogc.sogc_id, 161718);
    assert!(sogc.description.is_empty());
}

#[test]
fn test_sogc_publication_edge_cases() {
    // Test with ID 0
    let sogc_zero = SogcPublication::new(0, 0, 0, 0, vec![], "Zero values test".to_string());

    assert_eq!(sogc_zero.sogc_id, 0);
    assert_eq!(sogc_zero.publication_sogc_date, 0);
    assert_eq!(sogc_zero.daily_number, 0);
    assert_eq!(sogc_zero.publication_date, 0);

    // Test with maximum values
    let sogc_max = SogcPublication::new(
        u32::MAX,
        u64::MAX,
        u32::MAX,
        u64::MAX,
        vec![Mutation::ChangeOfCompany],
        "Maximum values test".to_string(),
    );

    assert_eq!(sogc_max.sogc_id, u32::MAX);
    assert_eq!(sogc_max.publication_sogc_date, u64::MAX);
    assert_eq!(sogc_max.daily_number, u32::MAX);
    assert_eq!(sogc_max.publication_date, u64::MAX);
}

#[test]
fn test_sogc_publication_serialization_with_complex_data() {
    let complex_sogc = SogcPublication::new(
            999999,
            1672531200000,
            100,
            1672617600000,
            vec![
                Mutation::ChangeOfCompany,
                Mutation::ChangeOfAddress,
                Mutation::ChangeOfCompany, // Duplicate mutation
            ],
            "Complex publication with very long description that contains special characters: àáâãäåæçèéêëìíîïðñòóôõöøùúûüýþÿ and numbers 1234567890 and symbols !@#$%^&*()".to_string(),
        );

    // Test that complex data serializes and deserializes correctly
    let bytes = complex_sogc.to_bytes();
    let deserialized = SogcPublication::from_bytes(bytes);

    assert_eq!(complex_sogc.sogc_id, deserialized.sogc_id);
    assert_eq!(complex_sogc.mutations, deserialized.mutations);
    assert_eq!(complex_sogc.description, deserialized.description);

    // Verify mutations include duplicates
    assert_eq!(deserialized.mutations.len(), 3);
}
