use candid::Principal;
use ic_cdk::api::time;

use crate::{models::Dao, repositories::DaoRepository};

pub struct DaoService;

impl DaoService {
    // First version without deletion of entries
    fn get_next_id() -> u32 {
        DaoRepository::size() as u32 + 1
    }

    pub fn save(canister: Principal) -> Dao {
        let dao = Dao::new(Self::get_next_id(), canister);

        DaoRepository::save(dao)
    }

    pub fn get(id: u32) -> Option<Dao> {
        DaoRepository::get(id)
    }

    pub fn get_randoms(amount: u32) -> Vec<Principal> {
        let mut daos = DaoRepository::get_all();
        let size = daos.len() as u32;

        if size <= amount {
            return daos.iter().map(|dao| dao.canister).collect();
        }

        let limit = amount.min(size);
        let mut random_daos = Vec::new();
        let seed = time() / 1000;

        for i in 0..limit {
            let random_index = (seed * (i + 1) as u64) % (daos.len() as u64);
            random_daos.push(daos.remove(random_index as usize).canister);
        }

        return random_daos;
    }

    pub fn find_by_canister(canister: Principal) -> Option<Dao> {
        DaoRepository::get_all()
            .iter()
            .find(|dao| dao.canister == canister)
            .cloned()
    }
}
