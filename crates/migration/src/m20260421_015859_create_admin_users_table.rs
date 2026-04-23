use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AdminUsers::Table).cascade().to_owned())
            .await
    }
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
