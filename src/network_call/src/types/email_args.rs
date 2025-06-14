use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct EmailArgs {
    pub to: String,
    pub subject: String,
    pub message: String,
}
