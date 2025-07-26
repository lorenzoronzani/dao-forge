use crate::models::DaoAssociation;
use crate::repositories::{get_memory, Memory, CONFIGURATION_MEMORY_ID, DAO_MEMORY_ID};
use common::models::Configuration;
use ic_stable_structures::Cell;
use std::cell::RefCell;

thread_local! {
    pub static DAO_MEMORY: RefCell<Cell<DaoAssociation, Memory>> = RefCell::new(Cell::init(get_memory(DAO_MEMORY_ID), DaoAssociation::default()).unwrap());

    pub static CONFIGURATION_MEMORY: RefCell<Cell<Configuration, Memory>> = RefCell::new(Cell::init(get_memory(CONFIGURATION_MEMORY_ID), Configuration::default()).unwrap());
}
