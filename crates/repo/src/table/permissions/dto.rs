use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: i64,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePermission {
    pub code: String,
}
