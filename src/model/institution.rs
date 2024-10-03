use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Institution {
    id: String,
    name: String,
    institution_type: String,
    location: String,
    capacity: u32,
    services_offered: Vec<String>,
}
