use crate::models::DaoAssociationPresentation;
use crate::services::DaoAssociationService;
use common::{
    services::SogcPublicationService, templates::SogcPublicationTemplateManager, types::Mutation,
    utils::Date,
};
use ic_cdk::{api::time, query, update};
use std::collections::HashMap;

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
fn add_sogc_publication(sogc_id: u32) -> DaoAssociationPresentation {
    let mut dao_association = DaoAssociationService::get();

    dao_association.parent.sogc_publications.push(sogc_id);

    let dao = DaoAssociationService::update(dao_association);

    DaoAssociationPresentation::from(dao)
}

#[update]
fn add_document(document_id: u32) -> DaoAssociationPresentation {
    let mut dao_association = DaoAssociationService::get();

    dao_association.parent.documents.push(document_id);

    let dao = DaoAssociationService::update(dao_association);

    DaoAssociationPresentation::from(dao)
}

#[update]
async fn update_name(name: String) -> DaoAssociationPresentation {
    let mut dao_association = DaoAssociationService::get();

    let template_manager = SogcPublicationTemplateManager::new();
    let sogc_id = SogcPublicationService::publish(
        1,
        Date::nanoseconds_to_milliseconds(time()),
        vec![Mutation::ChangeOfCompany],
        template_manager
            .render(
                "dao_name_changed",
                HashMap::from([
                    ("date".to_string(), Date::timestamp_to_date(time())),
                    ("old_name".to_string(), dao_association.parent.name.clone()),
                    ("new_name".to_string(), name.clone()),
                    (
                        "address".to_string(),
                        dao_association.parent.address.clone(),
                    ),
                    ("zip".to_string(), dao_association.parent.zip.to_string()),
                    ("town".to_string(), dao_association.parent.town.clone()),
                    ("uid".to_string(), dao_association.parent.uid.clone()),
                    ("ch_id".to_string(), dao_association.parent.ch_id.clone()),
                    (
                        "frc_id".to_string(),
                        dao_association.parent.frc_id.to_string(),
                    ),
                ]),
            )
            .unwrap(),
    )
    .await
    .unwrap();

    dao_association.parent.name = name;
    dao_association.parent.sogc_publications.push(sogc_id);

    let dao = DaoAssociationService::update(dao_association);

    DaoAssociationPresentation::from(dao)
}
