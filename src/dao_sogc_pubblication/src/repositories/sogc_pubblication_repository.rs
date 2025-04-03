use crate::models::SogcPublication;
use crate::repositories::SOGC_PUBBLICATION_MEMORY;

pub struct SogcPubblicationRepository;

impl SogcPubblicationRepository {
    fn get_new_id() -> u64 {
        Self::get_length()
    }

    pub fn save(sogc_publication: SogcPublication) -> SogcPublication {
        let id = Self::get_new_id();

        SOGC_PUBBLICATION_MEMORY.with_borrow_mut(|sogc_publication_memory| {
            sogc_publication_memory.insert(id, sogc_publication.clone())
        });

        sogc_publication
    }

    pub fn get(id: u64) -> Option<SogcPublication> {
        SOGC_PUBBLICATION_MEMORY
            .with_borrow(|sogc_publication_memory| sogc_publication_memory.get(&id).clone())
    }

    pub fn get_length() -> u64 {
        SOGC_PUBBLICATION_MEMORY
            .with_borrow(|sogc_publication_memory| sogc_publication_memory.len())
    }
}
