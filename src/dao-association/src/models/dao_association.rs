use common::models::{Dao, LegalForm};

#[derive(Debug)]
pub struct DaoAssociation {
    parent: Dao,
}

impl DaoAssociation {
    pub fn new(name: String, members: Vec<String>, created_at: u64) -> Self {
        Self {
            parent: Dao::new(name, members, LegalForm::Association, created_at),
        }
    }
}