use std::collections::BTreeMap;

use candid::{CandidType, Deserialize, Principal};

use crate::models::Action;

#[derive(CandidType, Deserialize)]
pub struct VotingArgs {
    pub title: String,
    pub description: String,
    pub options: BTreeMap<String, Option<Action>>,
    pub end_at: u64,
    pub dao_id: Principal,
    pub approval_threshold: u32,
    pub quorum: u32,
    pub voters_whitelist: Vec<Principal>,
}
