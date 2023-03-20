use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub id: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
