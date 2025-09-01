use candid::Principal;
use ic_cdk::println;

use crate::{
    services::InterCanisterService,
    types::{Mutation, SogcPublicationArgs},
};

pub struct SogcPublicationService;

impl SogcPublicationService {
    pub async fn publish(
        canister_id: Principal,
        daily_number: u32,
        publication_date: u64,
        mutations: Vec<Mutation>,
        description: String,
    ) -> Result<u32, String> {
        let args = SogcPublicationArgs {
            daily_number,
            publication_date,
            mutations,
            description,
        };

        let sogc_id: Result<u32, String> =
            InterCanisterService::call(canister_id, &"create_sogc_publication", (args,)).await;

        println!("sogc_id: {:?}", sogc_id);

        return sogc_id;
    }
}

#[cfg(test)]
#[path = "sogc_publication_service_tests.rs"]
mod sogc_publication_service_tests;
