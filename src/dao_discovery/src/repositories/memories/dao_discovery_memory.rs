use crate::models::PrincipalDaoRegistry;
use crate::repositories::{get_memory, Memory, DAO_DISCOVERY_MEMORY_ID};
use candid::Principal;
use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

thread_local! {
    pub static DAO_DISCOVERY_MEMORY: RefCell<StableBTreeMap<Principal, PrincipalDaoRegistry, Memory>> = RefCell::new(
        StableBTreeMap::init(
            get_memory(DAO_DISCOVERY_MEMORY_ID),
        )
    );
}
