use super::*;
use candid::Principal;

fn create_test_document() -> Document {
    Document::new(
        123,
        "test_document.pdf".to_string(),
        "application/pdf".to_string(),
        Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
        vec![0x25, 0x50, 0x44, 0x46], // %PDF header bytes
        1640995200000,                // 2022-01-01 timestamp
    )
}

#[test]
fn test_document_creation() {
    let doc = create_test_document();

    // Test basic properties
    assert_eq!(doc.id, 123);
    assert_eq!(doc.name, "test_document.pdf");
    assert_eq!(doc.content_type, "application/pdf");
    assert_eq!(
        doc.owner,
        Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap()
    );
    assert_eq!(doc.content, vec![0x25, 0x50, 0x44, 0x46]);
    assert_eq!(doc.updated_at, 1640995200000);
}

#[test]
fn test_document_new() {
    let id = 456;
    let name = "image.png".to_string();
    let content_type = "image/png".to_string();
    let owner = Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap();
    let content = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header bytes
    let timestamp = 1672531200000;

    let doc = Document::new(
        id,
        name.clone(),
        content_type.clone(),
        owner,
        content.clone(),
        timestamp,
    );

    assert_eq!(doc.id, id);
    assert_eq!(doc.name, name);
    assert_eq!(doc.content_type, content_type);
    assert_eq!(doc.owner, owner);
    assert_eq!(doc.content, content);
    assert_eq!(doc.updated_at, timestamp);
}

#[test]
fn test_document_storable() {
    let original_doc = create_test_document();

    // Test serialization
    let bytes = original_doc.to_bytes();
    assert!(!bytes.is_empty());

    // Test deserialization
    let deserialized_doc = Document::from_bytes(bytes);

    // Verify data integrity
    assert_eq!(original_doc.id, deserialized_doc.id);
    assert_eq!(original_doc.name, deserialized_doc.name);
    assert_eq!(original_doc.content_type, deserialized_doc.content_type);
    assert_eq!(original_doc.owner, deserialized_doc.owner);
    assert_eq!(original_doc.content, deserialized_doc.content);
    assert_eq!(original_doc.updated_at, deserialized_doc.updated_at);
}

#[test]
fn test_document_clone() {
    let original_doc = create_test_document();
    let cloned_doc = original_doc.clone();

    // Verify clone works correctly
    assert_eq!(original_doc.id, cloned_doc.id);
    assert_eq!(original_doc.name, cloned_doc.name);
    assert_eq!(original_doc.content_type, cloned_doc.content_type);
    assert_eq!(original_doc.owner, cloned_doc.owner);
    assert_eq!(original_doc.content, cloned_doc.content);
    assert_eq!(original_doc.updated_at, cloned_doc.updated_at);
}

#[test]
fn test_document_empty_content() {
    let doc = Document::new(
        789,
        "empty.txt".to_string(),
        "text/plain".to_string(),
        Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
        vec![], // Empty content
        1672531200000,
    );

    assert_eq!(doc.id, 789);
    assert!(doc.content.is_empty());
    assert_eq!(doc.name, "empty.txt");
}

#[test]
fn test_document_large_content() {
    let large_content = vec![0x42; 10000]; // 10KB of 'B' bytes

    let doc = Document::new(
        999,
        "large_file.bin".to_string(),
        "application/octet-stream".to_string(),
        Principal::from_text("ggppn-oqaaa-aaaaa-aaaaq-azy").unwrap(),
        large_content.clone(),
        1672617600000,
    );

    assert_eq!(doc.content.len(), 10000);
    assert_eq!(doc.content, large_content);

    // Test serialization works with large content
    let bytes = doc.to_bytes();
    let deserialized = Document::from_bytes(bytes);
    assert_eq!(deserialized.content.len(), 10000);
}

#[test]
fn test_document_different_content_types() {
    let doc1 = Document::new(
        1001,
        "document.json".to_string(),
        "application/json".to_string(),
        Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
        br#"{"key": "value"}"#.to_vec(),
        1640995200000,
    );

    let doc2 = Document::new(
        1002,
        "stylesheet.css".to_string(),
        "text/css".to_string(),
        Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap(),
        b"body { margin: 0; }".to_vec(),
        1641081600000,
    );

    assert_eq!(doc1.content_type, "application/json");
    assert_eq!(doc2.content_type, "text/css");
    assert_ne!(doc1.id, doc2.id);
    assert_ne!(doc1.content, doc2.content);
}

#[test]
fn test_document_different_owners() {
    let owner1 = Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap();
    let owner2 = Principal::from_text("36ijp-fqaaa-aaaaa-aaaaq-azi").unwrap();

    let doc1 = Document::new(
        2001,
        "file1.txt".to_string(),
        "text/plain".to_string(),
        owner1,
        b"Content 1".to_vec(),
        1640995200000,
    );

    let doc2 = Document::new(
        2002,
        "file2.txt".to_string(),
        "text/plain".to_string(),
        owner2,
        b"Content 2".to_vec(),
        1641081600000,
    );

    assert_eq!(doc1.owner, owner1);
    assert_eq!(doc2.owner, owner2);
    assert_ne!(doc1.owner, doc2.owner);
}

#[test]
fn test_document_edge_cases() {
    // Test with ID 0
    let doc_zero = Document::new(
        0,
        "zero.txt".to_string(),
        "text/plain".to_string(),
        Principal::anonymous(),
        vec![0],
        0,
    );

    assert_eq!(doc_zero.id, 0);
    assert_eq!(doc_zero.updated_at, 0);

    // Test with maximum values
    let doc_max = Document::new(
        u32::MAX,
        "max_values.txt".to_string(),
        "text/plain".to_string(),
        Principal::from_text("vcl2o-aaaaa-aaaaa-aaaaq-aza").unwrap(),
        vec![u8::MAX; 100],
        u64::MAX,
    );

    assert_eq!(doc_max.id, u32::MAX);
    assert_eq!(doc_max.updated_at, u64::MAX);
    assert_eq!(doc_max.content.len(), 100);
    assert!(doc_max.content.iter().all(|&b| b == u8::MAX));
}

#[test]
fn test_document_serialization_with_binary_content() {
    // Test with various binary content types
    let binary_data = vec![
        0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD, 0xFC, 0x7F, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85,
        0x86,
    ];

    let doc = Document::new(
        3000,
        "binary_file.dat".to_string(),
        "application/octet-stream".to_string(),
        Principal::from_text("i2m4m-laaaa-aaaaa-aaaaq-azq").unwrap(),
        binary_data.clone(),
        1672531200000,
    );

    // Test that binary data serializes and deserializes correctly
    let bytes = doc.to_bytes();
    let deserialized = Document::from_bytes(bytes);

    assert_eq!(doc.content, deserialized.content);
    assert_eq!(deserialized.content, binary_data);
}
