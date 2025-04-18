use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize)]
pub struct DaoArgs {
    pub name: String,
    pub address: String,
    pub zip: u32,
    pub town: String,
    pub uid: String,
    pub ch_id: String,
    pub frc_id: u32,
    pub purpose: String,
    pub board: Vec<Principal>,
    pub members: Vec<Principal>,
}
