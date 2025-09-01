#[cfg(test)]
mod document_storage_controller_integration_tests {
    use candid::{decode_one, encode_args, Principal};
    use common::types::DocumentArgs;
    use documents_storage::Document;
    use pocket_ic::PocketIc;

    fn setup_canister() -> (PocketIc, Principal) {
        let pic = PocketIc::new();
        let canister_id = pic.create_canister();
        pic.add_cycles(canister_id, 2_000_000_000_000);

        // Load your compiled WASM - adjust path as needed
        let wasm =
            std::fs::read("../../target/wasm32-unknown-unknown/release/documents_storage.wasm")
                .expect("Could not read WASM file");

        pic.install_canister(canister_id, wasm, vec![], None);

        (pic, canister_id)
    }

    fn create_test_document_args() -> DocumentArgs {
        DocumentArgs {
            name: "test_document.pdf".to_string(),
            content_type: "application/pdf".to_string(),
            content: vec![0x25, 0x50, 0x44, 0x46], // %PDF header bytes
        }
    }

    #[test]
    fn test_store_document() {
        let (pic, canister_id) = setup_canister();
        let doc_args = create_test_document_args();

        // Test store_document method
        let response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_document",
            encode_args((doc_args.clone(),)).unwrap(),
        );

        assert!(response.is_ok());

        let document_id: u32 = decode_one(&response.unwrap()).unwrap();

