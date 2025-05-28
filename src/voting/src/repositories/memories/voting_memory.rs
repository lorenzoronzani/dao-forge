use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

use crate::{
    models::Voting,
    repositories::{get_memory, Memory, VOTING_MEMORY_ID},
};

thread_local! {
    pub static VOTING_MEMORY: RefCell<StableBTreeMap<u32, Voting, Memory>> = RefCell::new(
        StableBTreeMap::init(
            get_memory(VOTING_MEMORY_ID),
        )
    );
}
