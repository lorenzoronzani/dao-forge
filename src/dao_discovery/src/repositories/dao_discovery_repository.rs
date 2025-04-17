use candid::Principal;

use crate::repositories::memories::DAO_DISCOVERY_MEMORY;

pub struct DaoDiscoveryRepository;

impl DaoDiscoveryRepository {
    pub fn save(principal_dao_registry: PrincipalDaoRegistry) -> PrincipalDaoRegistry {
        DAO_DISCOVERY_MEMORY.with_borrow_mut(|dao_memory| {
            dao_memory.insert(principal_dao_registry.user, principal_dao_registry.clone())
        });

        principal_dao_registry
    }

    pub fn get(key: Principal) -> Option<PrincipalDaoRegistry> {
        DAO_DISCOVERY_MEMORY.with_borrow(|dao_memory| dao_memory.get(&key))
    }
}
