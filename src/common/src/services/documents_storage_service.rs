use candid::Principal;

use crate::{services::InterCanisterService, types::DocumentArgs};

pub struct DocumentsStorageService;

impl DocumentsStorageService {
    pub async fn store_document(
        canister_id: Principal,
        name: String,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<u32, String> {
        let args = DocumentArgs {
            name,
            content_type,
            content,
        };

        let document_id: Result<u32, String> =
            InterCanisterService::call(canister_id, &"store_document", (args,)).await;

        return document_id;
    }
}

#[cfg(test)]
#[path = "documents_storage_service_tests.rs"]
mod documents_storage_service_tests;
