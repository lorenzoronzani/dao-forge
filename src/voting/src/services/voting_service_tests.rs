use super::*;
use crate::models::{Action, Notification, Voting, VotingState};
use candid::Principal;
use std::collections::BTreeMap;

fn create_test_voting_data() -> (BTreeMap<String, Option<Action>>, Vec<Principal>) {
    let mut options = BTreeMap::new();
    options.insert(
        "Yes".to_string(),
        Some(Action {
            canister_id: Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
            method: "approve_proposal".to_string(),
            args: vec!["proposal_id".to_string()],
        }),
    );
    options.insert("No".to_string(), None);

    let voters = vec![
        Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
        Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
    ];

    (options, voters)
}

#[test]
fn test_voting_service_save() {
    let (options, voters) = create_test_voting_data();

    let voting = VotingService::save(
        "Test Voting".to_string(),
        "A test voting proposal".to_string(),
        options.clone(),
        Principal::from_text("ggppn-oqaaa-aaaaa-aaaaq-azy").unwrap(),
        1672531200000,
        1672617600000,
        Principal::from_text("ueq6w-kyaaa-aaaaa-aaaaq-a2a").unwrap(),
        60,
        50,
        voters.clone(),
        None,
    );

    assert!(voting.id > 0);
    assert_eq!(voting.title, "Test Voting");
    assert_eq!(voting.description, "A test voting proposal");
    assert_eq!(voting.options, options);
    assert_eq!(voting.state, VotingState::Open);
    assert_eq!(voting.approval_threshold, 60);
    assert_eq!(voting.quorum, 50);
    assert_eq!(voting.voters_whitelist, voters);
    assert!(voting.result.is_empty());
    assert!(voting.voters_cast.is_empty());
}

#[test]
fn test_voting_service_get() {
    let (options, voters) = create_test_voting_data();

    let saved_voting = VotingService::save(
        "Get Test".to_string(),
        "Test get functionality".to_string(),
        options,
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        Principal::anonymous(),
        70,
        40,
        voters,
        None,
    );

    let retrieved_voting = VotingService::get(saved_voting.id);
    assert!(retrieved_voting.is_some());
    let retrieved = retrieved_voting.unwrap();
    assert_eq!(retrieved.id, saved_voting.id);
    assert_eq!(retrieved.title, "Get Test");
}

#[test]
fn test_voting_service_get_nonexistent() {
    let result = VotingService::get(99999);
    assert!(result.is_none());
}

#[test]
fn test_voting_service_update() {
    let (options, voters) = create_test_voting_data();

    let mut voting = VotingService::save(
        "Update Test".to_string(),
        "Test update functionality".to_string(),
        options,
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        Principal::anonymous(),
        65,
        45,
        voters,
        None,
    );

    voting.title = "Updated Title".to_string();
    voting.state = VotingState::Closed;

    let updated_voting = VotingService::update(voting.clone());

    assert_eq!(updated_voting.title, "Updated Title");
    assert_eq!(updated_voting.state, VotingState::Closed);
}

#[test]
fn test_check_quorum() {
    let mut voting = Voting::new(
        1,
        "Quorum Test".to_string(),
        "Test quorum calculation".to_string(),
        BTreeMap::new(),
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        VotingState::Open,
        Principal::anonymous(),
        1672531200000,
        60,
        50,
        vec![
            Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
            Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
            Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
            Principal::from_text("ggppn-oqaaa-aaaaa-aaaaq-azy").unwrap(),
        ],
        None,
    );

    voting.voters_cast.insert(
        Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
        "Yes".to_string(),
    );
    assert!(!VotingService::check_quorum(&voting));

    voting.voters_cast.insert(
        Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
        "No".to_string(),
    );
    assert!(VotingService::check_quorum(&voting));

    voting.voters_cast.insert(
        Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
        "Yes".to_string(),
    );
    assert!(VotingService::check_quorum(&voting));
}

#[test]
fn test_retrieve_winner() {
    let mut voting = Voting::new(
        1,
        "Winner Test".to_string(),
        "Test winner retrieval".to_string(),
        BTreeMap::new(),
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        VotingState::Open,
        Principal::anonymous(),
        1672531200000,
        60,
        50,
        vec![],
        None,
    );

    voting.result.insert("Option A".to_string(), 3);
    voting.result.insert("Option B".to_string(), 7);
    voting.result.insert("Option C".to_string(), 2);

    let winner = VotingService::retrieve_winner(&voting);
    assert_eq!(winner, "Option B");
}

#[test]
fn test_retrieve_winner_tie() {
    let mut voting = Voting::new(
        1,
        "Tie Test".to_string(),
        "Test tie handling".to_string(),
        BTreeMap::new(),
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        VotingState::Open,
        Principal::anonymous(),
        1672531200000,
        60,
        50,
        vec![],
        None,
    );

    voting.result.insert("Option A".to_string(), 5);
    voting.result.insert("Option B".to_string(), 5);

    let winner = VotingService::retrieve_winner(&voting);
    assert!(winner == "Option A" || winner == "Option B");
}

