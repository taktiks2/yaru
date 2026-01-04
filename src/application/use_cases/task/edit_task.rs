use crate::{
    application::dto::{TaskDTO, UpdateTaskDTO},
    domain::{
        tag::{repository::TagRepository, value_objects::TagId},
        task::{
            repository::TaskRepository,
            value_objects::{DueDate, Priority, Status, TaskDescription, TaskId, TaskTitle},
        },
    },
};
use anyhow::{Result, bail};
use std::sync::Arc;

/// EditTaskUseCase - タスク更新のユースケース
///
/// 既存のタスクを部分更新します。
pub struct EditTaskUseCase {
    task_repository: Arc<dyn TaskRepository>,
    tag_repository: Arc<dyn TagRepository>,
}

impl EditTaskUseCase {
    /// 新しいEditTaskUseCaseを作成
    pub fn new(
        task_repository: Arc<dyn TaskRepository>,
        tag_repository: Arc<dyn TagRepository>,
    ) -> Self {
        Self {
            task_repository,
            tag_repository,
        }
    }

    /// タスクを更新する
    ///
    /// # Arguments
    /// * `id` - 更新するタスクのID
    /// * `dto` - タスク更新時の入力DTO
    ///
    /// # Returns
    /// * `Ok(TaskDTO)` - 更新されたタスク
    /// * `Err` - エラーが発生した場合
    pub async fn execute(&self, id: i32, dto: UpdateTaskDTO) -> Result<TaskDTO> {
        let task_id = TaskId::new(id)?;

        // タスクを取得
        let mut task = self
            .task_repository
            .find_by_id(&task_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("タスクID {}は存在しません", id))?;

        // タイトルの更新
        if let Some(title_str) = dto.title {
            let title = TaskTitle::new(title_str)?;
            task.change_title(title)?;
        }

        // 説明の更新
        if let Some(description_str) = dto.description {
            let description = TaskDescription::new(description_str)?;
            task.change_description(description)?;
        }

        // ステータスの更新
        // Display形式（"InProgress"）とフィルタ形式（"in_progress"）の両方をサポート
        if let Some(status_str) = dto.status {
            let status = Status::from_str_anyhow(&status_str)
                .or_else(|_| Status::from_filter_value(&status_str))?;
            task.change_status(status)?;
        }

        // 優先度の更新
        if let Some(priority_str) = dto.priority {
            let priority = parse_priority(&priority_str)?;
            task.change_priority(priority)?;
        }

        // タグの更新
        if let Some(tag_ids) = dto.tags {
            // タグの存在確認（一括）
            if !tag_ids.is_empty() {
                let tag_id_vos: Result<Vec<_>> = tag_ids.iter().map(|id| TagId::new(*id)).collect();
                let tag_id_vos = tag_id_vos?;

                let found_tags = self.tag_repository.find_by_ids(&tag_id_vos).await?;

                if found_tags.len() != tag_ids.len() {
                    // どのIDが見つからなかったか特定
                    let found_ids: std::collections::HashSet<i32> =
                        found_tags.iter().map(|tag| tag.id().value()).collect();

                    for tag_id in &tag_ids {
                        if !found_ids.contains(tag_id) {
                            bail!("タグID {}は存在しません", tag_id);
                        }
                    }
                }
            }

            // タグIDのValue Objectに変換
            let tag_id_vos: Result<Vec<_>> = tag_ids.iter().map(|id| TagId::new(*id)).collect();
            let tag_id_vos = tag_id_vos?;

            // 既存のタグをすべて削除して新しいタグを追加
            task.replace_tags(tag_id_vos)?;
        }

        // 期限日の更新
        if let Some(due_date) = dto.due_date {
            let due_date_vo = Some(DueDate::new(due_date)?);
            task.change_due_date(due_date_vo)?;
        }

        // リポジトリに保存
        let updated_task = self.task_repository.update(task).await?;

        // DTOに変換して返す
        Ok(TaskDTO::from(updated_task))
    }
}

