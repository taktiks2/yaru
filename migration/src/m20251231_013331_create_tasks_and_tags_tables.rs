use entity::{tags, task_tags, tasks};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, TransactionTrait};
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
                    .col(
                        timestamp_with_time_zone(Tasks::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Tasks::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
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
                    .col(
                        timestamp_with_time_zone(Tags::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Tags::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. TaskTags中間テーブル作成（多対多関係）
        //
        // 外部キー制約の削除時動作:
        //
        // task_id -> tasks (CASCADE):
        //   タスクを削除すると、そのタスクに紐づくTaskTagsも自動削除される
        //   理由: タスクが削除されたら、そのタグ関連付けも不要になるため
        //   例: タスクID=1を削除 → task_tags内のtask_id=1も全て削除
        //
        // tag_id -> tags (RESTRICT):
        //   タグを削除しようとしても、使用中のTaskTagsが存在する場合は削除が拒否される
        //   理由: タグは複数のタスクで共有される重要なマスタデータなので誤削除を防ぐ
        //   例: タグID=1を削除 → task_tags内にtag_id=1があればエラー
        //       先にタスクからタグを外す必要がある
        manager
            .create_table(
                Table::create()
                    .table(TaskTags::Table)
                    .if_not_exists()
                    .col(integer(TaskTags::TaskId))
                    .col(integer(TaskTags::TagId))
                    .primary_key(Index::create().col(TaskTags::TaskId).col(TaskTags::TagId))
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
                            .on_delete(ForeignKeyAction::Restrict),
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

        // 5. Tasksテーブルのupdated_at自動更新トリガー
        //
        // WHEN NEW.updated_at = OLD.updated_at の意味:
        //   updated_atが明示的に変更されなかった場合のみトリガーを発動
        //   理由: ユーザーが意図的にupdated_atを設定した場合は、その値を尊重し自動更新をスキップする
        //   例: 通常のUPDATE → 自動更新される
        //       UPDATE tasks SET updated_at = '2025-01-01' → 自動更新されない（指定値が使われる）
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TRIGGER update_tasks_timestamp
                 AFTER UPDATE ON tasks
                 FOR EACH ROW
                 WHEN NEW.updated_at = OLD.updated_at
                 BEGIN
                     UPDATE tasks SET updated_at = CURRENT_TIMESTAMP
                     WHERE id = NEW.id;
                 END;",
            )
            .await?;

        // 6. Tagsテーブルのupdated_at自動更新トリガー
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TRIGGER update_tags_timestamp
                 AFTER UPDATE ON tags
                 FOR EACH ROW
                 WHEN NEW.updated_at = OLD.updated_at
                 BEGIN
                     UPDATE tags SET updated_at = CURRENT_TIMESTAMP
                     WHERE id = NEW.id;
                 END;",
            )
            .await?;

        // 7. 開発用のシーダー実行
        // 環境変数RUN_SEEDERが設定されている場合のみ実行
        if std::env::var("RUN_SEEDER").is_ok() {
            println!("環境変数RUN_SEEDERが設定されているため、シーダーを実行します...");
            self.seed_data(manager).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // トリガーを削除
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS update_tasks_timestamp")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS update_tags_timestamp")
            .await?;

        // テーブルを削除
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

impl Migration {
    /// シーディングデータを投入する
    async fn seed_data(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // トランザクションを開始（ベストプラクティス）
        let txn = db.begin().await?;

        println!("タグのサンプルデータを作成中...");

        // 1. タグを作成
        let tag_important = tags::ActiveModel {
            name: Set("重要".to_owned()),
            description: Set("優先度が高く、注目すべき項目".to_owned()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let tag_urgent = tags::ActiveModel {
            name: Set("緊急".to_owned()),
            description: Set("即座に対応が必要な項目".to_owned()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let _tag_bugfix = tags::ActiveModel {
            name: Set("バグ修正".to_owned()),
            description: Set("不具合の修正に関する項目".to_owned()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let tag_feature = tags::ActiveModel {
            name: Set("新機能".to_owned()),
            description: Set("新しい機能の追加に関する項目".to_owned()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        println!("タスクのサンプルデータを作成中...");

        // 2. タスクを作成
        let task1 = tasks::ActiveModel {
            title: Set("データベース設計の見直し".to_owned()),
            description: Set(
                "パフォーマンス向上のためのインデックス最適化とテーブル構造の見直し".to_owned(),
            ),
            status: Set("Pending".to_owned()),
            priority: Set("High".to_owned()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let task2 = tasks::ActiveModel {
            title: Set("ユニットテストの追加".to_owned()),
            description: Set("リポジトリ層とコマンド層のユニットテストを作成".to_owned()),
            status: Set("InProgress".to_owned()),
            priority: Set("Medium".to_owned()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let _task3 = tasks::ActiveModel {
            title: Set("ドキュメント作成".to_owned()),
            description: Set("API仕様書とユーザーマニュアルの作成".to_owned()),
            status: Set("Pending".to_owned()),
            priority: Set("Low".to_owned()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let task4 = tasks::ActiveModel {
            title: Set("パフォーマンス改善".to_owned()),
            description: Set("クエリの最適化とキャッシュ機構の導入".to_owned()),
            status: Set("Completed".to_owned()),
            priority: Set("High".to_owned()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        println!("タスクとタグの関連付けを作成中...");

        // 3. タスクとタグの関連付け
        // task1: 重要、緊急
        task_tags::ActiveModel {
            task_id: Set(task1.id),
            tag_id: Set(tag_important.id),
        }
        .insert(&txn)
        .await?;

        task_tags::ActiveModel {
            task_id: Set(task1.id),
            tag_id: Set(tag_urgent.id),
        }
        .insert(&txn)
        .await?;

        // task2: 重要
        task_tags::ActiveModel {
            task_id: Set(task2.id),
            tag_id: Set(tag_important.id),
        }
        .insert(&txn)
        .await?;

        // task3: なし（タグなしのタスク例）

        // task4: 重要、新機能
        task_tags::ActiveModel {
            task_id: Set(task4.id),
            tag_id: Set(tag_important.id),
        }
        .insert(&txn)
        .await?;

        task_tags::ActiveModel {
            task_id: Set(task4.id),
            tag_id: Set(tag_feature.id),
        }
        .insert(&txn)
        .await?;

        // トランザクションをコミット
        txn.commit().await?;

        println!("シーダーの実行が完了しました！");
        println!("  - タグ: 4件");
        println!("  - タスク: 4件");
        println!("  - タスク-タグ関連: 5件");

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
