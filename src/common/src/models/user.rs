use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum Role {
    #[default]
    Member,
    Board,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub struct User {
    pub id: String,
    pub role: Role,
}

impl User {
    pub fn new(id: String, role: Role) -> Self {
        Self { id, role }
    }
}

#[cfg(test)]
#[path = "user_tests.rs"]
mod user_tests;
