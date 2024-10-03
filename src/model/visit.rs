use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Visit {
    id: String,
    patient_id: String,
    institution_id: String,
    disease_id: String,
    visit_date: String,
    treatment_received: String,
}
