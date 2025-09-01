use super::*;

fn create_test_notification() -> Notification {
    Notification {
        email: "test@example.com".to_string(),
        pdf_bytes: vec![0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x34], // %PDF-1.4 header
    }
}

#[test]
fn test_notification_creation() {
    let notification = create_test_notification();

    // Test basic properties
    assert_eq!(notification.email, "test@example.com");
    assert_eq!(notification.pdf_bytes.len(), 8);
    assert_eq!(notification.pdf_bytes[0..4], [0x25, 0x50, 0x44, 0x46]); // %PDF
}

#[test]
fn test_notification_with_different_values() {
    let email = "user@company.org".to_string();
    let pdf_data = vec![0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x37]; // %PDF-1.7

    let notification = Notification {
        email: email.clone(),
        pdf_bytes: pdf_data.clone(),
    };

    assert_eq!(notification.email, email);
    assert_eq!(notification.pdf_bytes, pdf_data);
}

#[test]
fn test_notification_storable() {
    let original_notification = create_test_notification();

    // Test serialization
    let bytes = original_notification.to_bytes();
    assert!(!bytes.is_empty());

    // Test deserialization
    let deserialized_notification = Notification::from_bytes(bytes);

    // Verify data integrity
    assert_eq!(original_notification.email, deserialized_notification.email);
    assert_eq!(
        original_notification.pdf_bytes,
        deserialized_notification.pdf_bytes
    );
}

#[test]
fn test_notification_clone() {
    let original_notification = create_test_notification();
    let cloned_notification = original_notification.clone();

    // Verify clone works correctly
    assert_eq!(original_notification.email, cloned_notification.email);
    assert_eq!(
        original_notification.pdf_bytes,
        cloned_notification.pdf_bytes
    );
}

#[test]
fn test_notification_empty_pdf() {
    let notification = Notification {
        email: "empty@test.com".to_string(),
        pdf_bytes: vec![],
    };

    assert_eq!(notification.email, "empty@test.com");
    assert!(notification.pdf_bytes.is_empty());
}

#[test]
fn test_notification_large_pdf() {
    let large_pdf = vec![0x42; 100000]; // 100KB of 'B' bytes

    let notification = Notification {
        email: "large@example.com".to_string(),
        pdf_bytes: large_pdf.clone(),
    };

    assert_eq!(notification.pdf_bytes.len(), 100000);
    assert_eq!(notification.pdf_bytes, large_pdf);

    // Test serialization works with large data
    let bytes = notification.to_bytes();
    let deserialized = Notification::from_bytes(bytes);
    assert_eq!(deserialized.pdf_bytes.len(), 100000);
}

#[test]
fn test_notification_different_email_formats() {
    let notifications = vec![
        Notification {
            email: "simple@test.com".to_string(),
            pdf_bytes: vec![0x01],
        },
        Notification {
            email: "user.name+tag@domain.co.uk".to_string(),
            pdf_bytes: vec![0x02],
        },
        Notification {
            email: "test123@sub.domain.org".to_string(),
            pdf_bytes: vec![0x03],
        },
    ];

    assert_eq!(notifications[0].email, "simple@test.com");
    assert_eq!(notifications[1].email, "user.name+tag@domain.co.uk");
    assert_eq!(notifications[2].email, "test123@sub.domain.org");

    // Verify each has unique PDF data
    assert_ne!(notifications[0].pdf_bytes, notifications[1].pdf_bytes);
    assert_ne!(notifications[1].pdf_bytes, notifications[2].pdf_bytes);
}

#[test]
fn test_notification_binary_pdf_data() {
    let binary_pdf = vec![
        0x25, 0x50, 0x44, 0x46, // %PDF header
        0x00, 0xFF, 0x7F, 0x80, // Various byte values
        0x01, 0xFE, 0x02, 0xFD, // More binary data
        0xAA, 0x55, 0xCC, 0x33, // Pattern bytes
    ];

    let notification = Notification {
        email: "binary@test.com".to_string(),
        pdf_bytes: binary_pdf.clone(),
    };

    // Verify binary data integrity
    assert_eq!(notification.pdf_bytes, binary_pdf);
    assert_eq!(notification.pdf_bytes[0], 0x25); // %
    assert_eq!(notification.pdf_bytes[1], 0x50); // P
    assert_eq!(notification.pdf_bytes[4], 0x00); // Null byte
    assert_eq!(notification.pdf_bytes[5], 0xFF); // Max byte
}

#[test]
fn test_notification_complex_serialization() {
    let notification = Notification {
        email: "complex.email+test@long-domain-name.example.org".to_string(),
        pdf_bytes: vec![
            0x25, 0x50, 0x44, 0x46, // PDF header
            0x2D, 0x31, 0x2E, 0x34, // -1.4
            0x0A, 0x25, 0xE2, 0xE3, // PDF content
            0x00, 0xFF, 0x80, 0x7F, // Binary data
        ],
    };

    // Test that complex email and binary data serialize correctly
    let bytes = notification.to_bytes();
    let deserialized = Notification::from_bytes(bytes);

    assert_eq!(notification.email, deserialized.email);
    assert_eq!(notification.pdf_bytes, deserialized.pdf_bytes);
    assert_eq!(deserialized.pdf_bytes.len(), 16);
}

#[test]
fn test_notification_edge_cases() {
    // Test with very long email
    let long_email = format!("{}@example.com", "a".repeat(100));
    let notification = Notification {
        email: long_email.clone(),
        pdf_bytes: vec![0x25, 0x50, 0x44, 0x46],
    };

    assert_eq!(notification.email, long_email);
    assert_eq!(notification.email.len(), 112);

    // Test serialization with long email
    let bytes = notification.to_bytes();
    let deserialized = Notification::from_bytes(bytes);
    assert_eq!(deserialized.email, long_email);
}
