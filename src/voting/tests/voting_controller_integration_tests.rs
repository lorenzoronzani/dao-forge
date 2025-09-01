#[cfg(test)]
mod voting_controller_integration_tests {
    use candid::{decode_one, encode_args, Principal};
    use common::models::Configuration;
    use pocket_ic::PocketIc;
    use std::collections::BTreeMap;

    use voting::{Action, Voting, VotingArgs, VotingState};

    fn setup_canister() -> (PocketIc, Principal) {
        let pic = PocketIc::new();
        let canister_id = pic.create_canister();
        pic.add_cycles(canister_id, 2_000_000_000_000);
        let wasm = std::fs::read("../../target/wasm32-unknown-unknown/release/voting.wasm")
            .expect("read voting.wasm");

        let init_args = create_test_voting_args();
        pic.install_canister(canister_id, wasm, encode_args((init_args,)).unwrap(), None);
        (pic, canister_id)
    }

    fn create_test_voting_args() -> Configuration {
        Configuration::new(
            Some(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()),
            Some(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()),
            Some(Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap()),
            Some(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()),
            Some(Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap()),
            Some(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()),
            Some(Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap()),
        )
    }

    fn yes_no_options() -> BTreeMap<String, Option<Action>> {
        let mut m: BTreeMap<String, Option<Action>> = BTreeMap::new();
        m.insert("Yes".to_string(), None);
        m.insert("No".to_string(), None);
        m
    }

    fn make_args(whitelist: Vec<Principal>, quorum: u32, approval: u32) -> VotingArgs {
        VotingArgs {
            title: "Test".into(),
            description: "Desc".into(),
            options: yes_no_options(),
            end_at: 1_700_000_000_000,
            dao_id: Principal::self_authenticating(b"dao"),
            approval_threshold: approval,
            quorum,
            voters_whitelist: whitelist,
            notification: None,
        }
    }

    #[test]
    fn create_and_get_voting() {
        let (pic, canister_id) = setup_canister();
        let creator = Principal::self_authenticating(b"creator");

        let res = pic.update_call(
            canister_id,
            creator,
            "create_voting",
            encode_args((make_args(vec![creator], 50, 50),)).unwrap(),
        );
        assert!(res.is_ok());
        let created: Voting = decode_one(&res.unwrap()).unwrap();
        assert!(created.id > 0);
        assert_eq!(created.title, "Test");
        assert_eq!(created.state, VotingState::Open);

        let q = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_voting",
            encode_args((created.id,)).unwrap(),
        );
        assert!(q.is_ok());
        let fetched: Voting = decode_one(&q.unwrap()).unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.title, "Test");
    }

    #[test]
    fn vote_flow_happy_path_closes_on_all_votes() {
        let (pic, canister_id) = setup_canister();
        let p1 = Principal::self_authenticating(b"a");
        let p2 = Principal::self_authenticating(b"b");

        let res = pic.update_call(
            canister_id,
            p1,
            "create_voting",
            encode_args((make_args(vec![p1, p2], 100, 50),)).unwrap(),
        );
        let voting: Voting = decode_one(&res.unwrap()).unwrap();

        let _ = pic
            .update_call(
                canister_id,
                p1,
                "vote",
                encode_args((voting.id, "Yes".to_string())).unwrap(),
            )
            .expect("first vote ok");

        let q1 = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_voting",
            encode_args((voting.id,)).unwrap(),
        );
        let after_first: Voting = decode_one(&q1.unwrap()).unwrap();
        assert_eq!(after_first.state, VotingState::Open);
        assert_eq!(*after_first.result.get("Yes").unwrap_or(&0), 1);
        assert_eq!(after_first.voters_cast.len(), 1);

        let _ = pic
            .update_call(
                canister_id,
                p2,
                "vote",
                encode_args((voting.id, "Yes".to_string())).unwrap(),
            )
            .expect("second vote ok");

        let q2 = pic.query_call(
            canister_id,
            Principal::anonymous(),
            "get_voting",
            encode_args((voting.id,)).unwrap(),
        );
        let after_second: Voting = decode_one(&q2.unwrap()).unwrap();
        assert_eq!(after_second.state, VotingState::Closed);
        assert_eq!(*after_second.result.get("Yes").unwrap_or(&0), 2);
    }

    #[test]
    fn vote_rejects_non_whitelisted() {
        let (pic, canister_id) = setup_canister();
        let p1 = Principal::self_authenticating(b"a");
        let p2 = Principal::self_authenticating(b"b");

        let res = pic.update_call(
            canister_id,
            p1,
            "create_voting",
            encode_args((make_args(vec![p1], 50, 50),)).unwrap(),
        );
        let voting: Voting = decode_one(&res.unwrap()).unwrap();

        let r = pic.update_call(
            canister_id,
            p2,
            "vote",
            encode_args((voting.id, "Yes".to_string())).unwrap(),
        );
        assert!(r.is_err());
    }

    #[test]
    fn vote_rejects_invalid_option() {
        let (pic, canister_id) = setup_canister();
        let p1 = Principal::self_authenticating(b"a");

        let res = pic.update_call(
            canister_id,
            p1,
            "create_voting",
            encode_args((make_args(vec![p1], 50, 50),)).unwrap(),
        );
        let voting: Voting = decode_one(&res.unwrap()).unwrap();

        let r = pic.update_call(
            canister_id,
            p1,
            "vote",
            encode_args((voting.id, "Maybe".to_string())).unwrap(),
        );
        assert!(r.is_err());
    }

    #[test]
    fn vote_rejects_double_vote() {
        let (pic, canister_id) = setup_canister();
        let p1 = Principal::self_authenticating(b"a");
        let p2 = Principal::self_authenticating(b"b");

        let res = pic.update_call(
            canister_id,
            p1,
            "create_voting",
            encode_args((make_args(vec![p1, p2], 50, 50),)).unwrap(),
        );
        let voting: Voting = decode_one(&res.unwrap()).unwrap();

        let _ = pic
            .update_call(
                canister_id,
                p1,
                "vote",
                encode_args((voting.id, "Yes".to_string())).unwrap(),
            )
            .expect("first vote ok");

        let r = pic.update_call(
            canister_id,
            p1,
            "vote",
            encode_args((voting.id, "No".to_string())).unwrap(),
        );
        assert!(r.is_err());
    }
}
