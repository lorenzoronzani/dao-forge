use common::utils::Date;
use ic_cdk::{api::time, caller};

use crate::{
    models::{TimerAction, Voting, VotingState},
    services::{TimerService, VotingService},
    types::VotingArgs,
};

#[ic_cdk::update]
async fn create_voting(params: VotingArgs) -> Voting {
    let voting = VotingService::save(
        params.title,
        params.description,
        params.options,
        caller(),
        Date::nanoseconds_to_milliseconds(time()),
        params.end_at,
        params.dao_id,
        params.approval_threshold,
        params.quorum,
        params.voters_whitelist,
    );

    let _ = TimerService::set_voting_timer(voting.id, params.end_at, TimerAction::EvaluateVoting);

    voting
}

#[ic_cdk::query]
async fn get_voting(id: u32) -> Voting {
    VotingService::get(id).expect("Voting not found")
}

#[ic_cdk::update]
async fn vote(voting_id: u32, option: String) -> () {
    let mut voting = get_voting(voting_id).await;
    if voting.state != VotingState::Open {
        panic!("Voting is not open");
    }

    let user_principal = caller();
    if !voting.voters_whitelist.contains(&user_principal) {
        panic!("User is not allowed to vote");
    }

    let voting_options = voting.options.clone();
    if voting_options.keys().find(|key| **key == option).is_none() {
        panic!("Option is not valid");
    }

    let mut voting_voters = voting.voters_cast.clone();
    if voting_voters.contains_key(&user_principal) {
        panic!("User has already voted");
    }

    let mut voting_result = voting.result.clone();
    voting_result.insert(option.clone(), voting_result.get(&option).unwrap_or(&0) + 1);

    voting_voters.insert(user_principal, option);

    voting.updated_at = Date::nanoseconds_to_milliseconds(time());

    voting.voters_cast = voting_voters;
    voting.result = voting_result;

    let voting = VotingService::update(voting);

    if voting.voters_whitelist.len() == voting.voters_cast.len() {
        VotingService::evaluate_voting(voting_id).await;
    }
}
