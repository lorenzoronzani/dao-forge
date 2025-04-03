use crate::models::SogcPublication;
use crate::repositories::{get_memory, Memory, SOGC_PUBBLICATION_MEMORY_ID};
use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

thread_local! {
    pub static SOGC_PUBBLICATION_MEMORY: RefCell<StableBTreeMap<u64, SogcPublication, Memory>> = RefCell::new(StableBTreeMap::init(get_memory(SOGC_PUBBLICATION_MEMORY_ID)));
}
