use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub user_id: String,
    pub activity_type: String,
    pub timestamp: String,
}

