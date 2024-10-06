use crate::model::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Patient {
    id: String,
    name: String,
    age: u32,
    gender: String,
    insurance_status: String,
    registration_date: String,
}

impl Entity for Patient {
    fn id(&self) -> &str {
        &self.id
    }
}
