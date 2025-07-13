use candid::CandidType;
use common::models::User;
use serde::Deserialize;

// FIXME: Think a better solution to handle the init args
#[derive(CandidType, Deserialize, Clone)]
pub struct DaoAssociationInitArgs {
    pub name: String,
    pub address: String,
    pub zip: u32,
    pub town: String,
    pub uid: String,
    pub ch_id: String,
    pub frc_id: u32,
    pub purpose: String,
    pub members: Vec<User>,
    pub documents: Vec<u32>,
}
