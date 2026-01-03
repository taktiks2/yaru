use crate::{application::dto::TaskDTO, domain::task::repository::TaskRepository};
use anyhow::Result;
use std::sync::Arc;

/// ListTasksUseCase - タスク一覧取得のユースケース
///
/// 全タスクを取得してDTOに変換します。
pub struct ListTasksUseCase {
    task_repository: Arc<dyn TaskRepository>,
}

impl ListTasksUseCase {
    /// 新しいListTasksUseCaseを作成
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }

    /// タスク一覧を取得する
    ///
    /// # Returns
    /// * `Ok(Vec<TaskDTO>)` - タスクのリスト
    /// * `Err` - エラーが発生した場合
    pub async fn execute(&self) -> Result<Vec<TaskDTO>> {
        let tasks = self.task_repository.find_all().await?;
        Ok(tasks.into_iter().map(TaskDTO::from).collect())
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
    async fn test_list_tasks_empty() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let use_case = ListTasksUseCase::new(task_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert!(tasks.is_empty());
    }

    #[tokio::test]
    async fn test_list_tasks_single() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タスク1").unwrap(),
            TaskDescription::new("説明1").unwrap(),
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );
        task_repo.save(task).await.unwrap();

        let use_case = ListTasksUseCase::new(task_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "タスク1");
        assert_eq!(tasks[0].description, Some("説明1".to_string()));
        assert_eq!(tasks[0].status, "pending");
        assert_eq!(tasks[0].priority, "high");
    }

    #[tokio::test]
    async fn test_list_tasks_multiple() {
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
            Status::InProgress,
            Priority::Medium,
            vec![],
            None,
        );

        let task3 = TaskAggregate::new(
            TaskTitle::new("タスク3").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Completed,
            Priority::High,
            vec![],
            None,
        );

        task_repo.save(task1).await.unwrap();
        task_repo.save(task2).await.unwrap();
        task_repo.save(task3).await.unwrap();

        let use_case = ListTasksUseCase::new(task_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 3);

        // タイトルでソートされていないので、存在することだけを確認
        let titles: Vec<String> = tasks.iter().map(|t| t.title.clone()).collect();
        assert!(titles.contains(&"タスク1".to_string()));
        assert!(titles.contains(&"タスク2".to_string()));
        assert!(titles.contains(&"タスク3".to_string()));
    }

    #[tokio::test]
    async fn test_list_tasks_with_different_statuses() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());

        let pending_task = TaskAggregate::new(
            TaskTitle::new("Pendingタスク").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );

        let in_progress_task = TaskAggregate::new(
            TaskTitle::new("InProgressタスク").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::InProgress,
            Priority::Medium,
            vec![],
            None,
        );

        let completed_task = TaskAggregate::new(
            TaskTitle::new("Completedタスク").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Completed,
            Priority::Medium,
            vec![],
            None,
        );

        task_repo.save(pending_task).await.unwrap();
        task_repo.save(in_progress_task).await.unwrap();
        task_repo.save(completed_task).await.unwrap();

        let use_case = ListTasksUseCase::new(task_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 3);

        // すべてのステータスが含まれていることを確認
        let statuses: Vec<String> = tasks.iter().map(|t| t.status.clone()).collect();
        assert!(statuses.contains(&"pending".to_string()));
        assert!(statuses.contains(&"in_progress".to_string()));
        assert!(statuses.contains(&"completed".to_string()));
    }
}
