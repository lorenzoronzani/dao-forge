use super::*;
use crate::models::Configuration;

#[test]
fn test_save_and_get_configuration() {
    let new_config = Configuration {
        courier_url: "https://api.courier.com".to_string(),
        courier_auth_token: "super-secret-token".to_string(),
        template_id: "email_template_123".to_string(),
    };

    // 2. Act: Call the function under test.
    let saved_config = ConfigurationRepository::save(new_config.clone());
    let retrieved_config = ConfigurationRepository::get();

    // 3. Assert: Verify the results.
    // The saved configuration should be returned from the `save` function.
    assert_eq!(saved_config.courier_url, new_config.courier_url);
    assert_eq!(
        saved_config.courier_auth_token,
        new_config.courier_auth_token
    );
    assert_eq!(saved_config.template_id, new_config.template_id);

    // The retrieved configuration should match the one that was saved.
    assert_eq!(retrieved_config.courier_url, new_config.courier_url);
    assert_eq!(
        retrieved_config.courier_auth_token,
        new_config.courier_auth_token
    );
    assert_eq!(retrieved_config.template_id, new_config.template_id);
}

#[test]
fn test_overwrite_configuration() {
    let initial_config = Configuration {
        courier_url: "initial_url".to_string(),
        courier_auth_token: "initial_token".to_string(),
        template_id: "initial_template".to_string(),
    };
    ConfigurationRepository::save(initial_config);

    // Prepare the new configuration to overwrite the old one.
    let updated_config = Configuration {
        courier_url: "updated_url".to_string(),
        courier_auth_token: "updated_token".to_string(),
        template_id: "updated_template".to_string(),
    };

    // 2. Act: Save the new configuration. This should overwrite the old one.
    ConfigurationRepository::save(updated_config.clone());
    let retrieved_config = ConfigurationRepository::get();

    // 3. Assert: The retrieved configuration should be the updated one, not the original.
    assert_eq!(retrieved_config.courier_url, updated_config.courier_url);
    assert_ne!(retrieved_config.courier_url, "initial_url");
}
