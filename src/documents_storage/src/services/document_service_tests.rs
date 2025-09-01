use super::*;
use candid::Principal;

fn create_test_principal(id: u8) -> Principal {
    match id {
        1 => Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
        2 => Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
        3 => Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
        4 => Principal::from_text("ggppn-oqaaa-aaaaa-aaaaq-azy").unwrap(),
        5 => Principal::from_text("ueq6w-kyaaa-aaaaa-aaaaq-a2a").unwrap(),
        _ => Principal::anonymous(),
    }
}

#[test]
fn test_document_service_save() {
    let name = "test_document.pdf".to_string();
    let content_type = "application/pdf".to_string();
    let owner = create_test_principal(1);
    let content = vec![0x25, 0x50, 0x44, 0x46]; // %PDF header
    let timestamp = 1640995200000;

    // Test save method
    let saved_doc = DocumentService::save(
        name.clone(),
        content_type.clone(),
        owner,
        content.clone(),
        timestamp,
    );

    // Verify document was created with auto-generated ID
    assert!(saved_doc.id > 0);
    assert_eq!(saved_doc.name, name);
    assert_eq!(saved_doc.content_type, content_type);
    assert_eq!(saved_doc.owner, owner);
    assert_eq!(saved_doc.content, content);
    assert_eq!(saved_doc.updated_at, timestamp);
}

#[test]
fn test_document_service_save_and_get() {
    let name = "image.png".to_string();
    let content_type = "image/png".to_string();
    let owner = create_test_principal(2);
    let content = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
    let timestamp = 1672531200000;

    // Save a document
    let saved_doc = DocumentService::save(
        name.clone(),
        content_type.clone(),
        owner,
        content.clone(),
        timestamp,
    );
    let doc_id = saved_doc.id;

    // Get the document by ID
    let retrieved_doc = DocumentService::get(doc_id);

    // Verify get returns the saved document
    assert!(retrieved_doc.is_some());
    let retrieved = retrieved_doc.unwrap();
    assert_eq!(retrieved.id, doc_id);
    assert_eq!(retrieved.name, name);
    assert_eq!(retrieved.content_type, content_type);
    assert_eq!(retrieved.owner, owner);
    assert_eq!(retrieved.content, content);
    assert_eq!(retrieved.updated_at, timestamp);
}

#[test]
fn test_document_service_get_nonexistent() {
    // Test getting a document that doesn't exist
    let result = DocumentService::get(99999);
    assert!(result.is_none());
}

#[test]
fn test_document_service_auto_increment_ids() {
    let owner = create_test_principal(3);
    let content = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"

    // Save multiple documents
    let doc1 = DocumentService::save(
        "doc1.txt".to_string(),
        "text/plain".to_string(),
        owner,
        content.clone(),
        1640995200000,
    );

    let doc2 = DocumentService::save(
        "doc2.txt".to_string(),
        "text/plain".to_string(),
        owner,
        content.clone(),
        1672531200000,
    );

    let doc3 = DocumentService::save(
        "doc3.txt".to_string(),
        "text/plain".to_string(),
        owner,
        content,
        1704067200000,
    );

    // Verify IDs are auto-incremented
    assert!(doc2.id > doc1.id);
    assert!(doc3.id > doc2.id);
    assert_eq!(doc1.name, "doc1.txt");
    assert_eq!(doc2.name, "doc2.txt");
    assert_eq!(doc3.name, "doc3.txt");
}

#[test]
fn test_document_service_different_owners() {
    let owner1 = create_test_principal(1);
    let owner2 = create_test_principal(2);
    let content = b"Shared content".to_vec();

    let doc1 = DocumentService::save(
        "owner1_file.txt".to_string(),
        "text/plain".to_string(),
        owner1,
        content.clone(),
        1640995200000,
    );

    let doc2 = DocumentService::save(
        "owner2_file.txt".to_string(),
        "text/plain".to_string(),
        owner2,
        content,
        1672531200000,
    );

    assert_eq!(doc1.owner, owner1);
    assert_eq!(doc2.owner, owner2);
    assert_ne!(doc1.owner, doc2.owner);
    assert_ne!(doc1.id, doc2.id);
}

#[test]
fn test_document_service_empty_content() {
    let doc = DocumentService::save(
        "empty.txt".to_string(),
        "text/plain".to_string(),
        create_test_principal(4),
        vec![], // Empty content
        1672531200000,
    );

    assert!(doc.content.is_empty());
    assert_eq!(doc.name, "empty.txt");
    assert!(doc.id > 0);
}

#[test]
fn test_document_service_large_content() {
    let large_content = vec![0x42; 50000]; // 50KB of 'B' bytes

    let doc = DocumentService::save(
        "large_file.bin".to_string(),
        "application/octet-stream".to_string(),
        create_test_principal(5),
        large_content.clone(),
        1672617600000,
    );

    assert_eq!(doc.content.len(), 50000);
    assert_eq!(doc.content, large_content);

    // Verify it can be retrieved
    let retrieved = DocumentService::get(doc.id).unwrap();
    assert_eq!(retrieved.content.len(), 50000);
}

#[test]
fn test_document_service_different_content_types() {
    let owner = create_test_principal(1);

    let pdf_doc = DocumentService::save(
        "document.pdf".to_string(),
        "application/pdf".to_string(),
        owner,
        vec![0x25, 0x50, 0x44, 0x46],
        1640995200000,
    );

    let json_doc = DocumentService::save(
        "data.json".to_string(),
        "application/json".to_string(),
        owner,
        br#"{"key": "value"}"#.to_vec(),
        1672531200000,
    );

    assert_eq!(pdf_doc.content_type, "application/pdf");
    assert_eq!(json_doc.content_type, "application/json");
    assert_ne!(pdf_doc.content, json_doc.content);
}

#[test]
fn test_document_service_workflow() {
    let name = "workflow_test.txt".to_string();
    let content_type = "text/plain".to_string();
    let owner = create_test_principal(2);
    let content = b"Workflow test content".to_vec();
    let timestamp = 1640995200000;

    // Complete workflow: save -> get
    let saved_doc = DocumentService::save(
        name.clone(),
        content_type.clone(),
        owner,
        content.clone(),
        timestamp,
    );
    let doc_id = saved_doc.id;

    // Verify can get by ID
    let by_id = DocumentService::get(doc_id);
    assert!(by_id.is_some());
    assert_eq!(by_id.unwrap().name, name);

    // Verify auto-generated ID is positive
    assert!(doc_id > 0);
}

#[test]
fn test_document_service_edge_values() {
    let owner = create_test_principal(3);

    // Test with zero timestamp
    let doc_zero = DocumentService::save(
        "zero_timestamp.txt".to_string(),
        "text/plain".to_string(),
        owner,
        b"Content".to_vec(),
        0,
    );

    assert_eq!(doc_zero.updated_at, 0);
    assert!(doc_zero.id > 0); // ID should still be auto-generated

    // Test with maximum timestamp
    let doc_max = DocumentService::save(
        "max_timestamp.txt".to_string(),
        "text/plain".to_string(),
        owner,
        b"Content".to_vec(),
        u64::MAX,
    );

    assert_eq!(doc_max.updated_at, u64::MAX);
}
