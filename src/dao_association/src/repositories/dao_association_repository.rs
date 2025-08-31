use crate::models::DaoAssociation;
use crate::repositories::DAO_MEMORY;

pub struct DaoAssociationRepository;

impl DaoAssociationRepository {
    pub fn save(dao_association: DaoAssociation) -> DaoAssociation {
        DAO_MEMORY.with_borrow_mut(|dao_memory| {
            dao_memory.set(dao_association.clone()).unwrap();
        });

        dao_association
    }

    pub fn get() -> DaoAssociation {
        DAO_MEMORY.with_borrow(|dao_memory| dao_memory.get().clone())
    }
}

#[cfg(test)]
#[path = "dao_association_repository_tests.rs"]
mod dao_association_repository_tests;
