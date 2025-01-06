use crate::models::DaoAssociation;
use ic_cdk::query;

#[query]
fn get_information() -> String {
    let dao_association = DaoAssociation::new(
        "DAO Association".to_string(),
        vec!["Hello".to_string()],
        ic_cdk::api::time(),
    );

    format!("{:?}", dao_association)
}
