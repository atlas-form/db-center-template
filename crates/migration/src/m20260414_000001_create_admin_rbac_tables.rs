use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminRoles::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AdminRoles::Name).string().not_null())
                    .col(ColumnDef::new(AdminRoles::Code).string().not_null())
                    .index(
                        Index::create()
                            .name("uk_admin_roles_code")
                            .col(AdminRoles::Code)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AdminPermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminPermissions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AdminPermissions::Code).string().not_null())
                    .col(ColumnDef::new(AdminPermissions::Name).string().not_null())
                    .col(ColumnDef::new(AdminPermissions::ParentCode).string().null())
                    .col(
                        ColumnDef::new(AdminPermissions::Sort)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(AdminPermissions::Kind).string().not_null())
                    .index(
                        Index::create()
                            .name("uk_admin_permissions_code")
                            .col(AdminPermissions::Code)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_admin_permissions_parent_code")
                            .from(AdminPermissions::Table, AdminPermissions::ParentCode)
                            .to(AdminPermissions::Table, AdminPermissions::Code)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AdminUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminUsers::UserId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AdminUsers::DisplayId)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminUsers::DisplayName)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(ColumnDef::new(AdminUsers::Remark).string_len(255).null())
                    .col(ColumnDef::new(AdminUsers::Status).string().not_null())
                    .index(
                        Index::create()
                            .name("uk_admin_users_display_id")
                            .col(AdminUsers::DisplayId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AdminUserRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AdminUserRoles::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(AdminUserRoles::RoleId)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_admin_user_roles")
                            .col(AdminUserRoles::UserId)
                            .col(AdminUserRoles::RoleId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_admin_user_roles_user_id")
                            .from(AdminUserRoles::Table, AdminUserRoles::UserId)
                            .to(AdminUsers::Table, AdminUsers::UserId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_admin_user_roles_role_id")
                            .from(AdminUserRoles::Table, AdminUserRoles::RoleId)
                            .to(AdminRoles::Table, AdminRoles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AdminRolePermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminRolePermissions::RoleId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminRolePermissions::PermissionId)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_admin_role_permissions")
                            .col(AdminRolePermissions::RoleId)
                            .col(AdminRolePermissions::PermissionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_admin_role_permissions_role_id")
                            .from(AdminRolePermissions::Table, AdminRolePermissions::RoleId)
                            .to(AdminRoles::Table, AdminRoles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_admin_role_permissions_permission_id")
                            .from(
                                AdminRolePermissions::Table,
                                AdminRolePermissions::PermissionId,
                            )
                            .to(AdminPermissions::Table, AdminPermissions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AdminMenus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminMenus::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AdminMenus::Name).string().not_null())
                    .col(ColumnDef::new(AdminMenus::ParentId).big_integer().null())
                    .col(ColumnDef::new(AdminMenus::PermissionCode).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_admin_menus_parent_id")
                            .from(AdminMenus::Table, AdminMenus::ParentId)
                            .to(AdminMenus::Table, AdminMenus::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_admin_menus_permission_code")
                            .from(AdminMenus::Table, AdminMenus::PermissionCode)
                            .to(AdminPermissions::Table, AdminPermissions::Code)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AdminMenus::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(AdminRolePermissions::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(AdminUserRoles::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(AdminUsers::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(AdminPermissions::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(AdminRoles::Table).cascade().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum AdminRoles {
    Table,
    Id,
    Name,
    Code,
}

#[derive(DeriveIden)]
enum AdminPermissions {
    Table,
    Id,
    Code,
    Name,
    ParentCode,
    Sort,
    Kind,
}

#[derive(DeriveIden)]
enum AdminUsers {
    Table,
    UserId,
    DisplayId,
    DisplayName,
    Remark,
    Status,
}

#[derive(DeriveIden)]
enum AdminUserRoles {
    Table,
    UserId,
    RoleId,
}

#[derive(DeriveIden)]
enum AdminRolePermissions {
    Table,
    RoleId,
    PermissionId,
}

#[derive(DeriveIden)]
enum AdminMenus {
    Table,
    Id,
    Name,
    ParentId,
    PermissionCode,
}
