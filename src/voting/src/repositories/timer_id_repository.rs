use ic_cdk_timers::TimerId;

use crate::repositories::TIMER_ID_MAP;

pub struct TimerIdRepository;

impl TimerIdRepository {
    pub fn save(timer_id: u32, ic_timer_id: TimerId) {
        TIMER_ID_MAP.with_borrow_mut(|timer_id_map| {
            timer_id_map.insert(timer_id.clone(), ic_timer_id.clone());
        });
    }

    pub fn get(timer_id: u32) -> Option<TimerId> {
        TIMER_ID_MAP.with_borrow(|timer_id_map| timer_id_map.get(&timer_id).copied())
    }

    pub fn delete(timer_id: u32) {
        TIMER_ID_MAP.with_borrow_mut(|timer_id_map| timer_id_map.remove(&timer_id));
    }
}

#[cfg(test)]
#[path = "timer_id_repository_tests.rs"]
mod timer_id_repository_tests;
