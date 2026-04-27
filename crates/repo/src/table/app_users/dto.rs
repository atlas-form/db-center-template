use db_core::error::{BIZ_INTERNAL_ERROR, BizError, BizResult};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppUserStatus {
    Enabled,
    Disabled,
}

impl AppUserStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        }
    }
}

impl TryFrom<&str> for AppUserStatus {
    type Error = BizError;

    fn try_from(value: &str) -> BizResult<Self> {
        match value {
            "enabled" => Ok(Self::Enabled),
            "disabled" => Ok(Self::Disabled),
            _ => Err(BizError::new(
                BIZ_INTERNAL_ERROR,
                format!("invalid app user status: {value}"),
            )),
        }
    }
}

impl TryFrom<String> for AppUserStatus {
    type Error = BizError;

    fn try_from(value: String) -> BizResult<Self> {
        Self::try_from(value.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUser {
    pub user_id: Uuid,
    pub display_id: String,
    pub display_name: String,
    pub remark: Option<String>,
    pub status: AppUserStatus,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Default)]
pub struct AppUserFilter {
    pub keyword: Option<String>,
    pub keyword_user_id: Option<Uuid>,
    pub status: Option<AppUserStatus>,
    pub created_at_from: Option<OffsetDateTime>,
    pub created_at_to: Option<OffsetDateTime>,
    pub updated_at_from: Option<OffsetDateTime>,
    pub updated_at_to: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAppUser {
    pub user_id: Uuid,
    pub display_id: String,
    pub display_name: String,
    pub remark: Option<String>,
    pub status: AppUserStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAppUser {
    pub user_id: Uuid,
    pub remark: Option<String>,
    pub status: AppUserStatus,
}
