use super::*;
use candid::Principal;
use std::collections::BTreeMap;

fn create_test_voting() -> Voting {
    let mut options = BTreeMap::new();
    options.insert("Option A".to_string(), None);
    options.insert(
        "Option B".to_string(),
        Some(Action {
            canister_id: Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
            method: "execute_action".to_string(),
            args: vec!["arg1".to_string()],
        }),
    );

    let voters_whitelist = vec![
        Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
        Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
    ];

    Voting::new(
        123,
        "Test Voting".to_string(),
        "A test voting proposal".to_string(),
        options,
        Principal::from_text("ggppn-oqaaa-aaaaa-aaaaq-azy").unwrap(),
        1672531200000,
        1672617600000,
        VotingState::Open,
        Principal::from_text("ueq6w-kyaaa-aaaaa-aaaaq-a2a").unwrap(),
        1672531200000,
        60,
        50,
        voters_whitelist,
        None,
    )
}

#[test]
fn test_voting_creation() {
    let voting = create_test_voting();

    // Test basic properties
    assert_eq!(voting.id, 123);
    assert_eq!(voting.title, "Test Voting");
    assert_eq!(voting.description, "A test voting proposal");
    assert_eq!(voting.state, VotingState::Open);
    assert_eq!(voting.approval_threshold, 60);
    assert_eq!(voting.quorum, 50);

    // Test collections are initialized properly
    assert_eq!(voting.options.len(), 2);
    assert!(voting.result.is_empty());
    assert!(voting.voters_cast.is_empty());
    assert_eq!(voting.voters_whitelist.len(), 2);
}

#[test]
fn test_voting_new_constructor() {
    let mut options = BTreeMap::new();
    options.insert("Yes".to_string(), None);
    options.insert("No".to_string(), None);

    let owner = Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap();
    let dao_id = Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap();
    let voters = vec![owner];

    let voting = Voting::new(
        456,
        "Simple Vote".to_string(),
        "Yes or No decision".to_string(),
        options.clone(),
        owner,
        1640995200000,
        1641081600000,
        VotingState::Closed,
        dao_id,
        1640995200000,
        75,
        25,
        voters.clone(),
        None,
    );

    assert_eq!(voting.id, 456);
    assert_eq!(voting.title, "Simple Vote");
    assert_eq!(voting.options, options);
    assert_eq!(voting.owner, owner);
    assert_eq!(voting.dao_id, dao_id);
    assert_eq!(voting.state, VotingState::Closed);
    assert_eq!(voting.approval_threshold, 75);
    assert_eq!(voting.quorum, 25);
    assert_eq!(voting.voters_whitelist, voters);
}

#[test]
fn test_voting_state_enum() {
    let open_state = VotingState::Open;
    let closed_state = VotingState::Closed;

    // Test PartialEq
    assert_eq!(open_state, VotingState::Open);
    assert_eq!(closed_state, VotingState::Closed);
    assert_ne!(open_state, closed_state);

    // Test Clone
    let cloned_open = open_state.clone();
    assert_eq!(open_state, cloned_open);
}

#[test]
fn test_voting_storable() {
    let original_voting = create_test_voting();

    // Test serialization
    let bytes = original_voting.to_bytes();
    assert!(!bytes.is_empty());

    // Test deserialization
    let deserialized_voting = Voting::from_bytes(bytes);

    // Verify data integrity
    assert_eq!(original_voting.id, deserialized_voting.id);
    assert_eq!(original_voting.title, deserialized_voting.title);
    assert_eq!(original_voting.description, deserialized_voting.description);
    assert_eq!(original_voting.options, deserialized_voting.options);
    assert_eq!(original_voting.owner, deserialized_voting.owner);
    assert_eq!(original_voting.state, deserialized_voting.state);
    assert_eq!(
        original_voting.approval_threshold,
        deserialized_voting.approval_threshold
    );
    assert_eq!(
        original_voting.voters_whitelist,
        deserialized_voting.voters_whitelist
    );
}

