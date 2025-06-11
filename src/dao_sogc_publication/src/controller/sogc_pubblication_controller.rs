use crate::models::SogcPublication;
use crate::services::SogcPubblicationService;
use common::types::SogcPublicationArgs;
use common::utils::Date;
use ic_cdk::api::time;
use ic_cdk::{query, update};

#[update]
fn create_sogc_publication(params: SogcPublicationArgs) -> u32 {
    let sogc_publication = SogcPubblicationService::save(
        Date::nanoseconds_to_milliseconds(time()),
        params.daily_number,
        params.publication_date,
        params.mutations,
        params.description,
    );

    sogc_publication.sogc_id
}

#[query]
fn get_sogc_publication(id: u32) -> SogcPublication {
    SogcPubblicationService::get(id).expect("SogcPublication not found")
}
