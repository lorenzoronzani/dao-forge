use crate::{
    models::{Dao, User},
    repositories::{get_memory, Memory, DAO_MEMORY_ID, USER_MEMORY_ID},
};
use candid::Principal;
use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

thread_local! {
    pub static USER_MEMORY: RefCell<StableBTreeMap<Principal, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            get_memory(USER_MEMORY_ID),
        )
    );

    pub static DAO_MEMORY: RefCell<StableBTreeMap<u32, Dao, Memory>> = RefCell::new(
        StableBTreeMap::init(
            get_memory(DAO_MEMORY_ID),
        )
    );
}