// ヘルパー関数: 文字列からPriorityに変換
fn parse_priority(priority_str: &str) -> Result<Priority> {
    match priority_str.to_lowercase().as_str() {
        "low" => Ok(Priority::Low),
        "medium" => Ok(Priority::Medium),
        "high" => Ok(Priority::High),
        "critical" => Ok(Priority::Critical),
        _ => bail!("無効な優先度: {}", priority_str),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::task::{
            aggregate::TaskAggregate,
            value_objects::{Priority, Status, TaskDescription, TaskTitle},
        },
        interface::persistence::in_memory::{InMemoryTagRepository, InMemoryTaskRepository},
    };

    #[tokio::test]
    async fn test_edit_task_title() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("元のタイトル").unwrap(),
            TaskDescription::new("説明").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo.clone(), tag_repo);

        let dto = UpdateTaskDTO {
            title: Some("更新後のタイトル".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_task = result.unwrap();
        assert_eq!(updated_task.title, "更新後のタイトル");
        assert_eq!(updated_task.description, Some("説明".to_string())); // 変更なし
    }

    #[tokio::test]
    async fn test_edit_task_not_found() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = EditTaskUseCase::new(task_repo, tag_repo);

        let dto = UpdateTaskDTO {
            title: Some("新しいタイトル".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(999, dto).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("存在しません"));
    }

    #[tokio::test]
    async fn test_edit_task_invalid_id() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = EditTaskUseCase::new(task_repo, tag_repo);

        let dto = UpdateTaskDTO {
            title: Some("新しいタイトル".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(0, dto).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_task_empty_update() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タイトル").unwrap(),
            TaskDescription::new("説明").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo.clone(), tag_repo);

        let dto = UpdateTaskDTO::default(); // 何も更新しない

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_task = result.unwrap();
        // 変更なし
        assert_eq!(updated_task.title, "タイトル");
        assert_eq!(updated_task.description, Some("説明".to_string()));
    }

    #[tokio::test]
    async fn test_edit_task_with_invalid_title() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タイトル").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo, tag_repo);

        let dto = UpdateTaskDTO {
            title: Some("".to_string()), // 空のタイトル
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_task_description() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タイトル").unwrap(),
            TaskDescription::new("元の説明").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo.clone(), tag_repo);

        let dto = UpdateTaskDTO {
            description: Some("更新後の説明".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_task = result.unwrap();
        assert_eq!(updated_task.description, Some("更新後の説明".to_string()));
        assert_eq!(updated_task.title, "タイトル"); // 変更なし
    }

    #[tokio::test]
    async fn test_edit_task_status() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タイトル").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo.clone(), tag_repo);

        let dto = UpdateTaskDTO {
            status: Some("in_progress".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_task = result.unwrap();
        assert_eq!(updated_task.status, "in_progress");
    }

    #[tokio::test]
    async fn test_edit_task_priority() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タイトル").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo.clone(), tag_repo);

        let dto = UpdateTaskDTO {
            priority: Some("high".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_task = result.unwrap();
        assert_eq!(updated_task.priority, "high");
    }

    #[tokio::test]
    async fn test_edit_task_priority_invalid() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タイトル").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo, tag_repo);

        let dto = UpdateTaskDTO {
            priority: Some("invalid".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("無効な優先度"));
    }

    #[tokio::test]
    async fn test_edit_task_tags() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        // タグを作成
        use crate::domain::tag::{
            aggregate::TagAggregate,
            value_objects::{TagDescription, TagName},
        };
        let tag1 = TagAggregate::new(
            TagName::new("タグ1").unwrap(),
            TagDescription::new("").unwrap(),
        );
        let tag2 = TagAggregate::new(
            TagName::new("タグ2").unwrap(),
            TagDescription::new("").unwrap(),
        );
        let saved_tag1 = tag_repo.save(tag1).await.unwrap();
        let saved_tag2 = tag_repo.save(tag2).await.unwrap();

        // タスクを作成
        let task = TaskAggregate::new(
            TaskTitle::new("タイトル").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo.clone(), tag_repo);

        let dto = UpdateTaskDTO {
            tags: Some(vec![saved_tag1.id().value(), saved_tag2.id().value()]),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_task = result.unwrap();
        assert_eq!(updated_task.tags.len(), 2);
        assert!(updated_task.tags.contains(&saved_tag1.id().value()));
        assert!(updated_task.tags.contains(&saved_tag2.id().value()));
    }

    #[tokio::test]
    async fn test_edit_task_tags_nonexistent() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タイトル").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo, tag_repo);

        let dto = UpdateTaskDTO {
            tags: Some(vec![999]), // 存在しないタグID
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("タグID 999は存在しません")
        );
    }

    #[tokio::test]
    async fn test_edit_task_due_date() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タイトル").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo.clone(), tag_repo);

        use chrono::NaiveDate;
        let new_due_date = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();

        let dto = UpdateTaskDTO {
            due_date: Some(new_due_date),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_task = result.unwrap();
        assert_eq!(updated_task.due_date, Some(new_due_date));
    }

    #[tokio::test]
    async fn test_edit_task_multiple_fields() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("元のタイトル").unwrap(),
            TaskDescription::new("元の説明").unwrap(),
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );
        let saved_task = task_repo.save(task).await.unwrap();

        let use_case = EditTaskUseCase::new(task_repo.clone(), tag_repo);

        use chrono::NaiveDate;
        let new_due_date = NaiveDate::from_ymd_opt(2026, 6, 30).unwrap();

        let dto = UpdateTaskDTO {
            title: Some("新しいタイトル".to_string()),
            description: Some("新しい説明".to_string()),
            status: Some("in_progress".to_string()),
            priority: Some("critical".to_string()),
            due_date: Some(new_due_date),
            tags: None,
        };

        // Act
        let result = use_case.execute(saved_task.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_task = result.unwrap();
        assert_eq!(updated_task.title, "新しいタイトル");
        assert_eq!(updated_task.description, Some("新しい説明".to_string()));
        assert_eq!(updated_task.status, "in_progress");
        assert_eq!(updated_task.priority, "critical");
        assert_eq!(updated_task.due_date, Some(new_due_date));
    }
}
