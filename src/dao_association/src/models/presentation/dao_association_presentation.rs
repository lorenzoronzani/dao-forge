use candid::CandidType;
use common::models::{LegalForm, OrganizationStatus, User};
use serde::{Deserialize, Serialize};

use crate::models::DaoAssociation;

#[derive(Debug, Default, Clone, Serialize, Deserialize, CandidType)]
pub struct DaoAssociationPresentation {
    pub name: String,
    pub address: String,
    pub zip: u32,
    pub town: String,
    pub legal_form: LegalForm,
    pub status: OrganizationStatus,
    pub uid: String,
    pub ch_id: String,
    pub frc_id: u32,
    pub purpose: String,
    pub sogc_publications: Vec<u32>,
    pub members: Vec<User>,
    pub created_at: u64,
    pub documents: Vec<u32>,
    pub pools: Vec<u32>,
}

impl DaoAssociationPresentation {
    pub fn from(dao_association: DaoAssociation) -> Self {
        Self {
            name: dao_association.parent.name,
            address: dao_association.parent.address,
            zip: dao_association.parent.zip,
            town: dao_association.parent.town,
            legal_form: dao_association.parent.legal_form,
            status: dao_association.parent.status,
            uid: dao_association.parent.uid,
            ch_id: dao_association.parent.ch_id,
            frc_id: dao_association.parent.frc_id,
            purpose: dao_association.parent.purpose,
            sogc_publications: dao_association.parent.sogc_publications,
            members: dao_association.parent.members,
            created_at: dao_association.parent.created_at,
            documents: dao_association.parent.documents,
            pools: dao_association.parent.pools,
        }
    }
}
