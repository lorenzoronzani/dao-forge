use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, CandidType, PartialEq, Eq)]
pub enum Mutation {
    ChangeOfAddress,
    ChangeOfStatus,
    ChangeOfCompany,
    NewInscription,
}
