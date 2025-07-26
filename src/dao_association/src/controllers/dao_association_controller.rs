use crate::repositories::ConfigurationRepository;
use crate::services::DaoAssociationService;
use crate::{models::DaoAssociationPresentation, types::AddressArgs};
use candid::Principal;
use common::services::ConfigurationService;
use common::{
    models::User,
    services::{DaoDiscoveryService, SogcPublicationService},
    templates::SogcPublicationTemplateManager,
    types::Mutation,
    utils::Date,
};
use ic_cdk::{api::time, id, query, update};
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

    let configuration = ConfigurationService::new(ConfigurationRepository::new()).get();

    let template_manager = SogcPublicationTemplateManager::new();
    let sogc_id = SogcPublicationService::publish(
        configuration.sogc_publication_canister_id.unwrap(),
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

#[update]
async fn add_member(user: User) -> DaoAssociationPresentation {
    let mut dao_association = DaoAssociationService::get();

    let configuration = ConfigurationService::new(ConfigurationRepository::new()).get();

    let template_manager = SogcPublicationTemplateManager::new();
    let sogc_id = SogcPublicationService::publish(
        configuration.sogc_publication_canister_id.unwrap(),
        1,
        Date::nanoseconds_to_milliseconds(time()),
        vec![Mutation::ChangeOfCompany],
        template_manager
            .render(
                "dao_member_added",
                HashMap::from([
                    ("date".to_string(), Date::timestamp_to_date(time())),
                    ("name".to_string(), dao_association.parent.name.clone()),
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
                    ("new_member".to_string(), user.id.clone()),
                    ("member_role".to_string(), format!("{:?}", user.role)),
                ]),
            )
            .unwrap(),
    )
    .await
    .unwrap();

    dao_association.parent.members.push(user.clone());
    dao_association.parent.sogc_publications.push(sogc_id);

    let dao = DaoAssociationService::update(dao_association);

    DaoDiscoveryService::save_user_dao(Principal::from_text(user.id.clone()).unwrap(), id()).await;

    DaoAssociationPresentation::from(dao)
}

#[update]
async fn remove_member(user_id: String) -> DaoAssociationPresentation {
    let mut dao_association = DaoAssociationService::get();

    let configuration = ConfigurationService::new(ConfigurationRepository::new()).get();

    let template_manager = SogcPublicationTemplateManager::new();
    let sogc_id = SogcPublicationService::publish(
        configuration.sogc_publication_canister_id.unwrap(),
        1,
        Date::nanoseconds_to_milliseconds(time()),
        vec![Mutation::ChangeOfCompany],
        template_manager
            .render(
                "dao_member_removed",
                HashMap::from([
                    ("date".to_string(), Date::timestamp_to_date(time())),
                    ("name".to_string(), dao_association.parent.name.clone()),
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
                    ("removed_member".to_string(), user_id.clone()),
                    (
                        "member_role".to_string(),
                        format!(
                            "{:?}",
                            dao_association
                                .parent
                                .members
                                .iter()
                                .filter(|m| m.id == user_id.clone())
                                .collect::<Vec<&User>>()[0]
                                .role
                        ),
                    ),
                ]),
            )
            .unwrap(),
    )
    .await
    .unwrap();

    dao_association
        .parent
        .members
        .retain(|user| user.id != user_id);
    dao_association.parent.sogc_publications.push(sogc_id);
    let dao = DaoAssociationService::update(dao_association);

    DaoDiscoveryService::remove_user_dao(Principal::from_text(user_id.clone()).unwrap(), id())
        .await;

    DaoAssociationPresentation::from(dao)
}

#[update]
async fn update_address(address_args: AddressArgs) -> DaoAssociationPresentation {
    let mut dao_association = DaoAssociationService::get();

    let configuration = ConfigurationService::new(ConfigurationRepository::new()).get();

    let template_manager = SogcPublicationTemplateManager::new();
    let sogc_id = SogcPublicationService::publish(
        configuration.sogc_publication_canister_id.unwrap(),
        1,
        Date::nanoseconds_to_milliseconds(time()),
        vec![Mutation::ChangeOfCompany],
        template_manager
            .render(
                "dao_address_updated",
                HashMap::from([
                    ("date".to_string(), Date::timestamp_to_date(time())),
                    ("name".to_string(), dao_association.parent.name.clone()),
                    (
                        "old_address".to_string(),
                        dao_association.parent.address.clone(),
                    ),
                    (
                        "old_zip".to_string(),
                        dao_association.parent.zip.to_string(),
                    ),
                    ("old_town".to_string(), dao_association.parent.town.clone()),
                    ("new_address".to_string(), address_args.address.clone()),
                    ("new_zip".to_string(), address_args.zip.to_string()),
                    ("new_town".to_string(), address_args.town.clone()),
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

    dao_association.parent.address = address_args.address;
    dao_association.parent.zip = address_args.zip;
    dao_association.parent.town = address_args.town;
    dao_association.parent.sogc_publications.push(sogc_id);
    let dao = DaoAssociationService::update(dao_association);

    DaoAssociationPresentation::from(dao)
}
