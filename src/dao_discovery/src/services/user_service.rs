use candid::Principal;

use crate::{models::User, repositories::UserRepository};

pub struct UserService;

impl UserService {
    pub fn save(id: Principal, dao_ids: Vec<u32>) -> User {
        let mut user = User::new(id);
        user.dao_ids = dao_ids;

        UserRepository::save(user)
    }

    pub fn get(id: Principal) -> Option<User> {
        UserRepository::get(id)
    }

    pub fn update(id: Principal, dao_id: u32, add: bool) -> User {
        let mut user = Self::get(id).unwrap();

        if add {
            user.add_dao(dao_id);
        } else {
            user.remove_dao(dao_id);
        }

        UserRepository::update(user)
    }
}
