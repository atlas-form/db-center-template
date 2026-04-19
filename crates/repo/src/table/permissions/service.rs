use db_core::{DbContext, Repository, error::BizResult};
use sea_orm::{ActiveValue::Set, QueryOrder};

use crate::{
    entity::permissions,
    table::permissions::dto::{CreatePermission, Permission},
};

db_core::impl_repository!(PermissionRepo, permissions::Entity, permissions::Model);

pub struct PermissionService {
    repo: PermissionRepo,
}

impl PermissionService {
    pub fn new(db: DbContext) -> Self {
        Self {
            repo: PermissionRepo::new(db),
        }
    }

    pub async fn create(&self, input: CreatePermission) -> BizResult<Permission> {
        let model = permissions::ActiveModel {
            code: Set(input.code),
            ..Default::default()
        };

        Ok(Self::from_model(self.repo.insert(model).await?))
    }

    pub async fn list_all(&self) -> BizResult<Vec<Permission>> {
        let query = self.repo.query().order_by_asc(permissions::Column::Id);
        Ok(self
            .repo
            .select_all(query)
            .await?
            .into_iter()
            .map(Self::from_model)
            .collect())
    }

    fn from_model(model: permissions::Model) -> Permission {
        Permission {
            id: model.id,
            code: model.code,
        }
    }
}
