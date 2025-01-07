use candid::{CandidType, Decode, Encode};
use common::models::{Dao, LegalForm};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// FIXME: Think if the default implementation is the best approach
#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub struct DaoAssociation {
    parent: Dao,
}

impl DaoAssociation {
    pub fn new(name: String, members: Vec<String>, created_at: u64) -> Self {
        Self {
            parent: Dao::new(name, members, LegalForm::Association, created_at),
        }
    }
}

impl Storable for DaoAssociation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
