use crate::models::DaoAssociation;
use crate::services::DaoAssociationService;
use common::types::DaoArgs;
use common::utils::Date;
use ic_cdk::api::time;
use ic_cdk::init;

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
        args.sogc_publications,
        args.members,
        Date::nanoseconds_to_milliseconds(time()),
        args.documents,
        vec![],
    );

    DaoAssociationService::save(dao_association);
}
