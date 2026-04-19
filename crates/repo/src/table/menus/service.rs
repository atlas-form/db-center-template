use db_core::{DbContext, Repository, error::BizResult};
use sea_orm::{ActiveValue::Set, QueryOrder};

use crate::{
    entity::menus,
    table::menus::dto::{CreateMenu, Menu},
};

db_core::impl_repository!(MenuRepo, menus::Entity, menus::Model);

pub struct MenuService {
    repo: MenuRepo,
}

impl MenuService {
    pub fn new(db: DbContext) -> Self {
        Self {
            repo: MenuRepo::new(db),
        }
    }

    pub async fn create(&self, input: CreateMenu) -> BizResult<Menu> {
        let model = menus::ActiveModel {
            name: Set(input.name),
            path: Set(input.path),
            parent_id: Set(input.parent_id),
            permission_code: Set(input.permission_code),
            ..Default::default()
        };

        Ok(Self::from_model(self.repo.insert(model).await?))
    }

    pub async fn list_all(&self) -> BizResult<Vec<Menu>> {
        let query = self.repo.query().order_by_asc(menus::Column::Id);
        Ok(self
            .repo
            .select_all(query)
            .await?
            .into_iter()
            .map(Self::from_model)
            .collect())
    }

    fn from_model(model: menus::Model) -> Menu {
        Menu {
            id: model.id,
            name: model.name,
            path: model.path,
            parent_id: model.parent_id,
            permission_code: model.permission_code,
        }
    }
}
