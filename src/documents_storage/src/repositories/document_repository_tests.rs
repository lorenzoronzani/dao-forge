use super::*;
use crate::models::Document;
use candid::Principal;

fn create_test_document(id: u32) -> Document {
    Document::new(
        id,
        format!("test_document_{}.pdf", id),
        "application/pdf".to_string(),
        Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
        vec![0x25, 0x50, 0x44, 0x46], // %PDF header
        1640995200000 + id as u64,    // Unique timestamp
    )
}

#[test]
fn test_document_repository_save_and_get() {
    let test_doc = create_test_document(123);

    // Test save
    let saved_doc = DocumentRepository::save(test_doc.clone());

    // Verify save returns the same document
    assert_eq!(saved_doc.id, test_doc.id);
    assert_eq!(saved_doc.name, test_doc.name);
    assert_eq!(saved_doc.content, test_doc.content);

    // Test get
    let retrieved_doc = DocumentRepository::get(123);

    // Verify get returns the saved document
    assert!(retrieved_doc.is_some());
    let retrieved = retrieved_doc.unwrap();
    assert_eq!(retrieved.id, test_doc.id);
    assert_eq!(retrieved.name, test_doc.name);
    assert_eq!(retrieved.owner, test_doc.owner);
    assert_eq!(retrieved.content, test_doc.content);
}

#[test]
fn test_document_repository_get_nonexistent() {
    // Test getting a document that doesn't exist
    let result = DocumentRepository::get(99999);
    assert!(result.is_none());
}

#[test]
fn test_document_repository_save_multiple() {
    let doc1 = create_test_document(200);
    let doc2 = Document::new(
        201,
        "image.png".to_string(),
        "image/png".to_string(),
        Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
        vec![0x89, 0x50, 0x4E, 0x47], // PNG header
        1672531200000,
    );

    // Save multiple documents
    DocumentRepository::save(doc1.clone());
    DocumentRepository::save(doc2.clone());

    // Verify both can be retrieved
    let retrieved1 = DocumentRepository::get(200).unwrap();
    let retrieved2 = DocumentRepository::get(201).unwrap();

    assert_eq!(retrieved1.id, doc1.id);
    assert_eq!(retrieved2.id, doc2.id);
    assert_eq!(retrieved1.content_type, doc1.content_type);
    assert_eq!(retrieved2.content_type, doc2.content_type);
    assert_eq!(retrieved1.owner, doc1.owner);
    assert_eq!(retrieved2.owner, doc2.owner);
}

#[test]
fn test_document_repository_size() {
    // Get initial size
    let initial_size = DocumentRepository::size();

    // Add some documents
    let doc1 = create_test_document(300);
    let doc2 = create_test_document(301);

    DocumentRepository::save(doc1);
    let size_after_one = DocumentRepository::size();
    assert_eq!(size_after_one, initial_size + 1);

    DocumentRepository::save(doc2);
    let size_after_two = DocumentRepository::size();
    assert_eq!(size_after_two, initial_size + 2);
}

#[test]
fn test_document_repository_overwrite() {
    let original_doc = create_test_document(400);

    // Save original document
    DocumentRepository::save(original_doc.clone());

    // Create updated document with same ID but different content
    let updated_doc = Document::new(
        400,
        "updated_document.txt".to_string(),
        "text/plain".to_string(),
        Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
        b"Updated content".to_vec(),
        1704067200000,
    );

    // Save updated document (should overwrite)
    DocumentRepository::save(updated_doc.clone());

    // Verify the document was updated, not duplicated
    let retrieved = DocumentRepository::get(400).unwrap();
    assert_eq!(retrieved.id, 400);
    assert_eq!(retrieved.name, "updated_document.txt");
    assert_eq!(retrieved.content_type, "text/plain");
    assert_eq!(retrieved.content, b"Updated content");
    assert_ne!(retrieved.name, original_doc.name);
    assert_ne!(retrieved.content, original_doc.content);
}

#[test]
fn test_document_repository_with_binary_content() {
    let binary_content = vec![0x00, 0xFF, 0x7F, 0x80, 0x01, 0xFE];
    let doc = Document::new(
        500,
        "binary_file.bin".to_string(),
        "application/octet-stream".to_string(),
        Principal::from_text("ggppn-oqaaa-aaaaa-aaaaq-azy").unwrap(),
        binary_content.clone(),
        1672531200000,
    );

    // Save and retrieve document with binary content
    DocumentRepository::save(doc.clone());
    let retrieved = DocumentRepository::get(500).unwrap();

    // Verify binary content integrity
    assert_eq!(retrieved.content, binary_content);
    assert_eq!(retrieved.content.len(), binary_content.len());
}

#[test]
fn test_document_repository_workflow() {
    let doc_id = 600;
    let doc = create_test_document(doc_id);

    // Initial state: document doesn't exist
    assert!(DocumentRepository::get(doc_id).is_none());

    // Save document
    let saved = DocumentRepository::save(doc.clone());
    assert_eq!(saved.id, doc.id);

    // Verify it exists
    let retrieved = DocumentRepository::get(doc_id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, doc_id);

    // Verify size increased
    assert!(DocumentRepository::size() > 0);
}

#[test]
fn test_document_repository_edge_cases() {
    // Test with ID 0
    let doc_zero = create_test_document(0);
    DocumentRepository::save(doc_zero.clone());
    let retrieved_zero = DocumentRepository::get(0);
    assert!(retrieved_zero.is_some());
    assert_eq!(retrieved_zero.unwrap().id, 0);

    // Test with maximum u32
    let doc_max = create_test_document(u32::MAX);
    DocumentRepository::save(doc_max.clone());
    let retrieved_max = DocumentRepository::get(u32::MAX);
    assert!(retrieved_max.is_some());
    assert_eq!(retrieved_max.unwrap().id, u32::MAX);
}
