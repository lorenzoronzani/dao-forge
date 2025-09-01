use crate::models::Dao;
use crate::repositories::memories::DAO_MEMORY;

pub struct DaoRepository;

impl DaoRepository {
    pub fn save(dao: Dao) -> Dao {
        DAO_MEMORY.with_borrow_mut(|dao_memory| dao_memory.insert(dao.id, dao.clone()));

        dao
    }

    pub fn get(dao_id: u32) -> Option<Dao> {
        DAO_MEMORY.with_borrow(|dao_memory| dao_memory.get(&dao_id))
    }

    pub fn get_all() -> Vec<Dao> {
        DAO_MEMORY.with_borrow(|dao_memory| dao_memory.values().collect())
    }

    pub fn size() -> u64 {
        DAO_MEMORY.with_borrow(|dao_memory| dao_memory.len())
    }
}

#[cfg(test)]
#[path = "dao_repository_tests.rs"]
mod dao_repository_tests;
