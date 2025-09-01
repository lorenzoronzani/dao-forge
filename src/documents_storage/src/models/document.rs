use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Document {
    pub id: u32,
    pub name: String,
    pub content_type: String,
    pub owner: Principal,
    pub content: Vec<u8>,
    pub updated_at: u64,
}

impl Document {
    pub fn new(
        id: u32,
        name: String,
        content_type: String,
        owner: Principal,
        content: Vec<u8>,
        updated_at: u64,
    ) -> Self {
        Self {
            id,
            name,
            content_type,
            owner,
            content,
            updated_at,
        }
    }
}

impl Storable for Document {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
#[path = "document_tests.rs"]
mod document_tests;
