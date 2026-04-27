use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRole {
    pub name: String,
    pub code: String,
}
