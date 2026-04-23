pub use sea_orm_migration::prelude::*;

mod m20260414_000001_create_rbac_tables;
mod m20260414_000002_create_menus_table;
mod m20260421_015859_create_admin_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260414_000001_create_rbac_tables::Migration),
            Box::new(m20260414_000002_create_menus_table::Migration),
            Box::new(m20260421_015859_create_admin_users_table::Migration),
        ]
    }
}
