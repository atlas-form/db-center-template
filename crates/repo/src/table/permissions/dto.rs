use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub parent_code: Option<String>,
    pub sort: i32,
    pub kind: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePermission {
    pub code: String,
    pub name: String,
    pub parent_code: Option<String>,
    pub sort: i32,
    pub kind: String,
}
