use anyhow::{Result, bail};
use std::sync::{Arc, RwLock};

use crate::domain::task::{
    aggregate::TaskAggregate, repository::TaskRepository, value_objects::TaskId,
};

/// InMemoryTaskRepository - テスト用のタスクリポジトリ実装
///
/// メモリ上にタスクを保持します。本番環境では使用しないでください。
#[derive(Clone)]
pub struct InMemoryTaskRepository {
    tasks: Arc<RwLock<Vec<TaskAggregate>>>,
    next_id: Arc<RwLock<i32>>,
}

impl InMemoryTaskRepository {
    /// 新しいInMemoryTaskRepositoryを作成
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(Vec::new())),
            next_id: Arc::new(RwLock::new(1)),
        }
    }

    /// 次のIDを生成
    fn generate_id(&self) -> Result<i32> {
        let mut next_id = self.next_id.write().unwrap();
        let id = *next_id;
        *next_id += 1;
        Ok(id)
    }
}

impl Default for InMemoryTaskRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskAggregate>> {
        let tasks = self.tasks.read().unwrap();
        Ok(tasks.iter().find(|t| t.id() == id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<TaskAggregate>> {
        let tasks = self.tasks.read().unwrap();
        Ok(tasks.clone())
    }

    async fn save(&self, task: TaskAggregate) -> Result<TaskAggregate> {
        let task_to_save = if task.id().value() == 0 {
            // IDが0の場合、新しいIDを割り当てる
            let new_id = self.generate_id()?;
            task.with_id(TaskId::new(new_id)?)
        } else {
            task
        };

        let mut tasks = self.tasks.write().unwrap();

        // 既存のタスクがあれば更新、なければ追加
        if let Some(index) = tasks.iter().position(|t| t.id() == task_to_save.id()) {
            tasks[index] = task_to_save.clone();
        } else {
            tasks.push(task_to_save.clone());
        }

        Ok(task_to_save)
    }

    async fn update(&self, task: TaskAggregate) -> Result<TaskAggregate> {
        let mut tasks = self.tasks.write().unwrap();

        if let Some(index) = tasks.iter().position(|t| t.id() == task.id()) {
            tasks[index] = task.clone();
            Ok(task)
        } else {
            bail!("タスクが見つかりません: {}", task.id().value())
        }
    }

    async fn delete(&self, id: &TaskId) -> Result<bool> {
        let mut tasks = self.tasks.write().unwrap();

        if let Some(index) = tasks.iter().position(|t| t.id() == id) {
            tasks.remove(index);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::value_objects::{Priority, Status, TaskDescription, TaskTitle};

    #[tokio::test]
    async fn test_in_memory_repository_new() {
        let repo = InMemoryTaskRepository::new();
        let tasks = repo.find_all().await.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_find_all_empty() {
        let repo = InMemoryTaskRepository::new();
        let tasks = repo.find_all().await.unwrap();
        assert!(tasks.is_empty());
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let repo = InMemoryTaskRepository::new();
        let id = TaskId::new(1).unwrap();
        let result = repo.find_by_id(&id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_save_new_task() {
        let repo = InMemoryTaskRepository::new();

        let task = TaskAggregate::new(
            TaskTitle::new("新しいタスク").unwrap(),
            TaskDescription::new("説明").unwrap(),
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );

        let saved = repo.save(task).await.unwrap();

        // IDが割り当てられていることを確認
        assert_ne!(saved.id().value(), 0);
        assert_eq!(saved.title().value(), "新しいタスク");

        // リポジトリに保存されていることを確認
        let found = repo.find_by_id(saved.id()).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), saved.id());
    }

    #[tokio::test]
    async fn test_save_multiple_tasks() {
        let repo = InMemoryTaskRepository::new();

        let task1 = TaskAggregate::new(
            TaskTitle::new("タスク1").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );

        let task2 = TaskAggregate::new(
            TaskTitle::new("タスク2").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::InProgress,
            Priority::Medium,
            vec![],
            None,
        );

        let saved1 = repo.save(task1).await.unwrap();
        let saved2 = repo.save(task2).await.unwrap();

        // 異なるIDが割り当てられていることを確認
        assert_ne!(saved1.id(), saved2.id());

        // 両方のタスクが保存されていることを確認
        let all_tasks = repo.find_all().await.unwrap();
        assert_eq!(all_tasks.len(), 2);
    }

    #[tokio::test]
    async fn test_update_task() {
        let repo = InMemoryTaskRepository::new();

        let task = TaskAggregate::new(
            TaskTitle::new("元のタイトル").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );

        let saved = repo.save(task).await.unwrap();
        let mut updated_task = saved.clone();
        updated_task
            .change_title(TaskTitle::new("更新後のタイトル").unwrap())
            .unwrap();

        let result = repo.update(updated_task.clone()).await.unwrap();

        assert_eq!(result.title().value(), "更新後のタイトル");

        // 更新が反映されていることを確認
        let found = repo.find_by_id(saved.id()).await.unwrap().unwrap();
        assert_eq!(found.title().value(), "更新後のタイトル");
    }

    #[tokio::test]
    async fn test_delete_task() {
        let repo = InMemoryTaskRepository::new();

        let task = TaskAggregate::new(
            TaskTitle::new("削除するタスク").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );

        let saved = repo.save(task).await.unwrap();
        let deleted = repo.delete(saved.id()).await.unwrap();

        assert!(deleted);

        // 削除されたことを確認
        let found = repo.find_by_id(saved.id()).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_delete_nonexistent_task() {
        let repo = InMemoryTaskRepository::new();
        let id = TaskId::new(999).unwrap();

        let deleted = repo.delete(&id).await.unwrap();

        assert!(!deleted);
    }
}