#[test]
fn test_voting_clone() {
    let original_voting = create_test_voting();
    let cloned_voting = original_voting.clone();

    // Verify clone works correctly
    assert_eq!(original_voting.id, cloned_voting.id);
    assert_eq!(original_voting.title, cloned_voting.title);
    assert_eq!(original_voting.options, cloned_voting.options);
    assert_eq!(
        original_voting.voters_whitelist,
        cloned_voting.voters_whitelist
    );
    assert_eq!(original_voting.state, cloned_voting.state);
}

#[test]
fn test_voting_with_notification() {
    let notification = Notification {
        email: "voting@example.com".to_string(),
        pdf_bytes: vec![0x25, 0x50, 0x44, 0x46], // PDF header
    };

    let mut options = BTreeMap::new();
    options.insert("Approve".to_string(), None);

    let voting = Voting::new(
        789,
        "Voting with Notification".to_string(),
        "Test notification".to_string(),
        options,
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        VotingState::Open,
        Principal::anonymous(),
        1672531200000,
        50,
        30,
        vec![],
        Some(notification.clone()),
    );

    assert!(voting.notification.is_some());
    let stored_notification = voting.notification.unwrap();
    assert_eq!(stored_notification.email, notification.email);
    assert_eq!(stored_notification.pdf_bytes, notification.pdf_bytes);
}

#[test]
fn test_voting_complex_options() {
    let mut options = BTreeMap::new();

    // Option with no action
    options.insert("No Action".to_string(), None);

    // Option with action
    options.insert(
        "Execute Transfer".to_string(),
        Some(Action {
            canister_id: Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
            method: "transfer".to_string(),
            args: vec!["1000".to_string(), "destination".to_string()],
        }),
    );

    // Another option with different action
    options.insert(
        "Create Proposal".to_string(),
        Some(Action {
            canister_id: Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
            method: "create_proposal".to_string(),
            args: vec!["New Proposal".to_string()],
        }),
    );

    let voting = Voting::new(
        999,
        "Complex Voting".to_string(),
        "Multiple options with different actions".to_string(),
        options.clone(),
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        VotingState::Open,
        Principal::anonymous(),
        1672531200000,
        66,
        40,
        vec![],
        None,
    );

    assert_eq!(voting.options.len(), 3);
    assert!(voting.options.contains_key("No Action"));
    assert!(voting.options.contains_key("Execute Transfer"));
    assert!(voting.options.contains_key("Create Proposal"));

    // Verify actions are properly stored
    assert!(voting.options["No Action"].is_none());
    assert!(voting.options["Execute Transfer"].is_some());
    assert!(voting.options["Create Proposal"].is_some());
}

#[test]
fn test_voting_edge_cases() {
    // Test with minimal values
    let minimal_voting = Voting::new(
        0,
        String::new(),
        String::new(),
        BTreeMap::new(),
        Principal::anonymous(),
        0,
        0,
        VotingState::Open,
        Principal::anonymous(),
        0,
        0,
        0,
        vec![],
        None,
    );

    assert_eq!(minimal_voting.id, 0);
    assert!(minimal_voting.title.is_empty());
    assert!(minimal_voting.description.is_empty());
    assert!(minimal_voting.options.is_empty());
    assert_eq!(minimal_voting.approval_threshold, 0);
    assert_eq!(minimal_voting.quorum, 0);

    // Test with maximum values
    let max_voting = Voting::new(
        u32::MAX,
        "Max".to_string(),
        "Max description".to_string(),
        BTreeMap::new(),
        Principal::anonymous(),
        u64::MAX,
        u64::MAX,
        VotingState::Closed,
        Principal::anonymous(),
        u64::MAX,
        u32::MAX,
        u32::MAX,
        vec![],
        None,
    );

    assert_eq!(max_voting.id, u32::MAX);
    assert_eq!(max_voting.created_at, u64::MAX);
    assert_eq!(max_voting.end_at, u64::MAX);
    assert_eq!(max_voting.approval_threshold, u32::MAX);
    assert_eq!(max_voting.quorum, u32::MAX);
}
