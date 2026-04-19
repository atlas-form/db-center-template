use db_core::{DbContext, Repository, error::BizResult};
use sea_orm::{ActiveValue::Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    entity::user_roles,
    table::user_roles::dto::{CreateUserRole, UserRole},
};

db_core::impl_repository!(UserRoleRepo, user_roles::Entity, user_roles::Model);

pub struct UserRoleService {
    repo: UserRoleRepo,
}

impl UserRoleService {
    pub fn new(db: DbContext) -> Self {
        Self {
            repo: UserRoleRepo::new(db),
        }
    }

    pub async fn create(&self, input: CreateUserRole) -> BizResult<UserRole> {
        let model = user_roles::ActiveModel {
            user_id: Set(input.user_id),
            role_id: Set(input.role_id),
        };

        Ok(Self::from_model(self.repo.insert(model).await?))
    }

    pub async fn list_by_user_id(&self, user_id: Uuid) -> BizResult<Vec<UserRole>> {
        let query = self
            .repo
            .query()
            .filter(user_roles::Column::UserId.eq(user_id));

        Ok(self
            .repo
            .select_all(query)
            .await?
            .into_iter()
            .map(Self::from_model)
            .collect())
    }

    fn from_model(model: user_roles::Model) -> UserRole {
        UserRole {
            user_id: model.user_id,
            role_id: model.role_id,
        }
    }
}
