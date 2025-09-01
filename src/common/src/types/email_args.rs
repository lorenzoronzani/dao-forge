use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EmailArgs {
    pub to: String,
    pub subject: String,
    pub message: String,
    pub action_url: String,
}
