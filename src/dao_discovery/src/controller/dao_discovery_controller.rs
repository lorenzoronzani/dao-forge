use candid::Principal;
use ic_cdk::caller;

use crate::services::{DaoService, UserService};

#[ic_cdk::update]
async fn save_user_dao(user_id: Principal, dao_id: Principal) -> Vec<Principal> {
    let dao = DaoService::find_by_canister(dao_id).unwrap_or_else(|| DaoService::save(dao_id));

    let user = UserService::get(user_id);
    let result;

    if let Some(user) = user {
        result = UserService::update(user.id, dao.id, true);
    } else {
        result = UserService::save(user_id, vec![dao.id]);
    }

    get_user_daos(Some(result.id)).await
}

#[ic_cdk::query]
async fn get_user_daos(user: Option<Principal>) -> Vec<Principal> {
    let mut user_daos = Vec::new();

    let user = UserService::get(user.unwrap_or(caller()));

    if let Some(user) = user {
        user.dao_ids.iter().for_each(|dao_id| {
            user_daos.push(DaoService::get(*dao_id).unwrap().canister);
        });
    }

    user_daos
}

#[ic_cdk::query]
async fn get_random_daos(amount: Option<u32>) -> Vec<Principal> {
    DaoService::get_randoms(amount.unwrap_or(6))
}
