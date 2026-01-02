use crate::{
    domain::task::{Status, Task},
    repository::Repository,
};
use anyhow::{Context, Result};
use chrono::Utc;
use entity::{prelude::*, task_tags, tasks};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, Set, TransactionTrait,
};

/// Task用のリポジトリ実装
pub struct TaskRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> TaskRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }
}

impl<'a> Repository<Task> for TaskRepository<'a> {
    // SeaORMの関連エンティティ取得方法について:
    //
    // 現在の実装では `find_with_related()` を使用しています。
    // これは1回のクエリ（JOIN使用）で主エンティティと関連エンティティを取得します。
    //
    // 代替案として EntityLoader パターン（`load_many()`）もありますが、
    // 以下の理由から現在の実装を採用しています：
    //
    // 【find_with_related() の特徴】
    // - 1回のクエリで完結（JOIN使用）
    // - タスク管理アプリでは通常タグ数が少ない（2-3個程度）
    // - 常にタグ情報が必要
    // - シンプルで保守性が高い
    //
    // 【EntityLoader パターン（load_many()）の特徴】
    // - 複数回のクエリ（主エンティティ + 関連エンティティごと）
    // - タスクあたりのタグ数が非常に多い場合（10個以上）に効率的
    // - 複数の1:N関連がある場合に有効
    // - 選択的読み込み（必要なものだけ取得）が可能
    //
    // 参考: https://www.sea-ql.org/SeaORM/docs/advanced-query/custom-select/

