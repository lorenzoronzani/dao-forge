use candid::CandidType;
use serde::{Deserialize, Serialize};

// FIXME: Think if the default implementation is the best approach
#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub enum LegalForm {
    #[default]
    Corporation,
    LimitedLiabilityCompany,
    Association,
}

// FIXME: Think if the default implementation is the best approach
#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub struct Dao {
    pub name: String,
    pub members: Vec<String>,
    pub legal_form: LegalForm,
    pub created_at: u64,
}

impl Dao {
    pub fn new(name: String, members: Vec<String>, legal_form: LegalForm, created_at: u64) -> Self {
        Self {
            name,
            members,
            legal_form,
            created_at,
        }
    }
}
