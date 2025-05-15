use crate::models::DaoAssociation;
use crate::repositories::{get_memory, Memory, DAO_MEMORY_ID};
use ic_stable_structures::Cell;
use std::cell::RefCell;

thread_local! {
    pub static DAO_MEMORY: RefCell<Cell<DaoAssociation, Memory>> = RefCell::new(Cell::init(get_memory(DAO_MEMORY_ID), DaoAssociation::default()).unwrap());
}