    async fn find_by_id(&self, id: i32) -> Result<Option<Task>> {
        let result = Tasks::find_by_id(id)
            .find_with_related(Tags)
            .all(self.db)
            .await
            .context("タスクの検索に失敗しました")?;

        match result.into_iter().next() {
            Some((model, tags)) => {
                let task = Task::try_from((model, tags)).map_err(anyhow::Error::msg)?;
                Ok(Some(task))
            }
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<Task>> {
        let tasks_with_tags = Tasks::find()
            .find_with_related(Tags)
            .all(self.db)
            .await
            .context("タスクとタグの読み込みに失敗しました")?;

        tasks_with_tags
            .into_iter()
            .map(|(model, tags)| Task::try_from((model, tags)).map_err(anyhow::Error::msg))
            .collect()
    }

    async fn search<F>(&self, predicate: F) -> Result<Vec<Task>>
    where
        F: Fn(&Task) -> bool,
    {
        let tasks = self.find_all().await?;
        Ok(tasks.into_iter().filter(predicate).collect())
    }

    async fn create(&self, item: &Task) -> Result<Task> {
        let txn = self.db.begin().await?;

        // completed_atの自動設定ロジック
        let completed_at = if item.status == Status::Completed {
            item.completed_at.or_else(|| Some(Utc::now()))
        } else {
            None
        };

        // タスクを新規作成
        let new_task = tasks::ActiveModel {
            id: NotSet, // AUTO INCREMENT
            title: Set(item.title.clone()),
            description: Set(item.description.clone()),
            status: Set(item.status.as_ref().to_string()),
            priority: Set(item.priority.as_ref().to_string()),
            created_at: NotSet,
            updated_at: NotSet,
            due_date: Set(item.due_date),
            completed_at: Set(completed_at.map(|dt| dt.into())),
        };

        let inserted_task = new_task
            .insert(&txn)
            .await
            .context("タスクの挿入に失敗しました")?;

        // タグの関連付けを保存
        let mut tag_models = Vec::new();
        for tag in &item.tags {
            let task_tag = task_tags::ActiveModel {
                task_id: Set(inserted_task.id),
                tag_id: Set(tag.id),
            };
            task_tag
                .insert(&txn)
                .await
                .context("タスクタグの挿入に失敗しました")?;

            // タグのモデルを取得
            let tag_model = Tags::find_by_id(tag.id)
                .one(&txn)
                .await
                .context("タグの取得に失敗しました")?
                .context("タグが存在しません")?;
            tag_models.push(tag_model);
        }

        txn.commit()
            .await
            .context("トランザクションのコミットに失敗しました")?;

        Task::try_from((inserted_task, tag_models)).map_err(anyhow::Error::msg)
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        let txn = self.db.begin().await?;

        // タスクタグを削除
        TaskTags::delete_many()
            .filter(task_tags::Column::TaskId.eq(id))
            .exec(&txn)
            .await
            .context("タスクタグの削除に失敗しました")?;

        // タスクを削除
        let result = Tasks::delete_by_id(id)
            .exec(&txn)
            .await
            .context("タスクの削除に失敗しました")?;

        txn.commit()
            .await
            .context("トランザクションのコミットに失敗しました")?;

        Ok(result.rows_affected > 0)
    }

    async fn update(&self, item: &Task) -> Result<Task> {
        let txn = self.db.begin().await?;

        // 現在のタスクを取得
        let current_task = Tasks::find_by_id(item.id)
            .one(&txn)
            .await
            .context("タスクの取得に失敗しました")?
            .context("タスクが存在しません")?;

        let current_status = current_task.status.clone();
        let new_status = item.status.as_ref().to_string();

        // ActiveModelに変換
        let mut active_model = current_task.into_active_model();

        // フィールドを更新
        active_model.title = Set(item.title.clone());
        active_model.description = Set(item.description.clone());
        active_model.status = Set(new_status.clone());
        active_model.priority = Set(item.priority.as_ref().to_string());
        active_model.due_date = Set(item.due_date);

        // completed_atの制御ロジック
        // ケース1: Completedへの変更 -> completed_atを現在時刻に設定
        // ケース2: Completedから他への変更 -> completed_atをNoneにクリア
        // ケース3: それ以外 -> completed_atを変更しない
        let completed_at = match (current_status.as_str(), new_status.as_str()) {
            (old, "Completed") if old != "Completed" => Set(Some(Utc::now().into())),
            ("Completed", new) if new != "Completed" => Set(None),
            _ => NotSet, // 変更なし
        };
        active_model.completed_at = completed_at;

        // タスクを更新
        let updated_task = active_model
            .update(&txn)
            .await
            .context("タスクの更新に失敗しました")?;

        // 既存のタグ関連付けを削除
        TaskTags::delete_many()
            .filter(task_tags::Column::TaskId.eq(item.id))
            .exec(&txn)
            .await
            .context("タスクタグの削除に失敗しました")?;

        // 新しいタグ関連付けを作成
        let mut tag_models = Vec::new();
        for tag in &item.tags {
            let task_tag = task_tags::ActiveModel {
                task_id: Set(item.id),
                tag_id: Set(tag.id),
            };
            task_tag
                .insert(&txn)
                .await
                .context("タスクタグの挿入に失敗しました")?;

            // タグのモデルを取得
            let tag_model = Tags::find_by_id(tag.id)
                .one(&txn)
                .await
                .context("タグの取得に失敗しました")?
                .context("タグが存在しません")?;
            tag_models.push(tag_model);
        }

        txn.commit()
            .await
            .context("トランザクションのコミットに失敗しました")?;

        Task::try_from((updated_task, tag_models)).map_err(anyhow::Error::msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::{Priority, Status};
    use migration::MigratorTrait;
    use sea_orm::{Database, PaginatorTrait};

    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_find_all_empty() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        let tasks = repo.find_all().await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_create_task_without_tags() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        let new_task = Task::new(
            0,
            "テストタスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );

        let created_task = repo.create(&new_task).await.unwrap();

        assert!(created_task.id > 0);
        assert_eq!(created_task.title, "テストタスク");
        assert_eq!(created_task.description, "説明");
        assert_eq!(created_task.status, Status::Pending);
        assert_eq!(created_task.priority, Priority::Medium);
        assert!(created_task.tags.is_empty());
    }

    #[tokio::test]
    async fn test_create_task_with_tags() {
        let db = setup_test_db().await;
        let task_repo = TaskRepository::new(&db);

        // タグを作成（直接Entityを使用）
        use entity::tags;
        use sea_orm::ActiveValue::NotSet;
        let tag1 = tags::ActiveModel {
            id: NotSet,
            name: Set("重要".to_string()),
            description: Set("重要なタスク".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag1: tags::Model = tag1.insert(&db).await.unwrap();

        let tag2 = tags::ActiveModel {
            id: NotSet,
            name: Set("緊急".to_string()),
            description: Set("緊急タスク".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag2: tags::Model = tag2.insert(&db).await.unwrap();

        // タグ付きタスクを作成
        use crate::domain::tag::Tag;
        let tag1 = Tag::from(inserted_tag1.clone());
        let tag2 = Tag::from(inserted_tag2.clone());
        let new_task = Task::new(
            0,
            "タグ付きタスク",
            "説明",
            Status::InProgress,
            Priority::High,
            vec![tag1.clone(), tag2.clone()],
            None,
        );

        let created_task = task_repo.create(&new_task).await.unwrap();

        assert!(created_task.id > 0);
        assert_eq!(created_task.title, "タグ付きタスク");
        assert_eq!(created_task.tags.len(), 2);
        assert!(created_task.tags.iter().any(|t| t.id == tag1.id));
        assert!(created_task.tags.iter().any(|t| t.id == tag2.id));
    }

    #[tokio::test]
    async fn test_find_by_id_existing() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        // タスクを作成
        let new_task = Task::new(
            0,
            "検索テスト",
            "説明",
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );
        let created_task = repo.create(&new_task).await.unwrap();

        // IDで検索
        let found_task = repo.find_by_id(created_task.id).await.unwrap();

        assert!(found_task.is_some());
        let task = found_task.unwrap();
        assert_eq!(task.id, created_task.id);
        assert_eq!(task.title, "検索テスト");
    }

    #[tokio::test]
    async fn test_find_by_id_not_existing() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        let found_task = repo.find_by_id(999).await.unwrap();

        assert!(found_task.is_none());
    }

    #[tokio::test]
    async fn test_find_all_multiple_tasks() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        // 複数のタスクを作成
        let task1 = Task::new(
            0,
            "タスク1",
            "説明1",
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );
        let task2 = Task::new(
            0,
            "タスク2",
            "説明2",
            Status::InProgress,
            Priority::Medium,
            vec![],
            None,
        );
        let task3 = Task::new(
            0,
            "タスク3",
            "説明3",
            Status::Completed,
            Priority::High,
            vec![],
            None,
        );

        repo.create(&task1).await.unwrap();
        repo.create(&task2).await.unwrap();
        repo.create(&task3).await.unwrap();

        // 全件取得
        let tasks = repo.find_all().await.unwrap();

        assert_eq!(tasks.len(), 3);
    }

    #[tokio::test]
    async fn test_delete_existing_task() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        // タスクを作成
        let new_task = Task::new(
            0,
            "削除テスト",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let created_task = repo.create(&new_task).await.unwrap();

        // 削除
        let deleted = repo.delete(created_task.id).await.unwrap();

        assert!(deleted);

        // 削除されたことを確認
        let found_task = repo.find_by_id(created_task.id).await.unwrap();
        assert!(found_task.is_none());
    }

    #[tokio::test]
    async fn test_delete_not_existing_task() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        let deleted = repo.delete(999).await.unwrap();

        assert!(!deleted);
    }

    #[tokio::test]
    async fn test_delete_task_with_tags() {
        let db = setup_test_db().await;
        let task_repo = TaskRepository::new(&db);

        // タグを作成
        use entity::tags;
        use sea_orm::ActiveValue::NotSet;
        let tag = tags::ActiveModel {
            id: NotSet,
            name: Set("テストタグ".to_string()),
            description: Set("説明".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag: tags::Model = tag.insert(&db).await.unwrap();

        // タグ付きタスクを作成
        use crate::domain::tag::Tag;
        let tag = Tag::from(inserted_tag.clone());
        let new_task = Task::new(
            0,
            "タグ削除テスト",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![tag],
            None,
        );
        let created_task = task_repo.create(&new_task).await.unwrap();

        // タスクを削除（関連するtask_tagsも削除されるべき）
        let deleted = task_repo.delete(created_task.id).await.unwrap();

        assert!(deleted);

        // task_tagsが削除されたことを確認
        let task_tags_count = TaskTags::find()
            .filter(task_tags::Column::TaskId.eq(created_task.id))
            .count(&db)
            .await
            .unwrap();

        assert_eq!(task_tags_count, 0);
    }

    #[tokio::test]
    async fn test_search_with_predicate() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        // 複数のタスクを作成
        let task1 = Task::new(
            0,
            "タスク1",
            "説明1",
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );
        let task2 = Task::new(
            0,
            "タスク2",
            "説明2",
            Status::Completed,
            Priority::Medium,
            vec![],
            None,
        );
        let task3 = Task::new(
            0,
            "タスク3",
            "説明3",
            Status::Completed,
            Priority::High,
            vec![],
            None,
        );

        repo.create(&task1).await.unwrap();
        repo.create(&task2).await.unwrap();
        repo.create(&task3).await.unwrap();

        // 完了したタスクのみ検索
        let completed_tasks = repo
            .search(|task| task.status == Status::Completed)
            .await
            .unwrap();

        assert_eq!(completed_tasks.len(), 2);
        assert!(
            completed_tasks
                .iter()
                .all(|t| t.status == Status::Completed)
        );
    }

    #[tokio::test]
    async fn test_update_task_basic() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        // タスクを作成
        let new_task = Task::new(
            0,
            "元のタイトル",
            "元の説明",
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );
        let created_task = repo.create(&new_task).await.unwrap();

        // タスクを更新
        let mut updated_task = created_task.clone();
        updated_task.title = "新しいタイトル".to_string();
        updated_task.description = "新しい説明".to_string();
        updated_task.priority = Priority::High;

        let result = repo.update(&updated_task).await.unwrap();

        // 更新内容を検証
        assert_eq!(result.id, created_task.id);
        assert_eq!(result.title, "新しいタイトル");
        assert_eq!(result.description, "新しい説明");
        assert_eq!(result.priority, Priority::High);
        assert_eq!(result.status, Status::Pending);
    }

    #[tokio::test]
    async fn test_update_task_status_to_completed() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        // Pendingのタスクを作成
        let new_task = Task::new(
            0,
            "テストタスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let created_task = repo.create(&new_task).await.unwrap();

        // completed_atがNoneであることを確認
        assert!(created_task.completed_at.is_none());

        // StatusをCompletedに変更
        let mut updated_task = created_task.clone();
        updated_task.status = Status::Completed;

        let result = repo.update(&updated_task).await.unwrap();

        // StatusがCompletedになり、completed_atが設定されていることを確認
        assert_eq!(result.status, Status::Completed);
        assert!(result.completed_at.is_some());
    }

    #[tokio::test]
    async fn test_update_task_status_from_completed() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        // Completedのタスクを作成
        let new_task = Task::new(
            0,
            "テストタスク",
            "説明",
            Status::Completed,
            Priority::Medium,
            vec![],
            None,
        );
        let created_task = repo.create(&new_task).await.unwrap();

        // completed_atが設定されていることを確認
        assert!(created_task.completed_at.is_some());

        // StatusをPendingに変更
        let mut updated_task = created_task.clone();
        updated_task.status = Status::Pending;

        let result = repo.update(&updated_task).await.unwrap();

        // StatusがPendingになり、completed_atがNoneになっていることを確認
        assert_eq!(result.status, Status::Pending);
        assert!(result.completed_at.is_none());
    }

    #[tokio::test]
    async fn test_update_task_status_completed_to_completed() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        // Completedのタスクを作成
        let new_task = Task::new(
            0,
            "テストタスク",
            "説明",
            Status::Completed,
            Priority::Medium,
            vec![],
            None,
        );
        let created_task = repo.create(&new_task).await.unwrap();

        let original_completed_at = created_task.completed_at;
        assert!(original_completed_at.is_some());

        // StatusをCompletedのまま、タイトルだけ変更
        let mut updated_task = created_task.clone();
        updated_task.title = "新しいタイトル".to_string();

        let result = repo.update(&updated_task).await.unwrap();

        // StatusがCompletedのままで、completed_atが維持されていることを確認
        assert_eq!(result.status, Status::Completed);
        assert_eq!(result.completed_at, original_completed_at);
    }

    #[tokio::test]
    async fn test_update_task_tags() {
        let db = setup_test_db().await;
        let task_repo = TaskRepository::new(&db);

        // タグを作成
        use crate::domain::tag::Tag;
        use entity::tags;
        use sea_orm::ActiveValue::NotSet;

        let tag1 = tags::ActiveModel {
            id: NotSet,
            name: Set("タグ1".to_string()),
            description: Set("説明1".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag1: tags::Model = tag1.insert(&db).await.unwrap();

        let tag2 = tags::ActiveModel {
            id: NotSet,
            name: Set("タグ2".to_string()),
            description: Set("説明2".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag2: tags::Model = tag2.insert(&db).await.unwrap();

        // タグなしのタスクを作成
        let new_task = Task::new(
            0,
            "テストタスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let created_task = task_repo.create(&new_task).await.unwrap();

        assert_eq!(created_task.tags.len(), 0);

        // タグを追加
        let mut updated_task = created_task.clone();
        updated_task.tags = vec![
            Tag::from(inserted_tag1.clone()),
            Tag::from(inserted_tag2.clone()),
        ];

        let result = task_repo.update(&updated_task).await.unwrap();

        // タグが追加されたことを確認
        assert_eq!(result.tags.len(), 2);
        assert!(result.tags.iter().any(|t| t.id == inserted_tag1.id));
        assert!(result.tags.iter().any(|t| t.id == inserted_tag2.id));
    }

    #[tokio::test]
    async fn test_update_task_replace_tags() {
        let db = setup_test_db().await;
        let task_repo = TaskRepository::new(&db);

        // タグを3つ作成
        use crate::domain::tag::Tag;
        use entity::tags;
        use sea_orm::ActiveValue::NotSet;

        let tag1 = tags::ActiveModel {
            id: NotSet,
            name: Set("タグ1".to_string()),
            description: Set("".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag1: tags::Model = tag1.insert(&db).await.unwrap();

        let tag2 = tags::ActiveModel {
            id: NotSet,
            name: Set("タグ2".to_string()),
            description: Set("".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag2: tags::Model = tag2.insert(&db).await.unwrap();

        let tag3 = tags::ActiveModel {
            id: NotSet,
            name: Set("タグ3".to_string()),
            description: Set("".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag3: tags::Model = tag3.insert(&db).await.unwrap();

        // タグ1,2付きのタスクを作成
        let new_task = Task::new(
            0,
            "テストタスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![
                Tag::from(inserted_tag1.clone()),
                Tag::from(inserted_tag2.clone()),
            ],
            None,
        );
        let created_task = task_repo.create(&new_task).await.unwrap();

        assert_eq!(created_task.tags.len(), 2);

        // タグをタグ3のみに置換
        let mut updated_task = created_task.clone();
        updated_task.tags = vec![Tag::from(inserted_tag3.clone())];

        let result = task_repo.update(&updated_task).await.unwrap();

        // タグが置換されたことを確認
        assert_eq!(result.tags.len(), 1);
        assert_eq!(result.tags[0].id, inserted_tag3.id);
    }

    #[tokio::test]
    async fn test_update_task_due_date() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        // 期限なしのタスクを作成
        let new_task = Task::new(
            0,
            "テストタスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let created_task = repo.create(&new_task).await.unwrap();

        assert!(created_task.due_date.is_none());

        // 期限を追加
        let mut updated_task = created_task.clone();
        updated_task.due_date = Some(chrono::NaiveDate::from_ymd_opt(2026, 12, 31).unwrap());

        let result = repo.update(&updated_task).await.unwrap();

        // 期限が追加されたことを確認
        assert!(result.due_date.is_some());
        assert_eq!(
            result.due_date.unwrap(),
            chrono::NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()
        );
    }

    #[tokio::test]
    async fn test_update_task_not_existing() {
        let db = setup_test_db().await;
        let repo = TaskRepository::new(&db);

        // 存在しないタスクを更新しようとする
        let non_existing_task = Task::new(
            999,
            "タスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );

        let result = repo.update(&non_existing_task).await;

        // エラーが返されることを確認
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("タスクが存在しません")
        );
    }
}
