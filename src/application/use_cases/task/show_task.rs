use crate::{
    application::dto::task_dto::{TagInfo, TaskDTO},
    domain::{
        tag::repository::TagRepository,
        task::{repository::TaskRepository, value_objects::TaskId},
    },
};
use anyhow::Result;
use std::{collections::HashMap, sync::Arc};

/// ShowTaskUseCase - タスク詳細取得のユースケース
///
/// 指定されたIDのタスクの詳細を取得します。
/// タグ情報はTagRepositoryから取得します。
pub struct ShowTaskUseCase {
    task_repository: Arc<dyn TaskRepository>,
    tag_repository: Arc<dyn TagRepository>,
}

impl ShowTaskUseCase {
    /// 新しいShowTaskUseCaseを作成
    pub fn new(
        task_repository: Arc<dyn TaskRepository>,
        tag_repository: Arc<dyn TagRepository>,
    ) -> Self {
        Self {
            task_repository,
            tag_repository,
        }
    }

    /// タスクの詳細を取得する
    ///
    /// # Arguments
    /// * `id` - 取得するタスクのID
    ///
    /// # Returns
    /// * `Ok(TaskDTO)` - タスクの詳細
    /// * `Err` - エラーが発生した場合（タスクが見つからない場合を含む）
    pub async fn execute(&self, id: i32) -> Result<TaskDTO> {
        let task_id = TaskId::new(id)?;

        let task = self
            .task_repository
            .find_by_id(&task_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("タスクID {}は存在しません", id))?;

        // タグ情報を取得
        let tag_ids: Vec<_> = task.tags().clone();
        let tags = self.tag_repository.find_by_ids(&tag_ids).await?;

        // タグマップを作成
        let tag_map: HashMap<_, _> = tags.iter().map(|tag| (tag.id().value(), tag)).collect();

        // TaskDTOに変換（タグ詳細を含む）
        let tag_details = task
            .tags()
            .iter()
            .filter_map(|tag_id| {
                tag_map.get(&tag_id.value()).map(|tag| TagInfo {
                    id: tag.id().value(),
                    name: tag.name().value().to_string(),
                })
            })
            .collect();

        let mut dto = TaskDTO::from(task);
        dto.tags = tag_details;

        Ok(dto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tag::value_objects::TagId;
    use crate::domain::task::{
        aggregate::TaskAggregate,
        value_objects::{DueDate, Priority, Status, TaskDescription, TaskTitle},
    };
    use crate::interface::persistence::in_memory::{InMemoryTagRepository, InMemoryTaskRepository};
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_show_task_success() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("表示するタスク").unwrap(),
            TaskDescription::new("詳細な説明").unwrap(),
            Status::InProgress,
            Priority::High,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = ShowTaskUseCase::new(task_repo, tag_repo);

        // Act
        let result = use_case.execute(saved_task.id().value()).await;

        // Assert
        assert!(result.is_ok());
        let task_dto = result.unwrap();
        assert_eq!(task_dto.id, saved_task.id().value());
        assert_eq!(task_dto.title, "表示するタスク");
        assert_eq!(task_dto.description, Some("詳細な説明".to_string()));
        assert_eq!(task_dto.status, "in_progress");
        assert_eq!(task_dto.priority, "high");
    }

    #[tokio::test]
    async fn test_show_task_not_found() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = ShowTaskUseCase::new(task_repo, tag_repo);

        // Act
        let result = use_case.execute(999).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("存在しません"));
    }

    #[tokio::test]
    async fn test_show_task_with_invalid_id() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = ShowTaskUseCase::new(task_repo, tag_repo);

        // Act
        let result = use_case.execute(0).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_show_task_with_tags() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タグ付きタスク").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![TagId::new(1).unwrap(), TagId::new(2).unwrap()],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = ShowTaskUseCase::new(task_repo, tag_repo);

        // Act
        let result = use_case.execute(saved_task.id().value()).await;

        // Assert
        assert!(result.is_ok());
        let task_dto = result.unwrap();
        // From実装ではタグ情報は空（タグが存在しないため）
        assert_eq!(task_dto.tags, Vec::<TagInfo>::new());
    }

    #[tokio::test]
    async fn test_show_task_with_due_date() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let due_date = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        let task = TaskAggregate::new(
            TaskTitle::new("期限付きタスク").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::High,
            vec![],
            Some(DueDate::new(due_date).unwrap()),
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = ShowTaskUseCase::new(task_repo, tag_repo);

        // Act
        let result = use_case.execute(saved_task.id().value()).await;

        // Assert
        assert!(result.is_ok());
        let task_dto = result.unwrap();
        assert_eq!(task_dto.due_date, Some(due_date));
    }
}
