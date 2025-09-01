use crate::{models::Configuration, repositories::CONFIGURATION_MEMORY};

pub struct ConfigurationRepository;

impl ConfigurationRepository {
    pub fn save(configuration: Configuration) -> Configuration {
        CONFIGURATION_MEMORY.with_borrow_mut(|configuration_memory| {
            configuration_memory.set(configuration.clone()).unwrap();
        });

        configuration
    }

    pub fn get() -> Configuration {
        CONFIGURATION_MEMORY.with_borrow(|configuration_memory| configuration_memory.get().clone())
    }
}

#[cfg(test)]
#[path = "configuration_repository_tests.rs"]
mod configuration_repository_tests;
