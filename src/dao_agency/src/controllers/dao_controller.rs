use crate::repositories::ConfigurationRepository;
use crate::services::CanisterManagementService;
use crate::types::DaoAssociationInitArgs;
use candid::{encode_args, Principal};
use common::models::{Role, User};
use common::services::{ConfigurationService, DaoDiscoveryService, SogcPublicationService};
use common::templates::SogcPublicationTemplateManager;
use common::types::{DaoArgs, Mutation};
use common::utils::Date;
use ic_cdk::{api::time, caller, id};
use std::collections::HashMap;

static ADMIN_CONTROLLER_START: &str = "wdt2u-xhshh";
static ADMIN_CONTROLLER_END: &str = "7xud4-yqe";

#[ic_cdk::update]
async fn create_dao_association(params: DaoAssociationInitArgs) -> Result<Principal, String> {
    let wasm =
        include_bytes!("../../../../target/wasm32-unknown-unknown/release/dao_association.wasm")
            .to_vec();

    let configuration = ConfigurationService::new(ConfigurationRepository::new()).get();

    let mut dao_params = DaoArgs {
        name: params.name,
        address: params.address,
        zip: params.zip,
        town: params.town,
        uid: params.uid,
        ch_id: params.ch_id,
        frc_id: params.frc_id,
        purpose: params.purpose,
        sogc_publications: vec![],
        members: params.members,
        documents: params.documents,
        configuration: configuration.clone(),
    };

    let template_manager = SogcPublicationTemplateManager::new();
    let sogc_id = SogcPublicationService::publish(
        configuration.sogc_publication_canister_id.unwrap(),
        1,
        Date::nanoseconds_to_milliseconds(time()),
        vec![Mutation::NewInscription],
        template_manager.render(
            "dao_created",
            HashMap::from([
                ("date".to_string(), Date::timestamp_to_date(time())),
                ("name".to_string(), dao_params.name.clone()),
                ("address".to_string(), dao_params.address.clone()),
                ("zip".to_string(), dao_params.zip.to_string()),
                ("town".to_string(), dao_params.town.clone()),
                ("uid".to_string(), dao_params.uid.clone()),
                ("ch_id".to_string(), dao_params.ch_id.clone()),
                ("frc_id".to_string(), dao_params.frc_id.to_string()),
                ("purpose".to_string(), dao_params.purpose.clone()),
                (
                    "board".to_string(),
                    dao_params
                        .members
                        .iter()
                        .filter(|m| m.role == Role::Board)
                        .map(|p| p.id.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                        .clone(),
                ),
                (
                    "members".to_string(),
                    dao_params
                        .members
                        .iter()
                        .filter(|m| m.role == Role::Member)
                        .map(|p| p.id.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                        .clone(),
                ),
            ]),
        )?,
    )
    .await?;
    dao_params.sogc_publications.push(sogc_id);

    let board_count = dao_params
        .members
        .iter()
        .filter(|m| m.id == caller().to_string() && m.role == Role::Board)
        .count();

    if board_count == 0 {
        dao_params.members.push(User {
            id: caller().to_string(),
            role: Role::Board,
        });
    }
    let encoded_args = encode_args((dao_params.clone(),)).unwrap();

    let canister_status = CanisterManagementService::canister_status(id()).await?;

    let admin_controller = canister_status
        .settings
        .controllers
        .into_iter()
        .filter(|c| {
            c.to_string().starts_with(ADMIN_CONTROLLER_START)
                && c.to_string().ends_with(ADMIN_CONTROLLER_END)
        })
        .collect::<Vec<Principal>>()[0];

    // FIXME: I cannot use the caller() properly because my derivate principal is not the same of the NNS one so the caller is not able to access to it
    let canister_id = CanisterManagementService::deploy_canister(
        wasm,
        encoded_args,
        vec![id(), admin_controller],
    )
    .await?;

    let configuration = ConfigurationService::new(ConfigurationRepository::new()).get();

    for member in dao_params.members.iter() {
        DaoDiscoveryService::save_user_dao(
            configuration.dao_discovery_canister_id.unwrap(),
            Principal::from_text(member.id.clone()).unwrap(),
            canister_id,
        )
        .await;
    }

    Ok(canister_id)
}
