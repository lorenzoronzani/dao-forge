use crate::models::SogcPublication;
use crate::repositories::SogcPubblicationRepository;

pub struct SogcPubblicationService;

impl SogcPubblicationService {
    pub fn save(sogc_publication: SogcPublication) -> SogcPublication {
        SogcPubblicationRepository::save(sogc_publication)
    }

    pub fn get(id: u64) -> Option<SogcPublication> {
        SogcPubblicationRepository::get(id)
    }
}
