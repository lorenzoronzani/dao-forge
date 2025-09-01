use std::borrow::Cow;

use candid::{CandidType, Decode, Encode};
use common::types::Mutation;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct SogcPublication {
    pub sogc_id: u32,
    pub publication_sogc_date: u64,
    pub daily_number: u32,
    pub publication_date: u64,
    pub mutations: Vec<Mutation>,
    pub description: String,
}

impl SogcPublication {
    pub fn new(
        sogc_id: u32,
        publication_sogc_date: u64,
        daily_number: u32,
        publication_date: u64,
        mutations: Vec<Mutation>,
        description: String,
    ) -> Self {
        Self {
            sogc_id,
            publication_sogc_date,
            daily_number,
            publication_date,
            mutations,
            description,
        }
    }
}

impl Storable for SogcPublication {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
#[path = "sogc_publication_tests.rs"]
mod sogc_publication_tests;
