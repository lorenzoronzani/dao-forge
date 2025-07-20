use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct DocumentArgs {
    pub name: String,
    pub content_type: String,
    pub content: Vec<u8>,
}
