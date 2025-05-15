use candid::Principal;

use crate::{models::Document, repositories::DocumentRepository};

pub struct DocumentService;

impl DocumentService {
    // First version without deletion of entries
    fn get_next_id() -> u32 {
        DocumentRepository::size() as u32 + 1
    }

    pub fn save(
        name: String,
        content_type: String,
        owner: Principal,
        content: Vec<u8>,
        updated_at: u64,
    ) -> Document {
        let document = Document::new(
            Self::get_next_id(),
            name,
            content_type,
            owner,
            content,
            updated_at,
        );

        DocumentRepository::save(document)
    }

    pub fn get(id: u32) -> Option<Document> {
        DocumentRepository::get(id)
    }
}
