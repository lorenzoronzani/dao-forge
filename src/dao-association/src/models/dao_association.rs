use candid::{CandidType, Decode, Encode};
use common::models::{Dao, LegalForm, OrganizationStatus};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// FIXME: Think if the default implementation is the best approach
#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub struct DaoAssociation {
    pub parent: Dao,
}

impl DaoAssociation {
    pub fn new(
        name: String,
        address: String,
        zip: u32,
        town: String,
        uid: String,
        ch_id: String,
        frc_id: u64,
        purpose: String,
        board: Vec<String>,
        members: Vec<String>,
        created_at: u64,
    ) -> Self {
        Self {
            parent: Dao::new(
                name,
                address,
                zip,
                town,
                LegalForm::Association,
                OrganizationStatus::Active,
                uid,
                ch_id,
                frc_id,
                purpose,
                board,
                members,
                created_at,
            ),
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
