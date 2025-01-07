use crate::models::DaoAssociation;
use crate::services::DaoAssociationService;
use candid::{CandidType, Deserialize};
use ic_cdk::api::time;
use ic_cdk::init;

#[derive(CandidType, Deserialize)]
struct InitArgs {
    name: String,
    members: Vec<String>,
}

#[init]
fn canister_init(args: InitArgs) {
    let dao_association = DaoAssociation::new(args.name, args.members, time());

    DaoAssociationService::save(dao_association);
}
