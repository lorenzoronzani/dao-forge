enum LegalForm {
    Corporation,
    LimitedLiabilityCompany,
    Association,
}

pub struct Dao {
    pub name: String,
    pub members: Vec<String>,
    pub legal_form: LegalForm,
    pub created_at: u64,
}

pub trait DaoOperations {

}
