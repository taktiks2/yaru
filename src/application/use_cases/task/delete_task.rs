use crate::domain::task::{repository::TaskRepository, value_objects::TaskId};
use anyhow::{Result, bail};
use std::sync::Arc;

/// DeleteTaskUseCase - タスク削除のユースケース
///
/// 指定されたIDのタスクを削除します。
pub struct DeleteTaskUseCase {
    task_repository: Arc<dyn TaskRepository>,
}

impl DeleteTaskUseCase {
    /// 新しいDeleteTaskUseCaseを作成
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }

    /// タスクを削除する
    ///
    /// # Arguments
    /// * `id` - 削除するタスクのID
    ///
    /// # Returns
    /// * `Ok(())` - タスクが削除された場合
    /// * `Err` - エラーが発生した場合（タスクが見つからない場合を含む）
    pub async fn execute(&self, id: i32) -> Result<()> {
        let task_id = TaskId::new(id)?;

        // タスクの存在確認
        if self.task_repository.find_by_id(&task_id).await?.is_none() {
            bail!("タスクID {}は存在しません", id);
        }

        // タスクを削除
        let deleted = self.task_repository.delete(&task_id).await?;

        if deleted {
            Ok(())
        } else {
            bail!("タスクの削除に失敗しました: {}", id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::{
        aggregate::TaskAggregate,
        value_objects::{Priority, Status, TaskDescription, TaskTitle},
    };
    use crate::interface::persistence::in_memory::InMemoryTaskRepository;

    #[tokio::test]
    async fn test_delete_task_success() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("削除するタスク").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = DeleteTaskUseCase::new(task_repo.clone());

        // Act
        let result = use_case.execute(saved_task.id().value()).await;

        // Assert
        assert!(result.is_ok());

        // タスクが削除されていることを確認
        let found = task_repo.find_by_id(saved_task.id()).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_delete_task_not_found() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let use_case = DeleteTaskUseCase::new(task_repo);

        // Act
        let result = use_case.execute(999).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("存在しません"));
    }

    #[tokio::test]
    async fn test_delete_task_with_invalid_id() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let use_case = DeleteTaskUseCase::new(task_repo);

        // Act
        let result = use_case.execute(0).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_multiple_tasks() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());

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
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );

        let saved1 = task_repo.save(task1).await.unwrap();
        let saved2 = task_repo.save(task2).await.unwrap();

        let use_case = DeleteTaskUseCase::new(task_repo.clone());

        // Act - タスク1を削除
        let result1 = use_case.execute(saved1.id().value()).await;

        // Assert
        assert!(result1.is_ok());

        // タスク1が削除され、タスク2は残っていることを確認
        let all_tasks = task_repo.find_all().await.unwrap();
        assert_eq!(all_tasks.len(), 1);
        assert_eq!(all_tasks[0].id(), saved2.id());
    }
}
