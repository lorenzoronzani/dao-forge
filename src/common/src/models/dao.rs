use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::models::User;

// FIXME: Think if the default implementation is the best approach
#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType, PartialEq)]
pub enum LegalForm {
    #[default]
    Corporation,
    LimitedLiabilityCompany,
    Association,
}

// FIXME: Think if the default implementation is the best approach
#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType, PartialEq)]
pub enum OrganizationStatus {
    #[default]
    Active,
    Liquidation,
    Dissolved,
}

// FIXME: Think if the default implementation is the best approach
#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub struct Dao {
    pub name: String,
    pub address: String,
    pub zip: u32,
    pub town: String,
    pub legal_form: LegalForm,
    pub status: OrganizationStatus,
    // Business Unique Identifier
    pub uid: String,
    // Commercial Register Number
    pub ch_id: String,
    // Financial Reporting Council Identifier
    pub frc_id: u32,
    pub purpose: String,
    // Swiss Official Gazette of Commerce Publications
    pub sogc_publications: Vec<u32>,
    pub members: Vec<User>,
    pub created_at: u64,
    pub documents: Vec<u32>,
    pub pools: Vec<u32>,
}

impl Dao {
    pub fn new(
        name: String,
        address: String,
        zip: u32,
        town: String,
        legal_form: LegalForm,
        status: OrganizationStatus,
        uid: String,
        ch_id: String,
        frc_id: u32,
        purpose: String,
        sogc_publications: Vec<u32>,
        members: Vec<User>,
        created_at: u64,
        documents: Vec<u32>,
        pools: Vec<u32>,
    ) -> Self {
        Self {
            name,
            address,
            zip,
            town,
            legal_form,
            status,
            uid,
            ch_id,
            frc_id,
            purpose,
            sogc_publications,
            members,
            created_at,
            documents,
            pools,
        }
    }
}
