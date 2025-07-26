use crate::models::Configuration;

pub trait ConfigurationRepositoryInterface {
    fn save(&self, configuration: Configuration) -> Configuration;
    fn get(&self) -> Configuration;
}
