use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct User {
    pub id: Principal,
    pub dao_ids: Vec<u32>,
}

impl User {
    pub fn new(id: Principal) -> Self {
        Self {
            id,
            dao_ids: Vec::new(),
        }
    }

    pub fn add_dao(&mut self, dao_id: u32) {
        if !self.dao_ids.contains(&dao_id) {
            self.dao_ids.push(dao_id);
        }
    }

    pub fn remove_dao(&mut self, dao_id: u32) {
        if self.dao_ids.contains(&dao_id) {
            self.dao_ids.retain(|&x| x != dao_id);
        }
    }
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
#[path = "user_tests.rs"]
mod user_tests;
