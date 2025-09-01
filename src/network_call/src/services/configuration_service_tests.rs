use super::*;

#[test]
fn test_save_and_get_configuration_through_service() {
    let courier_url = "https://api.test-courier.com".to_string();
    let courier_auth_token = "auth_token_from_service".to_string();
    let template_id = "template_id_from_service".to_string();

    // Act: Use the service to save the configuration.
    let saved_config = ConfigurationService::save(
        courier_url.clone(),
        courier_auth_token.clone(),
        template_id.clone(),
    );

    // Assert: The service should return the correct configuration.
    assert_eq!(saved_config.courier_url, courier_url);
    assert_eq!(saved_config.courier_auth_token, courier_auth_token);
    assert_eq!(saved_config.template_id, template_id);

    // Act: Now use the service to retrieve the configuration.
    let retrieved_config = ConfigurationService::get();

    // Assert: The retrieved data should be the same as the data that was saved.
    assert_eq!(retrieved_config.courier_url, courier_url);
    assert_eq!(retrieved_config.courier_auth_token, courier_auth_token);
    assert_eq!(retrieved_config.template_id, template_id);
}
