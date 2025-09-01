use super::*;

fn create_test_timer() -> Timer {
    Timer::new(
        123,
        456,
        1672531200000, // 2023-01-01 timestamp in milliseconds
        TimerAction::EvaluateVoting,
    )
}

#[test]
fn test_timer_creation() {
    let timer = create_test_timer();

    // Test basic properties
    assert_eq!(timer.id, 123);
    assert_eq!(timer.voting_id, 456);
    assert_eq!(timer.end_at, 1672531200000);
    assert_eq!(timer.action, TimerAction::EvaluateVoting);
}

#[test]
fn test_timer_new() {
    let id = 789;
    let voting_id = 101112;
    let end_at = 1704067200000; // 2024-01-01
    let action = TimerAction::EvaluateVoting;

    let timer = Timer::new(id, voting_id, end_at, action.clone());

    assert_eq!(timer.id, id);
    assert_eq!(timer.voting_id, voting_id);
    assert_eq!(timer.end_at, end_at);
    assert_eq!(timer.action, action);
}

#[test]
fn test_timer_storable() {
    let original_timer = create_test_timer();

    // Test serialization
    let bytes = original_timer.to_bytes();
    assert!(!bytes.is_empty());

    // Test deserialization
    let deserialized_timer = Timer::from_bytes(bytes);

    // Verify data integrity
    assert_eq!(original_timer.id, deserialized_timer.id);
    assert_eq!(original_timer.voting_id, deserialized_timer.voting_id);
    assert_eq!(original_timer.end_at, deserialized_timer.end_at);
    assert_eq!(original_timer.action, deserialized_timer.action);
}

#[test]
fn test_timer_clone() {
    let original_timer = create_test_timer();
    let cloned_timer = original_timer.clone();

    // Verify clone works correctly
    assert_eq!(original_timer.id, cloned_timer.id);
    assert_eq!(original_timer.voting_id, cloned_timer.voting_id);
    assert_eq!(original_timer.end_at, cloned_timer.end_at);
    assert_eq!(original_timer.action, cloned_timer.action);
}

#[test]
fn test_timer_action_enum() {
    let action1 = TimerAction::EvaluateVoting;
    let action2 = TimerAction::EvaluateVoting;

    // Test PartialEq implementation
    assert_eq!(action1, action2);

    // Test Clone implementation
    let cloned_action = action1.clone();
    assert_eq!(action1, cloned_action);
}

#[test]
fn test_timer_with_different_values() {
    let timer1 = Timer::new(1, 100, 1640995200000, TimerAction::EvaluateVoting);
    let timer2 = Timer::new(2, 200, 1672531200000, TimerAction::EvaluateVoting);

    // Verify different timers have different properties
    assert_ne!(timer1.id, timer2.id);
    assert_ne!(timer1.voting_id, timer2.voting_id);
    assert_ne!(timer1.end_at, timer2.end_at);
    assert_eq!(timer1.action, timer2.action);
}

#[test]
fn test_timer_edge_cases() {
    let timer_zero = Timer::new(0, 0, 0, TimerAction::EvaluateVoting);
    assert_eq!(timer_zero.id, 0);
    assert_eq!(timer_zero.voting_id, 0);
    assert_eq!(timer_zero.end_at, 0);

    // Test with maximum values
    let timer_max = Timer::new(u32::MAX, u32::MAX, u64::MAX, TimerAction::EvaluateVoting);
    assert_eq!(timer_max.id, u32::MAX);
    assert_eq!(timer_max.voting_id, u32::MAX);
    assert_eq!(timer_max.end_at, u64::MAX);
}

#[test]
fn test_timer_serialization_with_enum() {
    let timer = Timer::new(999, 888, 1704153600000, TimerAction::EvaluateVoting);

    // Test that enum serializes and deserializes correctly
    let bytes = timer.to_bytes();
    let deserialized = Timer::from_bytes(bytes);

    assert_eq!(timer.action, deserialized.action);
    assert_eq!(deserialized.action, TimerAction::EvaluateVoting);
}

#[test]
fn test_timer_voting_scenarios() {
    let short_timer = Timer::new(1, 100, 1672531200000, TimerAction::EvaluateVoting);

    // Test timer for long voting period
    let long_timer = Timer::new(
        2,
        200,
        1735689600000, // End in future (2025-01-01)
        TimerAction::EvaluateVoting,
    );

    assert!(long_timer.end_at > short_timer.end_at);
    assert_eq!(short_timer.action, long_timer.action);
    assert_ne!(short_timer.voting_id, long_timer.voting_id);
}

#[test]
fn test_timer_action_debug() {
    let action = TimerAction::EvaluateVoting;
    let debug_str = format!("{:?}", action);
    assert_eq!(debug_str, "EvaluateVoting");
}
