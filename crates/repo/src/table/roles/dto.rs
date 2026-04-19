use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRole {
    pub name: String,
    pub code: String,
}
