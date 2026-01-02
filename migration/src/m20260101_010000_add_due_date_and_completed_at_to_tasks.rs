use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // tasksテーブルにdue_dateカラムを追加
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .add_column(date_null(Tasks::DueDate))
                    .to_owned(),
            )
            .await?;

        // tasksテーブルにcompleted_atカラムを追加
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .add_column(timestamp_with_time_zone_null(Tasks::CompletedAt))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // tasksテーブルからcompleted_atカラムを削除
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .drop_column(Tasks::CompletedAt)
                    .to_owned(),
            )
            .await?;

        // tasksテーブルからdue_dateカラムを削除
        manager
            .alter_table(
                Table::alter()
                    .table(Tasks::Table)
                    .drop_column(Tasks::DueDate)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    DueDate,
    CompletedAt,
}
