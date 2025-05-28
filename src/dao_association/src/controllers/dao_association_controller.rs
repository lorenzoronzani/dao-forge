use crate::models::DaoAssociationPresentation;
use crate::services::DaoAssociationService;
use ic_cdk::{query, update};

#[query]
fn get_information() -> DaoAssociationPresentation {
    let dao = DaoAssociationService::get();

    DaoAssociationPresentation::from(dao)
}

#[update]
fn add_pool(pool_id: u32) -> DaoAssociationPresentation {
    let mut dao_association = DaoAssociationService::get();

    dao_association.parent.pools.push(pool_id);

    let dao = DaoAssociationService::update(dao_association);

    DaoAssociationPresentation::from(dao)
}

#[update]
fn update_name(name: String) -> DaoAssociationPresentation {
    let mut dao_association = DaoAssociationService::get();

    dao_association.parent.name = name;

    let dao = DaoAssociationService::update(dao_association);

    DaoAssociationPresentation::from(dao)
}

#[update]
fn add_board_member(member_id: String) -> DaoAssociationPresentation {
    let mut dao_association = DaoAssociationService::get();

    dao_association.parent.board.push(member_id);

    let dao = DaoAssociationService::update(dao_association);

    DaoAssociationPresentation::from(dao)
}
