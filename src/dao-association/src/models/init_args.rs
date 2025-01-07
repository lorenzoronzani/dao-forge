use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub name: String,
    pub members: Vec<String>,
}
