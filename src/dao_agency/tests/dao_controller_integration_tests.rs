use candid::{decode_one, encode_args, Principal};
use common::models::{Configuration, Role, User};
use dao_agency::DaoAssociationInitArgs;
use ic_management_canister_types::CanisterSettings;
use pocket_ic::PocketIc;

const ADMIN_PRINCIPAL: &str = "vcl2o-aaaaa-aaaaa-aaaaq-aza";

fn new_pic() -> PocketIc {
    PocketIc::new()
}

fn create_controller_canister(pic: &PocketIc, config: Configuration) -> Principal {
    let admin = Principal::from_text(ADMIN_PRINCIPAL).expect("set ADMIN_PRINCIPAL");
    let controllers = vec![admin];

    let canister_id = pic.create_canister_with_settings(
        Some(admin),
        Some(CanisterSettings {
            controllers: Some(controllers),
            ..Default::default()
        }),
    );

    pic.add_cycles(canister_id, 2_000_000_000_000);

    let wasm = std::fs::read("../../target/wasm32-unknown-unknown/release/dao_agency.wasm")
        .expect("read dao_agency.wasm");
    let init = encode_args((config,)).unwrap();
    pic.install_canister(canister_id, wasm, init, Some(admin));

    canister_id
}

fn init_config(sogc: Principal, discovery: Principal) -> Configuration {
    Configuration {
        dao_agency_canister_id: None,
        sogc_publication_canister_id: Some(sogc),
        dao_discovery_canister_id: Some(discovery),
        documents_storage_canister_id: None,
        voting_canister_id: None,
        network_call_canister_id: None,
        dao_platform_canister_id: None,
    }
}

fn create_input() -> DaoAssociationInitArgs {
    DaoAssociationInitArgs {
        name: "Test DAO".into(),
        address: "123 Test St".into(),
        zip: 1234,
        town: "Test Town".into(),
        uid: "uid".into(),
        ch_id: "ch".into(),
        frc_id: 1,
        purpose: "Testing".into(),
        members: vec![User {
            id: Principal::anonymous().to_string(),
            role: Role::Member,
        }],
        documents: vec![],
    }
}

#[test]
fn create_dao_association_returns_err_when_sogc_callee_missing() {
    let pic = new_pic();

    let sogc_canister = pic.create_canister();
    let discovery_canister = pic.create_canister();

    let controller_id =
        create_controller_canister(&pic, init_config(sogc_canister, discovery_canister));

    pic.add_cycles(controller_id, 2_000_000_000_000);

    let args = create_input();
    let res = pic.update_call(
        controller_id,
        Principal::anonymous(),
        "create_dao_association",
        encode_args((args,)).unwrap(),
    );

    assert!(
        res.is_ok(),
        "ingress should succeed even if method returns Err"
    );
    let out: Result<Principal, String> = decode_one(&res.unwrap()).unwrap();
    assert!(out.is_err(), "should bubble up SOGC call failure");
}
