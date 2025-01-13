use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub name: String,
    pub address: String,
    pub zip: u32,
    pub town: String,
    pub uid: String,
    pub ch_id: String,
    pub frc_id: u64,
    pub purpose: String,
    pub board: Vec<String>,
    pub members: Vec<String>,
}
