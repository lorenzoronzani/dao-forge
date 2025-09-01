use super::*;
use candid::Principal;

fn create_test_action() -> Action {
    Action {
        canister_id: Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
        method: "transfer".to_string(),
        args: vec!["100".to_string(), "destination_account".to_string()],
    }
}

#[test]
fn test_action_creation() {
    let action = create_test_action();

    assert_eq!(
        action.canister_id,
        Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap()
    );
    assert_eq!(action.method, "transfer");
    assert_eq!(action.args.len(), 2);
    assert_eq!(action.args[0], "100");
    assert_eq!(action.args[1], "destination_account");
}

#[test]
fn test_action_with_different_values() {
    let canister_id = Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap();
    let method = "create_dao".to_string();
    let args = vec![
        "DAO Name".to_string(),
        "Purpose description".to_string(),
        "123456".to_string(),
    ];

    let action = Action {
        canister_id,
        method: method.clone(),
        args: args.clone(),
    };

    assert_eq!(action.canister_id, canister_id);
    assert_eq!(action.method, method);
    assert_eq!(action.args, args);
    assert_eq!(action.args.len(), 3);
}

#[test]
fn test_action_storable() {
    let original_action = create_test_action();

    // Test serialization
    let bytes = original_action.to_bytes();
    assert!(!bytes.is_empty());

    // Test deserialization
    let deserialized_action = Action::from_bytes(bytes);

    // Verify data integrity
    assert_eq!(original_action.canister_id, deserialized_action.canister_id);
    assert_eq!(original_action.method, deserialized_action.method);
    assert_eq!(original_action.args, deserialized_action.args);
}

#[test]
fn test_action_clone() {
    let original_action = create_test_action();
    let cloned_action = original_action.clone();

    // Verify clone works correctly
    assert_eq!(original_action.canister_id, cloned_action.canister_id);
    assert_eq!(original_action.method, cloned_action.method);
    assert_eq!(original_action.args, cloned_action.args);
}

#[test]
fn test_action_empty_args() {
    let action = Action {
        canister_id: Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
        method: "get_balance".to_string(),
        args: vec![],
    };

    assert_eq!(action.method, "get_balance");
    assert!(action.args.is_empty());
}

#[test]
fn test_action_single_arg() {
    let action = Action {
        canister_id: Principal::from_text("ggppn-oqaaa-aaaaa-aaaaq-azy").unwrap(),
        method: "get_user".to_string(),
        args: vec!["user_id_123".to_string()],
    };

    assert_eq!(action.args.len(), 1);
    assert_eq!(action.args[0], "user_id_123");
}

#[test]
fn test_action_many_args() {
    let args = vec![
        "arg1".to_string(),
        "arg2".to_string(),
        "arg3".to_string(),
        "arg4".to_string(),
        "arg5".to_string(),
    ];

    let action = Action {
        canister_id: Principal::from_text("ueq6w-kyaaa-aaaaa-aaaaq-a2a").unwrap(),
        method: "complex_method".to_string(),
        args: args.clone(),
    };

    assert_eq!(action.args.len(), 5);
    assert_eq!(action.args, args);
}

#[test]
fn test_action_different_principals() {
    let action1 = Action {
        canister_id: Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
        method: "method1".to_string(),
        args: vec!["arg1".to_string()],
    };

    let action2 = Action {
        canister_id: Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
        method: "method2".to_string(),
        args: vec!["arg2".to_string()],
    };

    assert_ne!(action1.canister_id, action2.canister_id);
    assert_ne!(action1.method, action2.method);
    assert_ne!(action1.args, action2.args);
}

#[test]
fn test_action_complex_serialization() {
    let action = Action {
        canister_id: Principal::anonymous(),
        method: "complex_method_with_underscores".to_string(),
        args: vec![
            "arg_with_special_chars_!@#$%".to_string(),
            "long_argument_with_many_characters_to_test_serialization".to_string(),
            "123456789".to_string(),
            "".to_string(),
        ],
    };

    // Test that complex data serializes and deserializes correctly
    let bytes = action.to_bytes();
    let deserialized = Action::from_bytes(bytes);

    assert_eq!(action.canister_id, deserialized.canister_id);
    assert_eq!(action.method, deserialized.method);
    assert_eq!(action.args, deserialized.args);
    assert_eq!(deserialized.args.len(), 4);
    assert!(deserialized.args[3].is_empty());
}

#[test]
fn test_action_with_anonymous_principal() {
    let action = Action {
        canister_id: Principal::anonymous(),
        method: "anonymous_call".to_string(),
        args: vec!["test".to_string()],
    };

    assert_eq!(action.canister_id, Principal::anonymous());
    assert_eq!(action.method, "anonymous_call");
}
