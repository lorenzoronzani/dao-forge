use std::collections::BTreeMap;

use candid::{
    types::{
        value::{IDLField, VariantValue},
        Label,
    },
    IDLArgs, IDLValue, Principal,
};
use common::{services::InterCanisterService, utils::Date};
use ic_cdk::api::time;

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
        voting.end_at = Date::nanoseconds_to_milliseconds(time());

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
        let idl_values: Vec<IDLValue> = args
            .iter()
            .map(|arg_str| {
                if arg_str.trim().starts_with('{') && arg_str.trim().ends_with('}') {
                    let json_value: serde_json::Value = serde_json::from_str(arg_str)
                        .expect("Failed to parse argument string as JSON");
                    Self::json_value_to_candid(json_value)
                } else {
                    IDLValue::Text(arg_str.clone())
                }
            })
            .collect();

        let idl_args = IDLArgs::new(&idl_values);

        idl_args
            .to_bytes()
            .expect("Failed to serialize IDLArgs to bytes")
    }

    fn json_value_to_candid(value: serde_json::Value) -> IDLValue {
        match value {
            serde_json::Value::Null => IDLValue::Null,
            serde_json::Value::Bool(b) => IDLValue::Bool(b),
            serde_json::Value::String(s) => {
                // Try to parse as Principal
                if let Ok(p) = Principal::from_text(&s) {
                    return IDLValue::Principal(p);
                }
                // Try to parse as a number
                if let Ok(n) = s.parse::<u32>() {
                    return IDLValue::Nat32(n);
                }
                IDLValue::Text(s)
            }
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    IDLValue::Int(n.as_i64().unwrap().into())
                } else {
                    IDLValue::Float64(n.as_f64().unwrap())
                }
            }
            serde_json::Value::Array(a) => {
                IDLValue::Vec(a.into_iter().map(Self::json_value_to_candid).collect())
            }
            serde_json::Value::Object(o) => {
                if o.len() == 1 {
                    let (key, value) = o.into_iter().next().unwrap();
                    let field = IDLField {
                        id: Label::Named(key.clone()),
                        val: Self::json_value_to_candid(value),
                    };
                    return IDLValue::Variant(VariantValue(Box::new(field), 0));
                }

                let mut fields: Vec<IDLField> = o
                    .into_iter()
                    .map(|(k, v)| IDLField {
                        id: Label::Named(k),
                        val: Self::json_value_to_candid(v),
                    })
                    .collect();

                // Candid records require fields to be sorted by label hash.
                fields.sort_by(|a, b| {
                    let a_id = match &a.id {
                        Label::Id(id) => *id,
                        Label::Named(name) => candid::idl_hash(name),
                        Label::Unnamed(id) => *id,
                    };
                    let b_id = match &b.id {
                        Label::Id(id) => *id,
                        Label::Named(name) => candid::idl_hash(name),
                        Label::Unnamed(id) => *id,
                    };
                    a_id.cmp(&b_id)
                });

                IDLValue::Record(fields)
            }
        }
    }
}
