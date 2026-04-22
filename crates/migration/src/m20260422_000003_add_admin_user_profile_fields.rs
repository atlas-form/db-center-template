use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminUsers::Table)
                    .add_column(
                        ColumnDef::new(AdminUsers::DisplayName)
                            .string_len(64)
                            .not_null()
                            .default(""),
                    )
                    .add_column(ColumnDef::new(AdminUsers::Remark).string_len(255).null())
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"UPDATE admin_users
SET display_name = CONCAT('admin-', LEFT(user_id::text, 8))
WHERE display_name = ''"#,
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUsers::Table)
                    .modify_column(
                        ColumnDef::new(AdminUsers::DisplayName)
                            .string_len(64)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminUsers::Table)
                    .drop_column(AdminUsers::Remark)
                    .drop_column(AdminUsers::DisplayName)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AdminUsers {
    Table,
    DisplayName,
    Remark,
}
