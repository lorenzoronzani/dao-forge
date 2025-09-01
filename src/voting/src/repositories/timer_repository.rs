use crate::{
    models::{Timer, TimerAction},
    repositories::TIMER_MEMORY,
};

pub struct TimerRepository;

impl TimerRepository {
    pub fn save(timer: Timer) -> Timer {
        TIMER_MEMORY.with_borrow_mut(|timer_memory| {
            timer_memory.insert(timer.id.clone(), timer.clone());
        });

        timer
    }

    pub fn get_all() -> Vec<Timer> {
        TIMER_MEMORY.with_borrow(|timer_memory| timer_memory.values().collect())
    }

    pub fn find_by_voting_id(voting_id: u32, action: TimerAction) -> Option<Timer> {
        TIMER_MEMORY.with_borrow(|timer_memory| {
            timer_memory
                .values()
                .find(|timer| timer.voting_id == voting_id && timer.action == action)
        })
    }

    pub fn delete(timer_id: u32) {
        TIMER_MEMORY.with_borrow_mut(|timer_memory| timer_memory.remove(&timer_id));
    }

    pub fn size() -> u64 {
        TIMER_MEMORY.with_borrow(|timer_memory| timer_memory.len() as u64)
    }
}

#[cfg(test)]
#[path = "timer_repository_tests.rs"]
mod timer_repository_tests;
