use crate::models::Document;

use super::DOCUMENT_MEMORY;

pub struct DocumentRepository;

impl DocumentRepository {
    pub fn save(document: Document) -> Document {
        DOCUMENT_MEMORY.with_borrow_mut(|document_memory| {
            document_memory.insert(document.id, document.clone())
        });

        document
    }

    pub fn get(document_id: u32) -> Option<Document> {
        DOCUMENT_MEMORY.with_borrow(|document_memory| document_memory.get(&document_id))
    }

    pub fn size() -> u64 {
        DOCUMENT_MEMORY.with_borrow(|document_memory| document_memory.len())
    }
}

#[cfg(test)]
#[path = "document_repository_tests.rs"]
mod document_repository_tests;
