use common::{models::Configuration, services::ConfigurationService};
use ic_cdk::init;

use crate::repositories::ConfigurationRepository;

#[init]
async fn canister_init(args: Configuration) {
    ConfigurationService::new(ConfigurationRepository::new()).save(
        args.dao_agency_canister_id,
        args.sogc_publication_canister_id,
        args.dao_discovery_canister_id,
        args.documents_storage_canister_id,
        args.voting_canister_id,
        args.network_call_canister_id,
    );
}
