use candid::{CandidType, Deserialize};

use super::Mutation;

#[derive(CandidType, Deserialize, Debug)]
pub struct SogcPublicationArgs {
    pub daily_number: u32,
    pub publication_date: u64,
    pub mutations: Vec<Mutation>,
    pub description: String,
}
