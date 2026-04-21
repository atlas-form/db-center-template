use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Menus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Menus::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Menus::Name).string().not_null())
                    .col(ColumnDef::new(Menus::Path).string().not_null())
                    .col(ColumnDef::new(Menus::ParentId).big_integer().null())
                    .col(ColumnDef::new(Menus::PermissionCode).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Menus::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Menus {
    Table,
    Id,
    Name,
    Path,
    ParentId,
    PermissionCode,
}
