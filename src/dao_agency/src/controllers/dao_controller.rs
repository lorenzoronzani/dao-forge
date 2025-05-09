use crate::services::{CanisterManagementService, DaoDiscoveryService};
use crate::types::DaoAssociationInitArgs;
use candid::{encode_args, Principal};
use ic_cdk::{caller, id};

#[ic_cdk::update]
async fn create_dao_association(mut params: DaoAssociationInitArgs) -> Result<Principal, String> {
    let wasm =
        include_bytes!("../../../../target/wasm32-unknown-unknown/release/dao_association.wasm")
            .to_vec();

    if !params.board.contains(&caller()) {
        params.board.push(caller());
    }
    let encoded_args = encode_args((params.clone(),)).unwrap();

    let canister_id =
        CanisterManagementService::deploy_canister(wasm, encoded_args, vec![id(), caller()])
            .await?;

    for board_member in params.board.iter() {
        DaoDiscoveryService::save_user_dao(*board_member, canister_id).await;
    }

    for member in params.members.iter() {
        DaoDiscoveryService::save_user_dao(*member, canister_id).await;
    }

    Ok(canister_id)
}
