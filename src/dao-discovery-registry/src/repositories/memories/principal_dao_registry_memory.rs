use crate::models::PrincipalDaoRegistry;
use crate::repositories::{get_memory, Memory, PRINCIPAL_DAO_REGISTRY_ID};
use candid::Principal;
use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

thread_local! {
    pub static PRINCIPAL_DAO_REGISTRY: RefCell<StableBTreeMap<Principal, PrincipalDaoRegistry, Memory>> = RefCell::new(
        StableBTreeMap::init(
            get_memory(PRINCIPAL_DAO_REGISTRY_ID),
        )
    );
}
