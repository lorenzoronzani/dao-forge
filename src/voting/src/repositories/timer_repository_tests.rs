use super::*;
use crate::models::{Timer, TimerAction};

fn create_test_timer(id: u32, voting_id: u32) -> Timer {
    Timer::new(
        id,
        voting_id,
        1672531200000 + id as u64,
        TimerAction::EvaluateVoting,
    )
}

#[test]
fn test_timer_repository_save() {
    let test_timer = create_test_timer(123, 456);

    let saved_timer = TimerRepository::save(test_timer.clone());

    assert_eq!(saved_timer.id, test_timer.id);
    assert_eq!(saved_timer.voting_id, test_timer.voting_id);
    assert_eq!(saved_timer.end_at, test_timer.end_at);
    assert_eq!(saved_timer.action, test_timer.action);
}

#[test]
fn test_timer_repository_get_all() {
    let timer1 = create_test_timer(200, 300);
    let timer2 = create_test_timer(201, 301);

    TimerRepository::save(timer1.clone());
    TimerRepository::save(timer2.clone());

    let all_timers = TimerRepository::get_all();

    assert!(all_timers.len() >= 2);
    assert!(all_timers.iter().any(|t| t.id == timer1.id));
    assert!(all_timers.iter().any(|t| t.id == timer2.id));
}

#[test]
fn test_timer_repository_find_by_voting_id() {
    let voting_id = 400;
    let timer1 = create_test_timer(500, voting_id);
    let timer2 = create_test_timer(501, voting_id + 1);

    TimerRepository::save(timer1.clone());
    TimerRepository::save(timer2.clone());

    let found_timer = TimerRepository::find_by_voting_id(voting_id, TimerAction::EvaluateVoting);

    assert!(found_timer.is_some());
    let found = found_timer.unwrap();
    assert_eq!(found.id, timer1.id);
    assert_eq!(found.voting_id, voting_id);

    let not_found = TimerRepository::find_by_voting_id(99999, TimerAction::EvaluateVoting);
    assert!(not_found.is_none());
}

#[test]
fn test_timer_repository_delete() {
    let timer_id = 600;
    let timer = create_test_timer(timer_id, 700);

    TimerRepository::save(timer.clone());

    let found_before = TimerRepository::find_by_voting_id(700, TimerAction::EvaluateVoting);
    assert!(found_before.is_some());

    TimerRepository::delete(timer_id);

    let found_after = TimerRepository::find_by_voting_id(700, TimerAction::EvaluateVoting);
    assert!(found_after.is_none());
}

#[test]
fn test_timer_repository_size() {
    let initial_size = TimerRepository::size();

    let timer1 = create_test_timer(800, 900);
    let timer2 = create_test_timer(801, 901);

    TimerRepository::save(timer1);
    let size_after_one = TimerRepository::size();
    assert_eq!(size_after_one, initial_size + 1);

    TimerRepository::save(timer2);
    let size_after_two = TimerRepository::size();
    assert_eq!(size_after_two, initial_size + 2);
}

#[test]
fn test_timer_repository_overwrite() {
    let timer_id = 1000;
    let original_timer = create_test_timer(timer_id, 1100);

    TimerRepository::save(original_timer.clone());

    let updated_timer = Timer::new(timer_id, 1200, 1704067200000, TimerAction::EvaluateVoting);

    TimerRepository::save(updated_timer.clone());

    let found_original = TimerRepository::find_by_voting_id(1100, TimerAction::EvaluateVoting);
    let found_updated = TimerRepository::find_by_voting_id(1200, TimerAction::EvaluateVoting);

    assert!(found_original.is_none());
    assert!(found_updated.is_some());
    assert_eq!(found_updated.unwrap().voting_id, 1200);
}

#[test]
fn test_timer_repository_workflow() {
    let timer_id = 1500;
    let voting_id = 1600;
    let timer = create_test_timer(timer_id, voting_id);

    let not_found = TimerRepository::find_by_voting_id(voting_id, TimerAction::EvaluateVoting);
    assert!(not_found.is_none());

    TimerRepository::save(timer.clone());

    let found = TimerRepository::find_by_voting_id(voting_id, TimerAction::EvaluateVoting);
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, timer_id);

    let all_timers = TimerRepository::get_all();
    assert!(all_timers.iter().any(|t| t.id == timer_id));

    TimerRepository::delete(timer_id);

    let not_found_after =
        TimerRepository::find_by_voting_id(voting_id, TimerAction::EvaluateVoting);
    assert!(not_found_after.is_none());
}

#[test]
fn test_timer_repository_edge_cases() {
    let timer_zero = create_test_timer(0, 0);
    TimerRepository::save(timer_zero.clone());
    let found_zero = TimerRepository::find_by_voting_id(0, TimerAction::EvaluateVoting);
    assert!(found_zero.is_some());
    assert_eq!(found_zero.unwrap().id, 0);

    let timer_max = create_test_timer(u32::MAX, u32::MAX);
    TimerRepository::save(timer_max.clone());
    let found_max = TimerRepository::find_by_voting_id(u32::MAX, TimerAction::EvaluateVoting);
    assert!(found_max.is_some());
    assert_eq!(found_max.unwrap().id, u32::MAX);

    TimerRepository::delete(0);
    TimerRepository::delete(u32::MAX);
}
