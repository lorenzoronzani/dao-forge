use candid::{CandidType, Decode, Encode};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub struct Configuration {
    pub courier_url: String,
    pub courier_auth_token: String,
    pub template_id: String,
}

impl Configuration {
    pub fn new(courier_url: String, courier_auth_token: String, template_id: String) -> Self {
        Self {
            courier_url,
            courier_auth_token,
            template_id,
        }
    }
}

impl Storable for Configuration {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
