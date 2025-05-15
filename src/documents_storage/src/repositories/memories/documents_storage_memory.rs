use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

use crate::{
    models::Document,
    repositories::{get_memory, Memory, DOCUMENT_MEMORY_ID},
};

thread_local! {
    pub static DOCUMENT_MEMORY: RefCell<StableBTreeMap<u32, Document, Memory>> = RefCell::new(
        StableBTreeMap::init(
            get_memory(DOCUMENT_MEMORY_ID),
        )
    );
}
