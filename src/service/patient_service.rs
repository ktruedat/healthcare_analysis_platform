use crate::model::patient::Patient;
use crate::repository::generic_repo::GenericRepository;
use crate::repository::repo::Repository;
use crate::service::generic_service::GenericService;

#[derive(Clone)]
pub struct PatientService {
    pub repo: Repository<Patient>,
}

impl PatientService {
    pub fn new(repo: Repository<Patient>) -> Self {
        PatientService { repo }
    }
}

impl GenericService<Patient> for PatientService {
    async fn handle_get(&self, id: &str) -> Result<Patient, Box<dyn std::error::Error>> {
        match self.repo.get(id).await {
            Ok(patient) => Ok(patient),
            Err(e) => Err(e),
        }
    }

    async fn handle_create(&self, entity: Patient) -> Result<Patient, Box<dyn std::error::Error>> {
        match self.repo.create(entity).await {
            Ok(patient) => Ok(patient),
            Err(e) => Err(e),
        }
    }

    async fn handle_update(&self, entity: Patient) -> Result<Patient, Box<dyn std::error::Error>> {
        match self.repo.update(entity).await {
            Ok(patient) => Ok(patient),
            Err(e) => Err(e),
        }
    }

    async fn handle_delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self.repo.delete(id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn handle_list(&self) -> Result<Vec<Patient>, Box<dyn std::error::Error>> {
        match self.repo.list().await {
            Ok(patients) => Ok(patients),
            Err(e) => Err(e),
        }
    }
}
