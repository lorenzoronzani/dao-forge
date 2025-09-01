use super::*;

fn create_test_configuration() -> Configuration {
    Configuration::new(
        "https://api.courier.com".to_string(),
        "pk_test_1234567890abcdef".to_string(),
        "template_abc123".to_string(),
    )
}

#[test]
fn test_configuration_creation() {
    let config = create_test_configuration();

    // Test basic properties
    assert_eq!(config.courier_url, "https://api.courier.com");
    assert_eq!(config.courier_auth_token, "pk_test_1234567890abcdef");
    assert_eq!(config.template_id, "template_abc123");
}

#[test]
fn test_configuration_new() {
    let url = "https://custom.courier.service".to_string();
    let token = "auth_token_xyz789".to_string();
    let template = "custom_template_456".to_string();

    let config = Configuration::new(url.clone(), token.clone(), template.clone());

    assert_eq!(config.courier_url, url);
    assert_eq!(config.courier_auth_token, token);
    assert_eq!(config.template_id, template);
}

#[test]
fn test_configuration_default() {
    let default_config = Configuration::default();

    // Default should create empty strings
    assert_eq!(default_config.courier_url, "");
    assert_eq!(default_config.courier_auth_token, "");
    assert_eq!(default_config.template_id, "");
}

#[test]
fn test_configuration_storable() {
    let original_config = create_test_configuration();

    // Test serialization
    let bytes = original_config.to_bytes();
    assert!(!bytes.is_empty());

    // Test deserialization
    let deserialized_config = Configuration::from_bytes(bytes);

    // Verify data integrity
    assert_eq!(original_config.courier_url, deserialized_config.courier_url);
    assert_eq!(
        original_config.courier_auth_token,
        deserialized_config.courier_auth_token
    );
    assert_eq!(original_config.template_id, deserialized_config.template_id);
}

#[test]
fn test_configuration_clone() {
    let original_config = create_test_configuration();
    let cloned_config = original_config.clone();

    // Verify clone works correctly
    assert_eq!(original_config.courier_url, cloned_config.courier_url);
    assert_eq!(
        original_config.courier_auth_token,
        cloned_config.courier_auth_token
    );
    assert_eq!(original_config.template_id, cloned_config.template_id);
}

#[test]
fn test_configuration_empty_values() {
    let config = Configuration::new(String::new(), String::new(), String::new());

    assert!(config.courier_url.is_empty());
    assert!(config.courier_auth_token.is_empty());
    assert!(config.template_id.is_empty());
}

#[test]
fn test_configuration_long_values() {
    let long_url = "https://very-long-courier-service-domain-name-for-testing.example.com/api/v2/messages/send".to_string();
    let long_token = "pk_live_very_long_authentication_token_with_many_characters_1234567890abcdefghijklmnopqrstuvwxyz".to_string();
    let long_template =
        "very_descriptive_template_name_for_email_notifications_with_detailed_branding_elements"
            .to_string();

    let config = Configuration::new(long_url.clone(), long_token.clone(), long_template.clone());

    assert_eq!(config.courier_url, long_url);
    assert_eq!(config.courier_auth_token, long_token);
    assert_eq!(config.template_id, long_template);

    // Test serialization works with long strings
    let bytes = config.to_bytes();
    let deserialized = Configuration::from_bytes(bytes);
    assert_eq!(deserialized.courier_url, long_url);
    assert_eq!(deserialized.courier_auth_token, long_token);
    assert_eq!(deserialized.template_id, long_template);
}

#[test]
fn test_configuration_special_characters() {
    let config = Configuration::new(
        "https://api.courier.com/v1?param=value&other=data".to_string(),
        "pk_test_!@#$%^&*()_+-={}|[]\\:;\"'<>?,./".to_string(),
        "template-with-dashes_and_underscores.123".to_string(),
    );

    // Verify special characters are preserved
    assert!(config.courier_url.contains("?param=value&other=data"));
    assert!(config.courier_auth_token.contains("!@#$%^&*()"));
    assert!(config.template_id.contains("-"));
    assert!(config.template_id.contains("_"));
    assert!(config.template_id.contains("."));

    // Test serialization preserves special characters
    let bytes = config.to_bytes();
    let deserialized = Configuration::from_bytes(bytes);
    assert_eq!(config.courier_url, deserialized.courier_url);
    assert_eq!(config.courier_auth_token, deserialized.courier_auth_token);
    assert_eq!(config.template_id, deserialized.template_id);
}

#[test]
fn test_configuration_realistic_values() {
    let config = Configuration::new(
        "https://api.courier.com".to_string(),
        "pk_prod_W1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        "WELCOME_EMAIL_TEMPLATE_V2".to_string(),
    );

    // Test with realistic courier service values
    assert!(config.courier_url.starts_with("https://"));
    assert!(config.courier_auth_token.starts_with("pk_"));
    assert!(config.template_id.contains("TEMPLATE"));
}

#[test]
fn test_configuration_serialization_stability() {
    let config1 = create_test_configuration();
    let config2 = create_test_configuration();

    // Same configurations should serialize to same bytes
    let bytes1 = config1.to_bytes();
    let bytes2 = config2.to_bytes();
    assert_eq!(bytes1, bytes2);

    // Deserialized versions should be equal
    let deserialized1 = Configuration::from_bytes(bytes1);
    let deserialized2 = Configuration::from_bytes(bytes2);
    assert_eq!(deserialized1.courier_url, deserialized2.courier_url);
    assert_eq!(
        deserialized1.courier_auth_token,
        deserialized2.courier_auth_token
    );
    assert_eq!(deserialized1.template_id, deserialized2.template_id);
}
