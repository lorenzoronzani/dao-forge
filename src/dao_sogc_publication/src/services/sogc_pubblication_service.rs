use crate::models::SogcPublication;
use crate::repositories::SogcPubblicationRepository;
use common::types::Mutation;

pub struct SogcPubblicationService;

impl SogcPubblicationService {
    fn get_next_id() -> u32 {
        SogcPubblicationRepository::size() as u32 + 1
    }

    pub fn save(
        publication_sogc_date: u64,
        daily_number: u32,
        publication_date: u64,
        mutations: Vec<Mutation>,
        description: String,
    ) -> SogcPublication {
        let sogc_publication = SogcPublication::new(
            Self::get_next_id(),
            publication_sogc_date,
            daily_number,
            publication_date,
            mutations,
            description,
        );

        SogcPubblicationRepository::save(sogc_publication)
    }

    pub fn get(id: u32) -> Option<SogcPublication> {
        SogcPubblicationRepository::get(id)
    }
}
