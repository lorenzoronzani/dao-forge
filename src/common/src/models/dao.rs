#[derive(Debug)]
pub enum LegalForm {
    Corporation,
    LimitedLiabilityCompany,
    Association,
}

#[derive(Debug)]
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
