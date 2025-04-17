use candid::Principal;

use crate::models::User;
use crate::repositories::memories::USER_MEMORY;

pub struct UserRepository;

impl UserRepository {
    pub fn save(user: User) -> User {
        USER_MEMORY.with_borrow_mut(|user_memory| user_memory.insert(user.id, user.clone()));

        user
    }

    pub fn get(user_id: Principal) -> Option<User> {
        USER_MEMORY.with_borrow(|user_memory| user_memory.get(&user_id))
    }

    pub fn update(user: User) -> User {
        USER_MEMORY.with_borrow_mut(|user_memory| user_memory.insert(user.id, user.clone()));

        user
    }
}
