use crate::{
    application::dto::{CreateTaskDTO, TagInfo, TaskDTO},
    domain::{
        tag::repository::TagRepository,
        task::{
            aggregate::TaskAggregate,
            repository::TaskRepository,
            value_objects::{Priority, Status, TaskDescription, TaskTitle},
        },
    },
};
use anyhow::{Result, bail};
use std::{collections::HashMap, sync::Arc};

/// AddTaskUseCase - タスク追加のユースケース
///
/// 新しいタスクを作成してリポジトリに保存します。
pub struct AddTaskUseCase {
    task_repository: Arc<dyn TaskRepository>,
    tag_repository: Arc<dyn TagRepository>,
}

impl AddTaskUseCase {
    /// 新しいAddTaskUseCaseを作成
    pub fn new(
        task_repository: Arc<dyn TaskRepository>,
        tag_repository: Arc<dyn TagRepository>,
    ) -> Self {
        Self {
            task_repository,
            tag_repository,
        }
    }

    /// タスクを追加する
    ///
    /// # Arguments
    /// * `dto` - タスク作成時の入力DTO
    ///
    /// # Returns
    /// * `Ok(TaskDTO)` - 作成されたタスク
    /// * `Err` - エラーが発生した場合
    pub async fn execute(&self, dto: CreateTaskDTO) -> Result<TaskDTO> {
        // タイトルのバリデーション
        let title = TaskTitle::new(dto.title)?;

        // 説明のバリデーション
        let description = if let Some(desc) = dto.description {
            TaskDescription::new(desc)?
        } else {
            TaskDescription::new("")?
        };

        // ステータスの変換（デフォルト: Pending）
        // Display形式（"InProgress"）とフィルタ形式（"in_progress"）の両方をサポート
        let status = if let Some(status_str) = dto.status {
            Status::from_str_anyhow(&status_str)
                .or_else(|_| Status::from_filter_value(&status_str))?
        } else {
            Status::Pending
        };

        // 優先度の変換（デフォルト: Medium）
        let priority = if let Some(priority_str) = dto.priority {
            Priority::from_str_anyhow(&priority_str)?
        } else {
            Priority::Medium
        };

        // タグの存在確認（一括）
        if !dto.tags.is_empty() {
            let tag_id_vos: Result<Vec<_>> = dto
                .tags
                .iter()
                .map(|id| {
                    use crate::domain::tag::value_objects::TagId;
                    TagId::new(*id)
                })
                .collect();
            let tag_id_vos = tag_id_vos?;

            let found_tags = self.tag_repository.find_by_ids(&tag_id_vos).await?;

            if found_tags.len() != dto.tags.len() {
                // どのIDが見つからなかったか特定
                let found_ids: std::collections::HashSet<i32> =
                    found_tags.iter().map(|tag| tag.id().value()).collect();

                for tag_id in &dto.tags {
                    if !found_ids.contains(tag_id) {
                        bail!("タグID {}は存在しません", tag_id);
                    }
                }
            }
        }

        // タグIDのValue Objectに変換
        let tag_ids: Result<Vec<_>> = dto
            .tags
            .iter()
            .map(|id| {
                use crate::domain::tag::value_objects::TagId;
                TagId::new(*id)
            })
            .collect();
        let tag_ids = tag_ids?;

        // 期限日の変換
        let due_date = if let Some(date) = dto.due_date {
            use crate::domain::task::value_objects::DueDate;
            Some(DueDate::new(date)?)
        } else {
            None
        };

        // TaskAggregateを作成
        let task = TaskAggregate::new(title, description, status, priority, tag_ids, due_date);

        // リポジトリに保存
        let saved_task = self.task_repository.save(task).await?;

        // タグ情報を取得（既に検証済みなので安全）
        let tag_ids: Vec<_> = saved_task.tags().clone();
        let tags = if !tag_ids.is_empty() {
            self.tag_repository.find_by_ids(&tag_ids).await?
        } else {
            Vec::new()
        };

        // タグマップを作成
        let tag_map: HashMap<_, _> = tags.iter().map(|tag| (tag.id().value(), tag)).collect();

        // タグ詳細を解決
        let tag_details = saved_task
            .tags()
            .iter()
            .filter_map(|tag_id| {
                tag_map.get(&tag_id.value()).map(|tag| TagInfo {
                    id: tag.id().value(),
                    name: tag.name().value().to_string(),
                })
            })
            .collect();

        // DTOに変換して返す
        let mut dto = TaskDTO::from(saved_task);
        dto.tags = tag_details;
        Ok(dto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tag::{
        aggregate::TagAggregate, value_objects::TagDescription, value_objects::TagName,
    };
    use crate::interface::persistence::in_memory::{InMemoryTagRepository, InMemoryTaskRepository};

    #[tokio::test]
    async fn test_add_task_minimal() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = AddTaskUseCase::new(task_repo.clone(), tag_repo);

        let dto = CreateTaskDTO {
            title: "新しいタスク".to_string(),
            description: None,
            status: None,
            priority: None,
            tags: vec![],
            due_date: None,
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.title, "新しいタスク");
        assert_eq!(task.description, None);
        assert_eq!(task.status, "pending");
        assert_eq!(task.priority, "medium");
        assert!(task.tags.is_empty());
        assert!(task.due_date.is_none());

        // リポジトリに保存されていることを確認
        let all_tasks = task_repo.find_all().await.unwrap();
        assert_eq!(all_tasks.len(), 1);
    }

    #[tokio::test]
    async fn test_add_task_full() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        // タグを事前に作成
        let tag = TagAggregate::new(
            TagName::new("重要").unwrap(),
            TagDescription::new("重要なタスク").unwrap(),
        );
        let saved_tag = tag_repo.save(tag).await.unwrap();

        let use_case = AddTaskUseCase::new(task_repo.clone(), tag_repo);

        let dto = CreateTaskDTO {
            title: "詳細タスク".to_string(),
            description: Some("詳細な説明".to_string()),
            status: Some("in_progress".to_string()),
            priority: Some("high".to_string()),
            tags: vec![saved_tag.id().value()],
            due_date: Some(chrono::NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.title, "詳細タスク");
        assert_eq!(task.description, Some("詳細な説明".to_string()));
        assert_eq!(task.status, "in_progress");
        assert_eq!(task.priority, "high");
        assert_eq!(task.tags.len(), 1);
        assert_eq!(task.tags[0].id, saved_tag.id().value());
        assert_eq!(task.tags[0].name, saved_tag.name().value());
        assert!(task.due_date.is_some());
    }

    #[tokio::test]
    async fn test_add_task_with_invalid_tag() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = AddTaskUseCase::new(task_repo, tag_repo);

        let dto = CreateTaskDTO {
            title: "タスク".to_string(),
            description: None,
            status: None,
            priority: None,
            tags: vec![999], // 存在しないタグID
            due_date: None,
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("存在しません"));
    }

    #[tokio::test]
    async fn test_add_task_with_empty_title() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = AddTaskUseCase::new(task_repo, tag_repo);

        let dto = CreateTaskDTO {
            title: "".to_string(),
            description: None,
            status: None,
            priority: None,
            tags: vec![],
            due_date: None,
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_task_with_invalid_priority() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = AddTaskUseCase::new(task_repo, tag_repo);

        let dto = CreateTaskDTO {
            title: "タスク".to_string(),
            description: None,
            status: None,
            priority: Some("invalid".to_string()),
            tags: vec![],
            due_date: None,
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid priority"));
    }
}
