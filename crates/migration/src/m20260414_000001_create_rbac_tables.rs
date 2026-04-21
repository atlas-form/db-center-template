use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Roles::Name).string().not_null())
                    .col(ColumnDef::new(Roles::Code).string().not_null())
                    .index(
                        Index::create()
                            .name("uk_roles_code")
                            .col(Roles::Code)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permissions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Permissions::Code).string().not_null())
                    .col(ColumnDef::new(Permissions::Name).string().not_null())
                    .col(ColumnDef::new(Permissions::ParentCode).string().null())
                    .col(
                        ColumnDef::new(Permissions::Sort)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Permissions::Kind).string().not_null())
                    .index(
                        Index::create()
                            .name("uk_permissions_code")
                            .col(Permissions::Code)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRoles::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserRoles::RoleId).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk_user_roles")
                            .col(UserRoles::UserId)
                            .col(UserRoles::RoleId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RolePermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermissions::RoleId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RolePermissions::PermissionCode)
                            .string()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_role_permissions")
                            .col(RolePermissions::RoleId)
                            .col(RolePermissions::PermissionCode),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(RolePermissions::Table)
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(UserRoles::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Permissions::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Roles::Table).cascade().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    Name,
    Code,
}

#[derive(DeriveIden)]
enum Permissions {
    Table,
    Id,
    Code,
    Name,
    ParentCode,
    Sort,
    Kind,
}

#[derive(DeriveIden)]
enum UserRoles {
    Table,
    UserId,
    RoleId,
}

#[derive(DeriveIden)]
enum RolePermissions {
    Table,
    RoleId,
    PermissionCode,
}
