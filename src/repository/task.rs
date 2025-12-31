use crate::{
    domain::task::{Priority, Status, Task},
    entity::prelude::*,
    entity::{task_tags, tasks},
    repository::Repository,
};
use anyhow::{Context, Result};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set, TransactionTrait,
};

/// Task用のリポジトリ実装
pub struct TaskRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> TaskRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Entityからドメインモデルへ変換
    async fn entity_to_domain(&self, model: tasks::Model) -> Result<Task> {
        let task_tag_models = TaskTags::find()
            .filter(task_tags::Column::TaskId.eq(model.id))
            .all(self.db)
            .await
            .context("タグIDの読み込みに失敗しました")?;

        let tag_ids: Vec<i32> = task_tag_models.into_iter().map(|tt| tt.tag_id).collect();

        Ok(Task {
            id: model.id,
            title: model.title,
            description: model.description,
            status: Status::from_filter_value(&model.status).unwrap_or(Status::Pending),
            priority: match model.priority.as_str() {
                "Low" => Priority::Low,
                "Medium" => Priority::Medium,
                "High" => Priority::High,
                "Critical" => Priority::Critical,
                _ => Priority::Medium,
            },
            tags: tag_ids,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        })
    }
}

impl<'a> Repository<Task> for TaskRepository<'a> {
    async fn find_by_id(&self, id: i32) -> Result<Option<Task>> {
        let task_model = Tasks::find_by_id(id)
            .one(self.db)
            .await
            .context("タスクの検索に失敗しました")?;

        match task_model {
            Some(model) => Ok(Some(self.entity_to_domain(model).await?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<Task>> {
        let task_models = Tasks::find()
            .all(self.db)
            .await
            .context("タスクの読み込みに失敗しました")?;

        let mut tasks = Vec::new();
        for model in task_models {
            tasks.push(self.entity_to_domain(model).await?);
        }

        Ok(tasks)
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

        // タスクを新規作成
        let new_task = tasks::ActiveModel {
            id: NotSet, // AUTO INCREMENT
            title: Set(item.title.clone()),
            description: Set(item.description.clone()),
            status: Set(format!("{:?}", item.status)),
            priority: Set(format!("{:?}", item.priority)),
            created_at: NotSet,
            updated_at: NotSet,
        };

        let inserted_task = new_task
            .insert(&txn)
            .await
            .context("タスクの挿入に失敗しました")?;

        // タグの関連付けを保存
        for tag_id in &item.tags {
            let task_tag = task_tags::ActiveModel {
                task_id: Set(inserted_task.id),
                tag_id: Set(*tag_id),
            };
            task_tag
                .insert(&txn)
                .await
                .context("タスクタグの挿入に失敗しました")?;
        }

        txn.commit()
            .await
            .context("トランザクションのコミットに失敗しました")?;

        self.entity_to_domain(inserted_task).await
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
}

#[cfg(test)]
mod tests {
    use super::*;
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
        use crate::entity::tags;
        use sea_orm::ActiveValue::NotSet;
        let tag1 = tags::ActiveModel {
            id: NotSet,
            name: Set("重要".to_string()),
            description: Set("重要なタスク".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag1 = tag1.insert(&db).await.unwrap();

        let tag2 = tags::ActiveModel {
            id: NotSet,
            name: Set("緊急".to_string()),
            description: Set("緊急タスク".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag2 = tag2.insert(&db).await.unwrap();

        // タグ付きタスクを作成
        let new_task = Task::new(
            0,
            "タグ付きタスク",
            "説明",
            Status::InProgress,
            Priority::High,
            vec![inserted_tag1.id, inserted_tag2.id],
        );

        let created_task = task_repo.create(&new_task).await.unwrap();

        assert!(created_task.id > 0);
        assert_eq!(created_task.title, "タグ付きタスク");
        assert_eq!(created_task.tags.len(), 2);
        assert!(created_task.tags.contains(&(inserted_tag1.id)));
        assert!(created_task.tags.contains(&(inserted_tag2.id)));
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
        );
        let task2 = Task::new(
            0,
            "タスク2",
            "説明2",
            Status::InProgress,
            Priority::Medium,
            vec![],
        );
        let task3 = Task::new(
            0,
            "タスク3",
            "説明3",
            Status::Completed,
            Priority::High,
            vec![],
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
        use crate::entity::tags;
        use sea_orm::ActiveValue::NotSet;
        let tag = tags::ActiveModel {
            id: NotSet,
            name: Set("テストタグ".to_string()),
            description: Set("説明".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };
        let inserted_tag = tag.insert(&db).await.unwrap();

        // タグ付きタスクを作成
        let new_task = Task::new(
            0,
            "タグ削除テスト",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![inserted_tag.id],
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
        );
        let task2 = Task::new(
            0,
            "タスク2",
            "説明2",
            Status::Completed,
            Priority::Medium,
            vec![],
        );
        let task3 = Task::new(
            0,
            "タスク3",
            "説明3",
            Status::Completed,
            Priority::High,
            vec![],
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
}
