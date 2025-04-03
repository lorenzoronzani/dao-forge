use crate::services::{CanisterManagementService, DaoDiscoveryService};
use crate::types::DaoAssociationInitArgs;
use candid::{encode_args, Principal};
use ic_cdk::{caller, id};

#[ic_cdk::update]
async fn create_dao_association(params: DaoAssociationInitArgs) -> Result<Principal, String> {
    let wasm =
        include_bytes!("../../../../target/wasm32-unknown-unknown/release/dao_association.wasm")
            .to_vec();

    let encoded_args = encode_args((params,)).unwrap();

    let canister_id =
        CanisterManagementService::deploy_canister(wasm, encoded_args, vec![id(), caller()])
            .await?;

    DaoDiscoveryService::save(caller(), canister_id).await;

    Ok(canister_id)
}
