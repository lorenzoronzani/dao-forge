use common::models::Configuration;
use common::repositories::ConfigurationRepositoryInterface;

use crate::repositories::CONFIGURATION_MEMORY;

pub struct ConfigurationRepository;

impl ConfigurationRepository {
    pub fn new() -> Self {
        Self
    }
}

impl ConfigurationRepositoryInterface for ConfigurationRepository {
    fn save(&self, configuration: Configuration) -> Configuration {
        CONFIGURATION_MEMORY.with_borrow_mut(|configuration_memory| {
            configuration_memory.set(configuration.clone()).unwrap();
        });

        configuration
    }

    fn get(&self) -> Configuration {
        CONFIGURATION_MEMORY.with_borrow(|configuration_memory| configuration_memory.get().clone())
    }
}
