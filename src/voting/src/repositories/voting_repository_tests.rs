use super::*;
use crate::models::{Voting, VotingState};
use candid::Principal;
use std::collections::BTreeMap;

fn create_test_voting(id: u32) -> Voting {
    let mut options = BTreeMap::new();
    options.insert("Option A".to_string(), None);
    options.insert("Option B".to_string(), None);

    let voters_whitelist = vec![Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap()];

    Voting::new(
        id,
        format!("Test Voting {}", id),
        format!("Description for voting {}", id),
        options,
        Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
        1672531200000,
        1672617600000,
        VotingState::Open,
        Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
        1672531200000,
        60,
        50,
        voters_whitelist,
        None,
    )
}

#[test]
fn test_voting_repository_save_and_get() {
    let test_voting = create_test_voting(123);

    let saved_voting = VotingRepository::save(test_voting.clone());

    assert_eq!(saved_voting.id, test_voting.id);
    assert_eq!(saved_voting.title, test_voting.title);
    assert_eq!(saved_voting.description, test_voting.description);

    let retrieved_voting = VotingRepository::get(123);

    assert!(retrieved_voting.is_some());
    let retrieved = retrieved_voting.unwrap();
    assert_eq!(retrieved.id, test_voting.id);
    assert_eq!(retrieved.title, test_voting.title);
    assert_eq!(retrieved.owner, test_voting.owner);
    assert_eq!(retrieved.state, test_voting.state);
}

#[test]
fn test_voting_repository_get_nonexistent() {
    let result = VotingRepository::get(99999);
    assert!(result.is_none());
}

#[test]
fn test_voting_repository_save_multiple() {
    let voting1 = create_test_voting(200);
    let voting2 = create_test_voting(201);

    VotingRepository::save(voting1.clone());
    VotingRepository::save(voting2.clone());

    let retrieved1 = VotingRepository::get(200).unwrap();
    let retrieved2 = VotingRepository::get(201).unwrap();

    assert_eq!(retrieved1.id, voting1.id);
    assert_eq!(retrieved2.id, voting2.id);
    assert_eq!(retrieved1.title, voting1.title);
    assert_eq!(retrieved2.title, voting2.title);
    assert_ne!(retrieved1.id, retrieved2.id);
}

#[test]
fn test_voting_repository_size() {
    let initial_size = VotingRepository::size();

    let voting1 = create_test_voting(300);
    let voting2 = create_test_voting(301);

    VotingRepository::save(voting1);
    let size_after_one = VotingRepository::size();
    assert_eq!(size_after_one, initial_size + 1);

    VotingRepository::save(voting2);
    let size_after_two = VotingRepository::size();
    assert_eq!(size_after_two, initial_size + 2);
}

#[test]
fn test_voting_repository_overwrite() {
    let voting_id = 400;
    let original_voting = create_test_voting(voting_id);

    VotingRepository::save(original_voting.clone());

    let mut updated_voting = create_test_voting(voting_id);
    updated_voting.title = "Updated Voting Title".to_string();
    updated_voting.state = VotingState::Closed;

    VotingRepository::save(updated_voting.clone());

    let retrieved = VotingRepository::get(voting_id).unwrap();
    assert_eq!(retrieved.id, voting_id);
    assert_eq!(retrieved.title, "Updated Voting Title");
    assert_eq!(retrieved.state, VotingState::Closed);
    assert_ne!(retrieved.title, original_voting.title);
}

#[test]
fn test_voting_repository_complex_voting() {
    let mut options = BTreeMap::new();
    options.insert("Complex Option 1".to_string(), None);
    options.insert("Complex Option 2".to_string(), None);

    let complex_voting = Voting::new(
        500,
        "Complex Voting Test".to_string(),
        "A complex voting with many parameters".to_string(),
        options,
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        VotingState::Open,
        Principal::anonymous(),
        1672531200000,
        75,
        25,
        vec![],
        None,
    );

    VotingRepository::save(complex_voting.clone());
    let retrieved = VotingRepository::get(500).unwrap();

    assert_eq!(retrieved.approval_threshold, 75);
    assert_eq!(retrieved.quorum, 25);
    assert_eq!(retrieved.options.len(), 2);
    assert!(retrieved.voters_whitelist.is_empty());
}

#[test]
fn test_voting_repository_workflow() {
    let voting_id = 600;
    let voting = create_test_voting(voting_id);

    assert!(VotingRepository::get(voting_id).is_none());

    let saved = VotingRepository::save(voting.clone());
    assert_eq!(saved.id, voting.id);

    let retrieved = VotingRepository::get(voting_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, voting_id);

    assert!(VotingRepository::size() > 0);
}

#[test]
fn test_voting_repository_edge_cases() {
    // Test with ID 0
    let voting_zero = create_test_voting(0);
    VotingRepository::save(voting_zero.clone());
    let retrieved_zero = VotingRepository::get(0);
    assert!(retrieved_zero.is_some());
    assert_eq!(retrieved_zero.unwrap().id, 0);

    // Test with maximum u32
    let voting_max = create_test_voting(u32::MAX);
    VotingRepository::save(voting_max.clone());
    let retrieved_max = VotingRepository::get(u32::MAX);
    assert!(retrieved_max.is_some());
    assert_eq!(retrieved_max.unwrap().id, u32::MAX);
}
