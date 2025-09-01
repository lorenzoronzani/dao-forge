use candid::{encode_one, Principal};

use crate::services::InterCanisterService;

pub struct DaoAssociationService;

impl DaoAssociationService {
    pub async fn add_document(dao_principal: Principal, document_id: u32) -> Result<(), String> {
        let args_encoded = encode_one(document_id).map_err(|e| e.to_string())?;

        let _: Result<(), String> =
            InterCanisterService::call_raw(dao_principal, &"add_document", args_encoded).await;

        return Ok(());
    }
}

#[cfg(test)]
#[path = "dao_association_service_tests.rs"]
mod dao_association_service_tests;
