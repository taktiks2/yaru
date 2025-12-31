use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Tasksテーブル作成
        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(pk_auto(Tasks::Id))
                    .col(string(Tasks::Title))
                    .col(text(Tasks::Description))
                    .col(string(Tasks::Status))
                    .col(string(Tasks::Priority))
                    .col(timestamp_with_time_zone(Tasks::CreatedAt))
                    .col(timestamp_with_time_zone(Tasks::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // 2. Tagsテーブル作成
        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(pk_auto(Tags::Id))
                    .col(string_uniq(Tags::Name))
                    .col(text(Tags::Description))
                    .col(timestamp_with_time_zone(Tags::CreatedAt))
                    .col(timestamp_with_time_zone(Tags::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // 3. TaskTags中間テーブル作成（多対多関係）
        manager
            .create_table(
                Table::create()
                    .table(TaskTags::Table)
                    .if_not_exists()
                    .col(big_integer(TaskTags::TaskId))
                    .col(big_integer(TaskTags::TagId))
                    .primary_key(
                        Index::create()
                            .col(TaskTags::TaskId)
                            .col(TaskTags::TagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TaskTags::Table, TaskTags::TaskId)
                            .to(Tasks::Table, Tasks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TaskTags::Table, TaskTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. インデックス作成
        manager
            .create_index(
                Index::create()
                    .name("idx_tasks_status")
                    .table(Tasks::Table)
                    .col(Tasks::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tasks_priority")
                    .table(Tasks::Table)
                    .col(Tasks::Priority)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaskTags::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    Id,
    Title,
    Description,
    Status,
    Priority,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tags {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum TaskTags {
    Table,
    TaskId,
    TagId,
}
