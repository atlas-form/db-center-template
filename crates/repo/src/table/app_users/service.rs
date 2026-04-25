use db_core::{DbContext, Repository, error::BizResult};
use sea_orm::{
    ActiveValue::{Set, Unchanged},
    QueryOrder,
};
use uuid::Uuid;

use crate::{
    entity::app_users,
    table::app_users::dto::{AppUser, AppUserStatus, CreateAppUser, UpdateAppUser},
};

db_core::impl_repository!(AppUserRepo, app_users::Entity, app_users::Model);

pub struct AppUserService {
    repo: AppUserRepo,
}

impl AppUserService {
    pub fn new(db: DbContext) -> Self {
        Self {
            repo: AppUserRepo::new(db),
        }
    }

    pub async fn create(&self, input: CreateAppUser) -> BizResult<AppUser> {
        let model = app_users::ActiveModel {
            user_id: Set(input.user_id),
            display_id: Set(input.display_id),
            display_name: Set(input.display_name),
            remark: Set(input.remark),
            status: Set(input.status.as_str().to_owned()),
        };

        Self::from_model(self.repo.insert(model).await?)
    }

    pub async fn list_all(&self) -> BizResult<Vec<AppUser>> {
        let query = self.repo.query().order_by_asc(app_users::Column::UserId);
        self.repo
            .select_all(query)
            .await?
            .into_iter()
            .map(Self::from_model)
            .collect()
    }

    pub async fn get_by_user_id(&self, user_id: Uuid) -> BizResult<Option<AppUser>> {
        self.repo
            .find_by_id(user_id)
            .await?
            .map(Self::from_model)
            .transpose()
    }

    pub async fn update(&self, input: UpdateAppUser) -> BizResult<AppUser> {
        let model = app_users::ActiveModel {
            user_id: Unchanged(input.user_id),
            remark: Set(input.remark),
            status: Set(input.status.as_str().to_owned()),
            ..Default::default()
        };

        Self::from_model(self.repo.update(model).await?)
    }

    pub async fn delete_by_user_id(&self, user_id: Uuid) -> BizResult<u64> {
        Ok(self.repo.delete_by_id(user_id).await?.rows_affected)
    }

    fn from_model(model: app_users::Model) -> BizResult<AppUser> {
        Ok(AppUser {
            user_id: model.user_id,
            display_id: model.display_id,
            display_name: model.display_name,
            remark: model.remark,
            status: AppUserStatus::try_from(model.status)?,
        })
    }
}
