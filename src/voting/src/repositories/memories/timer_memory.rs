use ic_cdk_timers::TimerId;
use ic_stable_structures::StableBTreeMap;
use std::{cell::RefCell, collections::HashMap};

use crate::{
    models::Timer,
    repositories::{get_memory, Memory, TIMER_MEMORY_ID},
};

thread_local! {
    pub static TIMER_MEMORY: RefCell<StableBTreeMap<u32, Timer, Memory>> = RefCell::new(
        StableBTreeMap::init(
            get_memory(TIMER_MEMORY_ID),
        )
    );

    pub static TIMER_ID_MAP: RefCell<HashMap<u32, TimerId>> = RefCell::new(HashMap::new());
}
