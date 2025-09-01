use std::time::Duration;

use common::utils::Date;
use ic_cdk::{api::time, println, spawn};
use ic_cdk_timers::{clear_timer, set_timer, TimerId};

use crate::models::{Timer, TimerAction};
use crate::repositories::{TimerIdRepository, TimerRepository};

use super::VotingService;

use std::sync::atomic::{AtomicU32, Ordering};

pub struct TimerService;

static TIMER_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

impl TimerService {
    pub fn set_voting_timer(
        voting_id: u32,
        end_at: u64,
        action: TimerAction,
    ) -> Result<Timer, String> {
        let current_time_ns = time();
        let end_at_ns = Date::milliseconds_to_nanoseconds(end_at);

        if end_at_ns <= current_time_ns {
            return Err("End time must be greater than current time".to_string());
        }

        let delay = Duration::from_nanos(end_at_ns - current_time_ns);

        let timer_id = Self::set_timer(delay, voting_id, action.clone());

        let timer = Timer::new(
            TIMER_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            voting_id,
            end_at,
            action,
        );

        TimerIdRepository::save(timer.id, timer_id);

        Ok(TimerRepository::save(timer))
    }

    fn set_timer(delay: Duration, voting_id: u32, action: TimerAction) -> TimerId {
        set_timer(delay, move || {
            spawn(async move {
                Self::execute_voting_timer_action(voting_id, action).await;
            });
        })
    }

    async fn execute_voting_timer_action(voting_id: u32, action: TimerAction) -> () {
        println!(
            "Executing timer action for voting {} with action {:?}",
            voting_id, action
        );

        match action {
            TimerAction::EvaluateVoting => {
                VotingService::evaluate_voting(voting_id).await;
            }
        }
    }

    pub fn get_all() -> Vec<Timer> {
        TimerRepository::get_all()
    }

    pub fn find_by_voting_id(voting_id: u32, action: TimerAction) -> Option<Timer> {
        TimerRepository::find_by_voting_id(voting_id, action)
    }

    pub fn clear_timer(timer_id: u32) {
        let ic_timer_id = TimerIdRepository::get(timer_id).expect("Timer not found");
        clear_timer(ic_timer_id);

        TimerIdRepository::delete(timer_id);
        TimerRepository::delete(timer_id);
    }

    pub async fn restore_timer(timer: Timer) -> () {
        TIMER_ID_COUNTER.fetch_max(timer.id, Ordering::Relaxed);

        let current_time_ns = time();
        let end_at_ns = Date::milliseconds_to_nanoseconds(timer.end_at);

        let action_cloned = timer.action.clone();

        if end_at_ns <= current_time_ns {
            spawn(async move {
                Self::execute_voting_timer_action(timer.voting_id, action_cloned).await;
            });
        }

        let delay = Duration::from_nanos(end_at_ns - current_time_ns);

        let timer_id = Self::set_timer(delay, timer.voting_id, timer.action);

        TimerIdRepository::save(timer.id, timer_id);
    }

    pub fn size() -> u64 {
        TimerRepository::size()
    }
}

#[cfg(test)]
#[path = "timer_service_tests.rs"]
mod timer_service_tests;
