use candid::Principal;

use crate::{services::InterCanisterService, types::DocumentArgs};

pub struct DocumentsStorageService;

impl DocumentsStorageService {
    pub async fn store_document(
        name: String,
        content_type: String,
        content: Vec<u8>,
    ) -> Result<u32, String> {
        let args = DocumentArgs {
            name,
            content_type,
            content,
        };

        let document_id: Result<u32, String> = InterCanisterService::call(
            Principal::from_text("ueq6w-kyaaa-aaaaa-aaaaq-a2a").unwrap(),
            &"store_document",
            (args,),
        )
        .await;

        return document_id;
    }
}
