use crate::models::DaoAssociation;
use crate::repositories::DaoAssociationRepository;

pub struct DaoAssociationService;

impl DaoAssociationService {
    pub fn save(dao_association: DaoAssociation) -> DaoAssociation {
        DaoAssociationRepository::save(dao_association)
    }

    pub fn get() -> DaoAssociation {
        DaoAssociationRepository::get()
    }

    pub fn update(dao_association: DaoAssociation) -> DaoAssociation {
        DaoAssociationRepository::save(dao_association)
    }
}
