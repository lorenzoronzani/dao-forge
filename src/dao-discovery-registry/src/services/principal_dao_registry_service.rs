use candid::Principal;

use crate::models::PrincipalDaoRegistry;
use crate::repositories::PrincipalDaoRegistryRepository;

pub struct PrincipalDaoRegistryService;

impl PrincipalDaoRegistryService {
    pub fn save(principal_dao_registry: PrincipalDaoRegistry) -> PrincipalDaoRegistry {
        PrincipalDaoRegistryRepository::save(principal_dao_registry)
    }

    pub fn get(key: Principal) -> Option<PrincipalDaoRegistry> {
        PrincipalDaoRegistryRepository::get(key)
    }
}
