use crate::services::CanisterManagementService;
use crate::types::DaoAssociationInitArgs;
use candid::{encode_args, Principal};

#[ic_cdk::update]
async fn create_dao_association(params: DaoAssociationInitArgs) -> Result<Principal, String> {
    let wasm =
        include_bytes!("../../../../target/wasm32-unknown-unknown/release/dao_association.wasm")
            .to_vec();

    let encoded_args = encode_args((params,)).unwrap();

    CanisterManagementService::deploy_canister(wasm, encoded_args, vec![ic_cdk::api::id()]).await
}