#[test]
fn test_check_approval_threshold() {
    let mut voting = Voting::new(
        1,
        "Threshold Test".to_string(),
        "Test approval threshold".to_string(),
        BTreeMap::new(),
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        VotingState::Open,
        Principal::anonymous(),
        1672531200000,
        60,
        50,
        vec![],
        None,
    );

    for i in 0u8..10 {
        let p = Principal::self_authenticating(&[i]);
        voting.voters_cast.insert(p, "Option A".to_string());
    }
    assert_eq!(voting.voters_cast.len(), 10);

    voting.result.insert("Option A".to_string(), 5);
    assert!(!VotingService::check_approval_threshold(
        &"Option A".to_string(),
        &voting
    ));

    voting.result.insert("Option A".to_string(), 6);
    assert!(VotingService::check_approval_threshold(
        &"Option A".to_string(),
        &voting
    ));

    voting.result.insert("Option A".to_string(), 7);
    assert!(VotingService::check_approval_threshold(
        &"Option A".to_string(),
        &voting
    ));
}

#[test]
fn test_encode_args_simple_strings() {
    let args = vec!["arg1".to_string(), "arg2".to_string()];
    let encoded = VotingService::encode_args(args);

    assert!(!encoded.is_empty());
}

#[test]
fn test_encode_args_with_numbers() {
    let args = vec!["123".to_string(), "hello".to_string()];
    let encoded = VotingService::encode_args(args);

    assert!(!encoded.is_empty());
}

#[test]
fn test_encode_args_with_json() {
    let args = vec![r#"{"name": "test", "value": 42}"#.to_string()];
    let encoded = VotingService::encode_args(args);

    assert!(!encoded.is_empty());
}

#[test]
fn test_json_value_to_candid_primitives() {
    let candid_null = VotingService::json_value_to_candid(serde_json::Value::Null);
    assert!(matches!(candid_null, IDLValue::Null));

    let candid_bool = VotingService::json_value_to_candid(serde_json::Value::Bool(true));
    assert!(matches!(candid_bool, IDLValue::Bool(true)));

    let candid_text =
        VotingService::json_value_to_candid(serde_json::Value::String("test".to_string()));
    assert!(matches!(candid_text, IDLValue::Text(_)));

    let candid_nat =
        VotingService::json_value_to_candid(serde_json::Value::String("123".to_string()));
    assert!(matches!(candid_nat, IDLValue::Nat32(123)));
}

#[test]
fn test_json_value_to_candid_complex() {
    let json_array = serde_json::json!(["a", "b", "c"]);
    let candid_vec = VotingService::json_value_to_candid(json_array);
    if let IDLValue::Vec(values) = candid_vec {
        assert_eq!(values.len(), 3);
    } else {
        panic!("Expected Vec");
    }

    let json_obj = serde_json::json!({"name": "test", "age": "25"});
    let candid_record = VotingService::json_value_to_candid(json_obj);
    assert!(matches!(candid_record, IDLValue::Record(_)));

    let json_variant = serde_json::json!({"Success": "data"});
    let candid_variant = VotingService::json_value_to_candid(json_variant);
    assert!(matches!(candid_variant, IDLValue::Variant(_)));
}

#[test]
fn test_voting_with_notification() {
    let notification = Notification {
        email: "test@example.com".to_string(),
        pdf_bytes: vec![0x25, 0x50, 0x44, 0x46],
    };

    let (options, voters) = create_test_voting_data();

    let voting = VotingService::save(
        "Notification Test".to_string(),
        "Test with notification".to_string(),
        options,
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        Principal::anonymous(),
        60,
        50,
        voters,
        Some(notification.clone()),
    );

    assert!(voting.notification.is_some());
    let stored_notification = voting.notification.unwrap();
    assert_eq!(stored_notification.email, notification.email);
    assert_eq!(stored_notification.pdf_bytes, notification.pdf_bytes);
}

#[test]
fn test_edge_cases_thresholds() {
    let (options, voters) = create_test_voting_data();

    let voting_zero = VotingService::save(
        "Zero Threshold".to_string(),
        "Test zero thresholds".to_string(),
        options.clone(),
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        Principal::anonymous(),
        0,
        0,
        voters.clone(),
        None,
    );

    assert_eq!(voting_zero.approval_threshold, 0);
    assert_eq!(voting_zero.quorum, 0);

    let voting_hundred = VotingService::save(
        "Hundred Threshold".to_string(),
        "Test hundred thresholds".to_string(),
        options,
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        Principal::anonymous(),
        100,
        100,
        voters,
        None,
    );

    assert_eq!(voting_hundred.approval_threshold, 100);
    assert_eq!(voting_hundred.quorum, 100);
}

#[test]
fn test_auto_increment_ids() {
    let (options, voters) = create_test_voting_data();

    let voting1 = VotingService::save(
        "ID Test 1".to_string(),
        "First voting".to_string(),
        options.clone(),
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        Principal::anonymous(),
        60,
        50,
        voters.clone(),
        None,
    );

    let voting2 = VotingService::save(
        "ID Test 2".to_string(),
        "Second voting".to_string(),
        options,
        Principal::anonymous(),
        1672531200000,
        1672617600000,
        Principal::anonymous(),
        60,
        50,
        voters,
        None,
    );

    assert!(voting2.id > voting1.id);
    assert_ne!(voting1.id, voting2.id);
}
