use candid::Principal;
use ic_cdk::caller;

use crate::{models::PrincipalDaoRegistry, services::PrincipalDaoRegistryService};

#[ic_cdk::update]
async fn save(user: Principal) -> PrincipalDaoRegistry {
    let mut principal_dao_registry = PrincipalDaoRegistry::new(user);
    principal_dao_registry.add_dao(caller());

    PrincipalDaoRegistryService::save(principal_dao_registry)
}

#[ic_cdk::query]
async fn get(key: Principal) -> Option<PrincipalDaoRegistry> {
    PrincipalDaoRegistryService::get(key)
}
