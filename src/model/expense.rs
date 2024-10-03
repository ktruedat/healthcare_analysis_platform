use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Expense {
    id: String,
    institution_id: String,
    year: u32,
    total_expense: f32,
    expenses_by_category: serde_json::Value, // Use a JSON object for flexible categories
}
