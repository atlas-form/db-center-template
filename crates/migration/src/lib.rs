pub use sea_orm_migration::prelude::*;

mod m20260414_000001_create_rbac_tables;
mod m20260414_000002_create_menus_table;
mod m20260421_015859_create_admin_users_table;
mod m20260424_000001_use_permission_id_for_role_permissions;
mod m20260424_000002_add_rbac_foreign_keys;
mod m20260425_000001_delete_root_role_permissions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260414_000001_create_rbac_tables::Migration),
            Box::new(m20260414_000002_create_menus_table::Migration),
            Box::new(m20260421_015859_create_admin_users_table::Migration),
            Box::new(m20260424_000001_use_permission_id_for_role_permissions::Migration),
            Box::new(m20260424_000002_add_rbac_foreign_keys::Migration),
            Box::new(m20260425_000001_delete_root_role_permissions::Migration),
        ]
    }
}
