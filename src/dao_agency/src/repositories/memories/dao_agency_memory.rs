use crate::repositories::{get_memory, Memory, CONFIGURATION_MEMORY_ID};
use common::models::Configuration;
use ic_stable_structures::Cell;
use std::cell::RefCell;

thread_local! {
    pub static CONFIGURATION_MEMORY: RefCell<Cell<Configuration, Memory>> = RefCell::new(Cell::init(get_memory(CONFIGURATION_MEMORY_ID), Configuration::default()).unwrap());
}
