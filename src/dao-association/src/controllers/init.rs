use crate::models::DaoAssociation;
use crate::models::InitArgs;
use crate::services::DaoAssociationService;
use ic_cdk::api::time;
use ic_cdk::init;

#[init]
fn canister_init(args: InitArgs) {
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
}
