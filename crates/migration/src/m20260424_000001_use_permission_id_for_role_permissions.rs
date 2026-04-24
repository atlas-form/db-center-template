use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(RolePermissions::Table)
                    .add_column(
                        ColumnDef::new(RolePermissions::PermissionId)
                            .big_integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                UPDATE role_permissions rp
                SET permission_id = p.id
                FROM permissions p
                WHERE rp.permission_code = p.code
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE role_permissions
                    ALTER COLUMN permission_id SET NOT NULL,
                    DROP CONSTRAINT pk_role_permissions,
                    DROP COLUMN permission_code,
                    ADD CONSTRAINT pk_role_permissions PRIMARY KEY (role_id, permission_id),
                    ADD CONSTRAINT fk_role_permissions_role_id
                        FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
                    ADD CONSTRAINT fk_role_permissions_permission_id
                        FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(RolePermissions::Table)
                    .add_column(
                        ColumnDef::new(RolePermissions::PermissionCode)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                UPDATE role_permissions rp
                SET permission_code = p.code
                FROM permissions p
                WHERE rp.permission_id = p.id
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE role_permissions
                    ALTER COLUMN permission_code SET NOT NULL,
                    DROP CONSTRAINT fk_role_permissions_permission_id,
                    DROP CONSTRAINT fk_role_permissions_role_id,
                    DROP CONSTRAINT pk_role_permissions,
                    DROP COLUMN permission_id,
                    ADD CONSTRAINT pk_role_permissions PRIMARY KEY (role_id, permission_code)
                "#,
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RolePermissions {
    Table,
    PermissionCode,
    PermissionId,
}
