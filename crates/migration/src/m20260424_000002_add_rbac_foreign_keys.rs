use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE user_roles
                    ADD CONSTRAINT fk_user_roles_user_id
                        FOREIGN KEY (user_id) REFERENCES admin_users(user_id) ON DELETE CASCADE,
                    ADD CONSTRAINT fk_user_roles_role_id
                        FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE;

                ALTER TABLE permissions
                    ADD CONSTRAINT fk_permissions_parent_code
                        FOREIGN KEY (parent_code) REFERENCES permissions(code) ON DELETE SET NULL;

                ALTER TABLE menus
                    ADD CONSTRAINT fk_menus_parent_id
                        FOREIGN KEY (parent_id) REFERENCES menus(id) ON DELETE SET NULL,
                    ADD CONSTRAINT fk_menus_permission_code
                        FOREIGN KEY (permission_code) REFERENCES permissions(code) ON DELETE SET NULL;
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE menus
                    DROP CONSTRAINT fk_menus_permission_code,
                    DROP CONSTRAINT fk_menus_parent_id;

                ALTER TABLE permissions
                    DROP CONSTRAINT fk_permissions_parent_code;

                ALTER TABLE user_roles
                    DROP CONSTRAINT fk_user_roles_role_id,
                    DROP CONSTRAINT fk_user_roles_user_id;
                "#,
            )
            .await?;

        Ok(())
    }
}
