use candid::Principal;

use crate::{models::PrincipalDaoRegistry, services::DaoDiscoveryService};

#[ic_cdk::update]
async fn save(user: Principal, dao: Principal) -> PrincipalDaoRegistry {
    let mut user =
        DaoDiscoveryService::get(user).unwrap_or_else(|| PrincipalDaoRegistry::new(user));

    user.add_dao(dao);

    DaoDiscoveryService::save(user)
}

#[ic_cdk::query]
async fn get(key: Principal) -> Option<PrincipalDaoRegistry> {
    DaoDiscoveryService::get(key)
}
