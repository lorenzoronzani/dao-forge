use crate::services::CanisterManagementService;
use candid::{encode_args, CandidType, Principal};

// FIXME: Think a better solution to handle the init args
#[derive(CandidType)]
struct InitArgs {
    pub name: String,
    pub address: String,
    pub zip: u32,
    pub town: String,
    pub uid: String,
    pub ch_id: String,
    pub frc_id: u64,
    pub purpose: String,
    pub board: Vec<String>,
    pub members: Vec<String>,
}

#[ic_cdk::update]
async fn create_dao() -> Result<Principal, String> {
    let wasm =
        include_bytes!("../../../../target/wasm32-unknown-unknown/release/dao_association.wasm")
            .to_vec();

    let init_args = InitArgs {
        name: "My DAO".to_string(),
        address: "My Address".to_string(),
        zip: 12345,
        town: "My Town".to_string(),
        uid: "My UID".to_string(),
        ch_id: "My CH ID".to_string(),
        frc_id: 123,
        purpose: "My Purpose".to_string(),
        board: vec!["board1".to_string(), "board2".to_string()],
        members: vec!["member1".to_string(), "member2".to_string()],
    };

    let encoded_args = encode_args((init_args,)).unwrap();

    CanisterManagementService::deploy_canister(wasm, encoded_args, vec![ic_cdk::api::id()]).await
}
