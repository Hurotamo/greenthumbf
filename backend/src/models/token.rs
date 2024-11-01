use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub user_id: String,
    pub amount: f64,
}

