use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Disease {
    id: String,
    name: String,
    description: String,
    morbidity_rate: f32,
    treatment_cost: f32,
}
