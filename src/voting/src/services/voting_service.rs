use std::collections::BTreeMap;

use candid::{ser::IDLBuilder, Principal};
use common::services::InterCanisterService;

use crate::{
    models::{Action, TimerAction, Voting, VotingState},
    repositories::VotingRepository,
};

use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug)]
pub struct EmailArgs {
    pub to: String,
    pub subject: String,
    pub message: String,
}

use super::TimerService;

pub struct VotingService;

impl VotingService {
    fn get_next_id() -> u32 {
        VotingRepository::size() as u32 + 1
    }

    pub fn save(
        title: String,
        description: String,
        options: BTreeMap<String, Option<Action>>,
        owner: Principal,
        created_at: u64,
        end_at: u64,
        dao_id: Principal,
        approval_threshold: u32,
        quorum: u32,
        voters_whitelist: Vec<Principal>,
    ) -> Voting {
        let voting = Voting::new(
            Self::get_next_id(),
            title,
            description,
            options,
            owner,
            created_at,
            end_at,
            VotingState::Open,
            dao_id,
            created_at,
            approval_threshold,
            quorum,
            voters_whitelist,
        );

        VotingRepository::save(voting)
    }

    pub fn get(id: u32) -> Option<Voting> {
        VotingRepository::get(id)
    }

    pub fn update(voting: Voting) -> Voting {
        VotingRepository::save(voting)
    }

    pub async fn evaluate_voting(id: u32) -> () {
        let mut voting = Self::get(id).expect("Voting not found");

        if voting.state != VotingState::Open {
            return;
        }

        voting.state = VotingState::Closed;

        let voting = Self::update(voting);

        if let Some(timer) = TimerService::find_by_voting_id(id, TimerAction::EvaluateVoting) {
            TimerService::clear_timer(timer.id);
        }

        if !Self::check_quorum(&voting) {
            return;
        }

        let winner = Self::retrieve_winner(&voting);

        if !Self::check_approval_threshold(&winner, &voting) {
            return;
        }

        Self::execute_action(&winner, &voting).await;
    }

    fn check_quorum(voting: &Voting) -> bool {
        let total_eligible_voters = voting.voters_whitelist.len() as f64;
        let total_voters = voting.voters_cast.len() as f64;

        (total_voters / total_eligible_voters) * 100.0 >= voting.quorum as f64
    }

    // FIXME: In case of a tie, the last option is returned
    fn retrieve_winner(voting: &Voting) -> String {
        let result = voting.result.clone();

        result
            .iter()
            .max_by_key(|(_, count)| *count)
            .unwrap()
            .0
            .clone()
    }

    fn check_approval_threshold(winner: &String, voting: &Voting) -> bool {
        let total_voters = voting.voters_cast.len() as f64;
        let winner_votes = *voting.result.get(winner).unwrap_or(&0) as f64;

        (winner_votes / total_voters) * 100.0 >= voting.approval_threshold as f64
    }

    async fn execute_action(winner: &String, voting: &Voting) {
        if let Some(Some(action)) = voting.options.get(winner).cloned() {
            let encoded_args = Self::encode_args(action.args.clone());

            let _: Result<(), String> =
                InterCanisterService::call_raw(action.canister_id, &action.method, encoded_args)
                    .await;
        }
    }

    fn encode_args(args: Vec<String>) -> Vec<u8> {
        let mut builder = IDLBuilder::new();

        for arg in args {
            if arg.trim().starts_with('{') && arg.trim().ends_with('}') {
                let candid_record = serde_json::from_str::<EmailArgs>(arg.as_str()).unwrap();

                let _ = builder.arg(&candid_record);
            } else {
                let _ = builder.arg(&arg);
            }
        }

        builder
            .serialize_to_vec()
            .expect("Failed to serialize arguments")
    }
}
