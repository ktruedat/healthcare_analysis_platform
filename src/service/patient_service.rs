use crate::model::patient::Patient;
use crate::repository::repo::Repository;
use crate::service::generic_service::GenericService;

pub struct PatientService {
    pub repo: Repository<Patient>,
}

impl PatientService {
    pub fn new(repo: Repository<Patient>) -> Self {
        PatientService { repo }
    }
}

impl GenericService<Patient> for PatientService {
    fn handle_get(&self, id: &str) -> Result<Patient, Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn handle_create(&self, entity: Patient) -> Result<Patient, Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn handle_update(&self, entity: Patient) -> Result<Patient, Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn handle_delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn handle_list(&self) -> Result<Vec<Patient>, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
