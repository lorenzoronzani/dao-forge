use crate::models::SogcPublication;
use crate::services::SogcPubblicationService;
use ic_cdk::{query, update};

#[update]
fn create_sogc_pubblication(sogc_publication: SogcPublication) -> SogcPublication {
    SogcPubblicationService::save(sogc_publication)
}

#[query]
fn get_sogc_pubblication(id: u32) -> Option<SogcPublication> {
    SogcPubblicationService::get(id)
}
