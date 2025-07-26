use candid::{CandidType, Deserialize};

use crate::models::{Configuration, User};

#[derive(CandidType, Deserialize, Clone)]
pub struct DaoArgs {
    pub name: String,
    pub address: String,
    pub zip: u32,
    pub town: String,
    pub uid: String,
    pub ch_id: String,
    pub frc_id: u32,
    pub purpose: String,
    pub sogc_publications: Vec<u32>,
    pub members: Vec<User>,
    pub documents: Vec<u32>,
    pub configuration: Configuration,
}
