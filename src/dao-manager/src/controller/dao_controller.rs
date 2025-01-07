use crate::services::CanisterManagementService;
use candid::{encode_args, CandidType, Principal};

// FIXME: Think a better solution to handle the init args
#[derive(CandidType)]
struct InitArgs {
    pub name: String,
    pub members: Vec<String>,
}

#[ic_cdk::update]
async fn create_dao() -> Result<Principal, String> {
    let wasm =
        include_bytes!("../../../../target/wasm32-unknown-unknown/release/dao_association.wasm")
            .to_vec();

    let init_args = InitArgs {
        name: "My DAO".to_string(),
        members: vec!["member1".to_string(), "member2".to_string()],
    };

    let encoded_args = encode_args((init_args,)).unwrap();

    CanisterManagementService::deploy_canister(wasm, encoded_args, vec![ic_cdk::api::id()]).await
}
