use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct AddressArgs {
    pub address: String,
    pub zip: u32,
    pub town: String,
}
