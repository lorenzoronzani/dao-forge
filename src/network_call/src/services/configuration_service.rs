use crate::{models::Configuration, repositories::ConfigurationRepository};

pub struct ConfigurationService {}

impl ConfigurationService {
    pub fn save(
        courier_url: String,
        courier_auth_token: String,
        template_id: String,
    ) -> Configuration {
        let config = Configuration::new(courier_url, courier_auth_token, template_id);

        ConfigurationRepository::save(config)
    }

    pub fn get() -> Configuration {
        ConfigurationRepository::get()
    }
}

#[cfg(test)]
#[path = "configuration_service_tests.rs"]
mod configuration_service_tests;
