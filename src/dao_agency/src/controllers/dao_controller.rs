use crate::services::{CanisterManagementService, DaoDiscoveryService};
use crate::types::DaoAssociationInitArgs;
use candid::{encode_args, Principal};
use common::services::SogcPublicationService;
use common::templates::SogcPublicationTemplateManager;
use common::types::{DaoArgs, Mutation};
use common::utils::Date;
use ic_cdk::{api::time, caller, id};
use std::collections::HashMap;

#[ic_cdk::update]
async fn create_dao_association(params: DaoAssociationInitArgs) -> Result<Principal, String> {
    let wasm =
        include_bytes!("../../../../target/wasm32-unknown-unknown/release/dao_association.wasm")
            .to_vec();

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
        board: params.board,
        members: params.members,
        documents: params.documents,
    };

    let template_manager = SogcPublicationTemplateManager::new();
    let sogc_id = SogcPublicationService::publish(
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
                        .board
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                        .clone(),
                ),
                (
                    "members".to_string(),
                    dao_params
                        .members
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                        .clone(),
                ),
            ]),
        )?,
    )
    .await?;
    dao_params.sogc_publications.push(sogc_id);

    if !dao_params.board.contains(&caller()) {
        dao_params.board.push(caller());
    }
    let encoded_args = encode_args((dao_params.clone(),)).unwrap();

    let canister_id =
        CanisterManagementService::deploy_canister(wasm, encoded_args, vec![id(), caller()])
            .await?;

    for board_member in dao_params.board.iter() {
        DaoDiscoveryService::save_user_dao(*board_member, canister_id).await;
    }

    for member in dao_params.members.iter() {
        DaoDiscoveryService::save_user_dao(*member, canister_id).await;
    }

    Ok(canister_id)
}