        // Verify a valid ID was returned
        assert!(document_id > 0);
    }

    #[test]
    fn test_store_and_get_document() {
        let (pic, canister_id) = setup_canister();
        let doc_args = create_test_document_args();

        // Store a document
        let store_response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_document",
            encode_args((doc_args.clone(),)).unwrap(),
        );

        let document_id: u32 = decode_one(&store_response.unwrap()).unwrap();

        // Get the stored document
        let get_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_document",
            encode_args((document_id,)).unwrap(),
        );

        assert!(get_response.is_ok());

        let retrieved_document: Document = decode_one(&get_response.unwrap()).unwrap();

        // Verify the document data
        assert_eq!(retrieved_document.id, document_id);
        assert_eq!(retrieved_document.name, doc_args.name);
        assert_eq!(retrieved_document.content_type, doc_args.content_type);
        assert_eq!(retrieved_document.content, doc_args.content);

        // Verify owner is set to caller (anonymous in this case)
        assert_eq!(retrieved_document.owner, Principal::anonymous());

        // Verify updated_at timestamp was set
        assert!(retrieved_document.updated_at > 0);
    }

    #[test]
    fn test_store_multiple_documents() {
        let (pic, canister_id) = setup_canister();

        // Store first document
        let doc_args1 = DocumentArgs {
            name: "document1.pdf".to_string(),
            content_type: "application/pdf".to_string(),
            content: vec![0x25, 0x50, 0x44, 0x46],
        };

        let response1 = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_document",
            encode_args((doc_args1,)).unwrap(),
        );
        let doc_id1: u32 = decode_one(&response1.unwrap()).unwrap();

        // Store second document
        let doc_args2 = DocumentArgs {
            name: "image.png".to_string(),
            content_type: "image/png".to_string(),
            content: vec![0x89, 0x50, 0x4E, 0x47], // PNG header
        };

        let response2 = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_document",
            encode_args((doc_args2,)).unwrap(),
        );
        let doc_id2: u32 = decode_one(&response2.unwrap()).unwrap();

        // Verify different IDs
        assert_ne!(doc_id1, doc_id2);
        assert!(doc_id2 > doc_id1); // Auto-increment behavior

        // Verify both can be retrieved
        let get_response1 = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_document",
            encode_args((doc_id1,)).unwrap(),
        );
        let document1: Document = decode_one(&get_response1.unwrap()).unwrap();
        assert_eq!(document1.name, "document1.pdf");
        assert_eq!(document1.content_type, "application/pdf");

        let get_response2 = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_document",
            encode_args((doc_id2,)).unwrap(),
        );
        let document2: Document = decode_one(&get_response2.unwrap()).unwrap();
        assert_eq!(document2.name, "image.png");
        assert_eq!(document2.content_type, "image/png");
    }

    #[test]
    fn test_get_nonexistent_document() {
        let (pic, canister_id) = setup_canister();

        // Try to get a document that doesn't exist
        let response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_document",
            encode_args((99999u32,)).unwrap(),
        );

        // Should fail because document doesn't exist (expect panics)
        assert!(response.is_err());
    }

    #[test]
    fn test_store_document_with_empty_content() {
        let (pic, canister_id) = setup_canister();

        let doc_args = DocumentArgs {
            name: "empty_file.txt".to_string(),
            content_type: "text/plain".to_string(),
            content: vec![], // Empty content
        };

        // Store document
        let store_response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_document",
            encode_args((doc_args,)).unwrap(),
        );

        let document_id: u32 = decode_one(&store_response.unwrap()).unwrap();

        // Get and verify
        let get_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_document",
            encode_args((document_id,)).unwrap(),
        );

        let document: Document = decode_one(&get_response.unwrap()).unwrap();
        assert!(document.content.is_empty());
        assert_eq!(document.name, "empty_file.txt");
        assert_eq!(document.content_type, "text/plain");
    }

    #[test]
    fn test_store_document_with_large_content() {
        let (pic, canister_id) = setup_canister();

        let large_content = vec![0x42; 100000]; // 100KB of 'B' bytes
        let doc_args = DocumentArgs {
            name: "large_file.bin".to_string(),
            content_type: "application/octet-stream".to_string(),
            content: large_content.clone(),
        };

        // Store document
        let store_response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_document",
            encode_args((doc_args,)).unwrap(),
        );

        let document_id: u32 = decode_one(&store_response.unwrap()).unwrap();

        // Get and verify large content integrity
        let get_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_document",
            encode_args((document_id,)).unwrap(),
        );

        let document: Document = decode_one(&get_response.unwrap()).unwrap();
        assert_eq!(document.content.len(), 100000);
        assert_eq!(document.content, large_content);
        assert_eq!(document.name, "large_file.bin");
    }

    #[test]
    fn test_store_different_content_types() {
        let (pic, canister_id) = setup_canister();

        // Store JSON document
        let json_args = DocumentArgs {
            name: "data.json".to_string(),
            content_type: "application/json".to_string(),
            content: br#"{"key": "value", "number": 42}"#.to_vec(),
        };

        let json_response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_document",
            encode_args((json_args,)).unwrap(),
        );
        let json_id: u32 = decode_one(&json_response.unwrap()).unwrap();

        // Store CSS document
        let css_args = DocumentArgs {
            name: "styles.css".to_string(),
            content_type: "text/css".to_string(),
            content: b"body { margin: 0; padding: 10px; }".to_vec(),
        };

        let css_response = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_document",
            encode_args((css_args,)).unwrap(),
        );
        let css_id: u32 = decode_one(&css_response.unwrap()).unwrap();

        // Verify both documents
        let json_doc: Document = decode_one(
            &pic.query_call(
                canister_id,
                Principal::anonymous(),
                "get_document",
                encode_args((json_id,)).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();

        let css_doc: Document = decode_one(
            &pic.query_call(
                canister_id,
                Principal::anonymous(),
                "get_document",
                encode_args((css_id,)).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(json_doc.content_type, "application/json");
        assert_eq!(css_doc.content_type, "text/css");
        assert_ne!(json_doc.content, css_doc.content);
    }

    #[test]
    fn test_complete_workflow() {
        let (pic, canister_id) = setup_canister();

        // Step 1: Store first document
        let doc_args1 = DocumentArgs {
            name: "workflow_test1.txt".to_string(),
            content_type: "text/plain".to_string(),
            content: b"First document content".to_vec(),
        };

        let response1 = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_document",
            encode_args((doc_args1.clone(),)).unwrap(),
        );
        let doc_id1: u32 = decode_one(&response1.unwrap()).unwrap();

        // Step 2: Store second document
        let doc_args2 = DocumentArgs {
            name: "workflow_test2.txt".to_string(),
            content_type: "text/plain".to_string(),
            content: b"Second document content".to_vec(),
        };

        let response2 = pic.update_call(
            canister_id,
            Principal::anonymous(),
            "store_document",
            encode_args((doc_args2.clone(),)).unwrap(),
        );
        let doc_id2: u32 = decode_one(&response2.unwrap()).unwrap();

        // Step 3: Verify both documents exist and have correct data
        let get1_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_document",
            encode_args((doc_id1,)).unwrap(),
        );
        let document1: Document = decode_one(&get1_response.unwrap()).unwrap();

        let get2_response = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_document",
            encode_args((doc_id2,)).unwrap(),
        );
        let document2: Document = decode_one(&get2_response.unwrap()).unwrap();

        // Final verifications
        assert_eq!(document1.name, "workflow_test1.txt");
        assert_eq!(document2.name, "workflow_test2.txt");
        assert_eq!(document1.content, b"First document content");
        assert_eq!(document2.content, b"Second document content");
        assert!(document2.id > document1.id); // Auto-increment
        assert_eq!(document1.owner, Principal::anonymous());
        assert_eq!(document2.owner, Principal::anonymous());
    }
}
