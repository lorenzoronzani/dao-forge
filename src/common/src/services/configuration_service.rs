use candid::Principal;

use crate::{models::Configuration, repositories::ConfigurationRepositoryInterface};

pub struct ConfigurationService<R: ConfigurationRepositoryInterface> {
    configuration_repository: R,
}

impl<R: ConfigurationRepositoryInterface> ConfigurationService<R> {
    pub fn new(configuration_repository: R) -> Self {
        Self {
            configuration_repository,
        }
    }

    pub fn save(
        &self,
        dao_agency_canister_id: Option<Principal>,
        sogc_publication_canister_id: Option<Principal>,
        dao_discovery_canister_id: Option<Principal>,
        documents_storage_canister_id: Option<Principal>,
        voting_canister_id: Option<Principal>,
        network_call_canister_id: Option<Principal>,
        dao_platform_canister_id: Option<Principal>,
    ) -> Configuration {
        let config = Configuration::new(
            dao_agency_canister_id,
            sogc_publication_canister_id,
            dao_discovery_canister_id,
            documents_storage_canister_id,
            voting_canister_id,
            network_call_canister_id,
            dao_platform_canister_id,
        );

        self.configuration_repository.save(config)
    }

    pub fn get(&self) -> Configuration {
        self.configuration_repository.get()
    }

    pub fn update(
        &self,
        dao_agency_canister_id: Option<Principal>,
        sogc_publication_canister_id: Option<Principal>,
        dao_discovery_canister_id: Option<Principal>,
        documents_storage_canister_id: Option<Principal>,
        voting_canister_id: Option<Principal>,
        network_call_canister_id: Option<Principal>,
        dao_platform_canister_id: Option<Principal>,
    ) -> Configuration {
        let mut config = self.get();

        if dao_agency_canister_id.is_some() {
            config.dao_agency_canister_id = dao_agency_canister_id;
        }

        if sogc_publication_canister_id.is_some() {
            config.sogc_publication_canister_id = sogc_publication_canister_id;
        }

        if dao_discovery_canister_id.is_some() {
            config.dao_discovery_canister_id = dao_discovery_canister_id;
        }

        if documents_storage_canister_id.is_some() {
            config.documents_storage_canister_id = documents_storage_canister_id;
        }

        if voting_canister_id.is_some() {
            config.voting_canister_id = voting_canister_id;
        }

        if network_call_canister_id.is_some() {
            config.network_call_canister_id = network_call_canister_id;
        }

        if dao_platform_canister_id.is_some() {
            config.dao_platform_canister_id = dao_platform_canister_id;
        }

        self.configuration_repository.save(config)
    }
}
