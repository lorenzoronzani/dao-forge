use crate::models::DaoAssociationPresentation;
use crate::types::DaoArgs;
use crate::{models::DaoAssociation, services::DaoAssociationService};
use common::models::SogcPublication;
use common::utils::Date;
use ic_cdk::api::time;
use ic_cdk::{println, query, update};

#[query]
fn get_information() -> DaoAssociationPresentation {
    let dao = DaoAssociationService::get();

    DaoAssociationPresentation::from(dao)
}

// #[update]
// fn update_information(dao_args: DaoArgs, sogc_publication: SogcPublication) -> String {
//     let mut dao_association = DaoAssociationService::get();

//     dao_association.parent.name = dao_args.name;
//     dao_association.parent.address = dao_args.address;
//     dao_association.parent.zip = dao_args.zip;
//     dao_association.parent.town = dao_args.town;
//     dao_association.parent.uid = dao_args.uid;
//     dao_association.parent.ch_id = dao_args.ch_id;
//     dao_association.parent.frc_id = dao_args.frc_id;
//     dao_association.parent.purpose = dao_args.purpose;
//     dao_association
//         .parent
//         .sogc_pubblications
//         .push(sogc_publication);
//     dao_association.parent.board = dao_args.board;
//     dao_association.parent.members = dao_args.members;

//     format!("{:?}", DaoAssociationService::update(dao_association))
// }
