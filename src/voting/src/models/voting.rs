use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

use crate::models::Notification;

use super::Action;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum VotingState {
    Open,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Voting {
    pub id: u32,
    pub title: String,
    pub description: String,
    // Key = option name, Value = action or None (if no action)
    pub options: BTreeMap<String, Option<Action>>,
    pub result: HashMap<String, u32>,
    pub owner: Principal,
    pub created_at: u64,
    pub end_at: u64,
    pub state: VotingState,
    pub dao_id: Principal,
    pub updated_at: u64,
    pub approval_threshold: u32,
    pub quorum: u32,
    pub voters_whitelist: Vec<Principal>,
    pub voters_cast: HashMap<Principal, String>,
    pub notification: Option<Notification>,
}

impl Voting {
    pub fn new(
        id: u32,
        title: String,
        description: String,
        options: BTreeMap<String, Option<Action>>,
        owner: Principal,
        created_at: u64,
        end_at: u64,
        state: VotingState,
        dao_id: Principal,
        updated_at: u64,
        approval_threshold: u32,
        quorum: u32,
        voters_whitelist: Vec<Principal>,
        notification: Option<Notification>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            options,
            result: HashMap::new(),
            owner,
            created_at,
            end_at,
            state,
            dao_id,
            updated_at,
            approval_threshold,
            quorum,
            voters_whitelist,
            voters_cast: HashMap::new(),
            notification,
        }
    }
}

impl Storable for Voting {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
#[path = "voting_tests.rs"]
mod voting_tests;
