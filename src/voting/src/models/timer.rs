use candid::{CandidType, Decode, Encode};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum TimerAction {
    EvaluateVoting,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct Timer {
    pub id: u32,
    pub voting_id: u32,
    pub end_at: u64,
    pub action: TimerAction,
}

impl Timer {
    pub fn new(id: u32, voting_id: u32, end_at: u64, action: TimerAction) -> Self {
        Self {
            id,
            voting_id,
            end_at,
            action,
        }
    }
}

impl Storable for Timer {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
