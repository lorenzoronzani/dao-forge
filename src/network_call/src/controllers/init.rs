use ic_cdk::init;

use crate::{models::Configuration, services::ConfigurationService};

#[init]
async fn canister_init(args: Configuration) {
    ConfigurationService::save(args.courier_url, args.courier_auth_token, args.template_id);
}
