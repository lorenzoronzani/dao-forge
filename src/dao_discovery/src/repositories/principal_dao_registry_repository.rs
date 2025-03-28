use candid::Principal;

use crate::models::PrincipalDaoRegistry;
use crate::repositories::memories::PRINCIPAL_DAO_REGISTRY;

pub struct PrincipalDaoRegistryRepository;

impl PrincipalDaoRegistryRepository {
    pub fn save(principal_dao_registry: PrincipalDaoRegistry) -> PrincipalDaoRegistry {
        PRINCIPAL_DAO_REGISTRY.with_borrow_mut(|dao_memory| {
            dao_memory.insert(principal_dao_registry.user, principal_dao_registry.clone())
        });

        principal_dao_registry
    }

    pub fn get(key: Principal) -> Option<PrincipalDaoRegistry> {
        PRINCIPAL_DAO_REGISTRY.with_borrow(|dao_memory| dao_memory.get(&key))
    }
}
