use crate::services::DaoAssociationService;
use ic_cdk::query;

#[query]
fn get_information() -> String {
    let dao_association = DaoAssociationService::get();

    format!("{:?}", dao_association)
}
