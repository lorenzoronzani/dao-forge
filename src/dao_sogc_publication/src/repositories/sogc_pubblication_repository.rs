use crate::models::SogcPublication;
use crate::repositories::SOGC_PUBBLICATION_MEMORY;

pub struct SogcPubblicationRepository;

impl SogcPubblicationRepository {
    pub fn save(sogc_publication: SogcPublication) -> SogcPublication {
        SOGC_PUBBLICATION_MEMORY.with_borrow_mut(|sogc_publication_memory| {
            sogc_publication_memory.insert(sogc_publication.sogc_id, sogc_publication.clone())
        });

        sogc_publication
    }

    pub fn get(sogc_id: u32) -> Option<SogcPublication> {
        SOGC_PUBBLICATION_MEMORY
            .with_borrow(|sogc_publication_memory| sogc_publication_memory.get(&sogc_id).clone())
    }

    pub fn size() -> u64 {
        SOGC_PUBBLICATION_MEMORY
            .with_borrow(|sogc_publication_memory| sogc_publication_memory.len())
    }
}

#[cfg(test)]
#[path = "sogc_publication_repository_tests.rs"]
mod sogc_publication_repository_tests;
