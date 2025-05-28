use crate::models::Voting;

use super::VOTING_MEMORY;

pub struct VotingRepository;

impl VotingRepository {
    pub fn save(voting: Voting) -> Voting {
        VOTING_MEMORY.with_borrow_mut(|voting_memory| {
            voting_memory.insert(voting.id, voting.clone());
        });

        voting
    }

    pub fn get(voting_id: u32) -> Option<Voting> {
        VOTING_MEMORY.with_borrow(|voting_memory| voting_memory.get(&voting_id))
    }

    pub fn size() -> u64 {
        VOTING_MEMORY.with_borrow(|voting_memory| voting_memory.len())
    }
}
