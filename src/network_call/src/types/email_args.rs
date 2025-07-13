use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug)]
pub struct EmailArgs {
    pub to: String,
    pub subject: String,
    pub message: String,
}
