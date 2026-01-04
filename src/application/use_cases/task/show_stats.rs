use crate::{
    application::dto::StatsDTO,
    domain::{
        services::TaskStatisticsService, tag::repository::TagRepository,
        task::repository::TaskRepository,
    },
};
use anyhow::Result;
use chrono::Utc;
use std::{collections::HashMap, sync::Arc};

/// ShowStatsUseCase - タスク統計表示のユースケース
///
/// 全タスクの統計情報を計算して返します。
pub struct ShowStatsUseCase {
    task_repository: Arc<dyn TaskRepository>,
    tag_repository: Arc<dyn TagRepository>,
}

impl ShowStatsUseCase {
    /// 新しいShowStatsUseCaseを作成
    pub fn new(
        task_repository: Arc<dyn TaskRepository>,
        tag_repository: Arc<dyn TagRepository>,
    ) -> Self {
        Self {
            task_repository,
            tag_repository,
        }
    }

    /// タスクの統計情報を取得する
    ///
    /// # Returns
    /// * `Ok(StatsDTO)` - 統計情報
    /// * `Err` - エラーが発生した場合
    pub async fn execute(&self) -> Result<StatsDTO> {
        // 全タスクを取得
        let tasks = self.task_repository.find_all().await?;

        // 今日の日付を取得
        let today = Utc::now().naive_utc().date();

        // TaskStatisticsServiceで統計を計算
        let stats = TaskStatisticsService::calculate_stats(&tasks, today);

        // タグIDからタグ名へのマッピングを作成
        let all_tags = self.tag_repository.find_all().await?;
        let tag_names: HashMap<_, _> = all_tags
            .into_iter()
            .map(|tag| (*tag.id(), tag.name().value().to_string()))
            .collect();

        // DTOに変換（タグ名マップ付き）
        Ok(StatsDTO::from_task_stats_with_tag_names(stats, tag_names))
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
    use chrono::{Duration, Utc};

    #[tokio::test]
    async fn test_show_stats_empty() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = ShowStatsUseCase::new(task_repo, tag_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_count, 0);
        assert!(stats.status_stats.is_empty());
        assert!(stats.priority_stats.is_empty());
    }

    #[tokio::test]
    async fn test_show_stats_single_task() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task = TaskAggregate::new(
            TaskTitle::new("タスク1").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );
        task_repo.save(task).await.unwrap();

        let use_case = ShowStatsUseCase::new(task_repo, tag_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_count, 1);
        assert_eq!(stats.status_stats.get("pending"), Some(&1));
        assert_eq!(stats.priority_stats.get("high"), Some(&1));
    }

    #[tokio::test]
    async fn test_show_stats_multiple_tasks() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        // Pendingタスク（High）
        let task1 = TaskAggregate::new(
            TaskTitle::new("タスク1").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );

        // InProgressタスク（Medium）
        let task2 = TaskAggregate::new(
            TaskTitle::new("タスク2").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::InProgress,
            Priority::Medium,
            vec![],
            None,
        );

        // Completedタスク（Low）
        let mut task3 = TaskAggregate::new(
            TaskTitle::new("タスク3").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );
        task3.complete().unwrap();

        task_repo.save(task1).await.unwrap();
        task_repo.save(task2).await.unwrap();
        task_repo.save(task3).await.unwrap();

        let use_case = ShowStatsUseCase::new(task_repo, tag_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_count, 3);
        assert_eq!(stats.status_stats.get("pending"), Some(&1));
        assert_eq!(stats.status_stats.get("in_progress"), Some(&1));
        assert_eq!(stats.status_stats.get("completed"), Some(&1));
        assert_eq!(stats.priority_stats.get("high"), Some(&1));
        assert_eq!(stats.priority_stats.get("medium"), Some(&1));
        assert_eq!(stats.priority_stats.get("low"), Some(&1));
    }

    #[tokio::test]
    async fn test_show_stats_with_overdue_tasks() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let past_date = Utc::now().naive_utc().date() - Duration::days(1);
        let task = TaskAggregate::new(
            TaskTitle::new("期限切れタスク").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::High,
            vec![],
            Some(DueDate::new(past_date).unwrap()),
        );
        task_repo.save(task).await.unwrap();

        let use_case = ShowStatsUseCase::new(task_repo, tag_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_count, 1);
        assert_eq!(stats.due_date_stats.get("overdue"), Some(&1));
    }

    #[tokio::test]
    async fn test_show_stats_with_tags() {
        // Arrange
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let task1 = TaskAggregate::new(
            TaskTitle::new("タスク1").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![TagId::new(1).unwrap()],
            None,
        );

        let task2 = TaskAggregate::new(
            TaskTitle::new("タスク2").unwrap(),
            TaskDescription::new("").unwrap(),
            Status::Pending,
            Priority::Medium,
            vec![TagId::new(1).unwrap(), TagId::new(2).unwrap()],
            None,
        );

        task_repo.save(task1).await.unwrap();
        task_repo.save(task2).await.unwrap();

        let use_case = ShowStatsUseCase::new(task_repo, tag_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.total_count, 2);
        // タグの統計情報は含まれる（ただし、タグ名ではなくIDで集計される）
    }
}
