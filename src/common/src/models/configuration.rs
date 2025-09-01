use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub struct Configuration {
    pub dao_agency_canister_id: Option<Principal>,
    pub sogc_publication_canister_id: Option<Principal>,
    pub dao_discovery_canister_id: Option<Principal>,
    pub documents_storage_canister_id: Option<Principal>,
    pub voting_canister_id: Option<Principal>,
    pub network_call_canister_id: Option<Principal>,
    pub dao_platform_canister_id: Option<Principal>,
}

impl Configuration {
    pub fn new(
        dao_agency_canister_id: Option<Principal>,
        sogc_publication_canister_id: Option<Principal>,
        dao_discovery_canister_id: Option<Principal>,
        documents_storage_canister_id: Option<Principal>,
        voting_canister_id: Option<Principal>,
        network_call_canister_id: Option<Principal>,
        dao_platform_canister_id: Option<Principal>,
    ) -> Self {
        Self {
            dao_agency_canister_id,
            sogc_publication_canister_id,
            dao_discovery_canister_id,
            documents_storage_canister_id,
            voting_canister_id,
            network_call_canister_id,
            dao_platform_canister_id,
        }
    }
}

impl Storable for Configuration {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[cfg(test)]
#[path = "configuration_tests.rs"]
mod configuration_tests;
