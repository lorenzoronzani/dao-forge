use candid::CandidType;
use serde::Deserialize;

// FIXME: Think a better solution to handle the init args
#[derive(CandidType, Deserialize)]
pub struct DaoAssociationInitArgs {
    pub name: String,
    pub address: String,
    pub zip: u32,
    pub town: String,
    pub uid: String,
    pub ch_id: String,
    pub frc_id: u32,
    pub purpose: String,
    pub board: Vec<String>,
    pub members: Vec<String>,
}
