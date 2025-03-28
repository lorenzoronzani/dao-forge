use candid::Principal;
use ic_cdk::caller;

use crate::{models::PrincipalDaoRegistry, services::PrincipalDaoRegistryService};

#[ic_cdk::update]
async fn save(user: Principal) -> PrincipalDaoRegistry {
    let mut user =
        PrincipalDaoRegistryService::get(user).unwrap_or_else(|| PrincipalDaoRegistry::new(user));

    user.add_dao(caller());

    PrincipalDaoRegistryService::save(user)
}

#[ic_cdk::query]
async fn get(key: Principal) -> Option<PrincipalDaoRegistry> {
    PrincipalDaoRegistryService::get(key)
}
