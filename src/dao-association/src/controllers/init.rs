use crate::models::DaoAssociation;
use crate::services::{DaoAssociationService, DaoDiscoveryRegistryService};
use crate::types::DaoArgs;
use ic_cdk::api::time;
use ic_cdk::{caller, init};

#[init]
async fn canister_init(args: DaoArgs) {
    let dao_association = DaoAssociation::new(
        args.name,
        args.address,
        args.zip,
        args.town,
        args.uid,
        args.ch_id,
        args.frc_id,
        args.purpose,
        args.board,
        args.members,
        time(),
    );

    DaoAssociationService::save(dao_association);

    DaoDiscoveryRegistryService::save(caller()).await;
}
