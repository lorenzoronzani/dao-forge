use candid::Principal;

use crate::models::PrincipalDaoRegistry;
use crate::repositories::DaoDiscoveryRepository;

pub struct DaoDiscoveryService;

impl DaoDiscoveryService {
    pub fn save(principal_dao_registry: PrincipalDaoRegistry) -> PrincipalDaoRegistry {
        DaoDiscoveryRepository::save(principal_dao_registry)
    }

    pub fn get(key: Principal) -> Option<PrincipalDaoRegistry> {
        DaoDiscoveryRepository::get(key)
    }
}
