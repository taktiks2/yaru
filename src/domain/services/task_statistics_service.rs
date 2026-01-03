use std::collections::HashMap;

use chrono::NaiveDate;

use crate::domain::task::{
    aggregate::TaskAggregate,
    value_objects::{DueDateStatus, Priority, Status, TaskStats},
};

/// TaskStatisticsService - タスクの統計情報を計算するドメインサービス
///
/// 複数のAggregateにまたがる統計計算ロジックを実装します。
/// ステートレスなサービスとして設計されています。
pub struct TaskStatisticsService;

impl TaskStatisticsService {
    /// タスクリストから統計情報を計算
    ///
    /// # Arguments
    /// * `tasks` - 統計を計算するタスクのリスト
    /// * `today` - 基準日（期限切れ判定などに使用）
    ///
    /// # Returns
    /// * `TaskStats` - 計算された統計情報
    pub fn calculate_stats(tasks: &[TaskAggregate], today: NaiveDate) -> TaskStats {
        let total_count = tasks.len();

        let mut status_stats: HashMap<Status, usize> = HashMap::new();
        let mut priority_stats: HashMap<Priority, usize> = HashMap::new();
        let mut due_date_stats: HashMap<DueDateStatus, usize> = HashMap::new();
        let mut tag_stats: HashMap<String, usize> = HashMap::new();
        let mut priority_status_matrix: HashMap<(Priority, Status), usize> = HashMap::new();

        for task in tasks {
            // ステータス別カウント
            *status_stats.entry(*task.status()).or_default() += 1;

            // 優先度別カウント
            *priority_stats.entry(*task.priority()).or_default() += 1;

            // 期限関連カウント (完了済みタスクは除外)
            if task.status() != &Status::Completed {
                if let Some(due_date) = task.due_date() {
                    let due = due_date.value();
                    if due < today {
                        *due_date_stats.entry(DueDateStatus::Overdue).or_default() += 1;
                    } else if due == today {
                        *due_date_stats.entry(DueDateStatus::DueToday).or_default() += 1;
                    } else {
                        let week_later = today + chrono::Duration::days(7);
                        if due <= week_later {
                            *due_date_stats
                                .entry(DueDateStatus::DueThisWeek)
                                .or_default() += 1;
                        }
                    }
                } else {
                    *due_date_stats.entry(DueDateStatus::NoDueDate).or_default() += 1;
                }
            }

            // タグ別統計
            if task.tags().is_empty() {
                *tag_stats.entry("(タグなし)".to_string()).or_default() += 1;
            } else {
                for tag_id in task.tags() {
                    // 注: タグ名を取得するにはTagRepositoryが必要
                    // ここではIDの文字列表現を使用
                    let tag_key = format!("Tag ID: {}", tag_id.value());
                    *tag_stats.entry(tag_key).or_default() += 1;
                }
            }

            // 優先度×ステータス クロス集計
            *priority_status_matrix
                .entry((*task.priority(), *task.status()))
                .or_default() += 1;
        }

        TaskStats::new(
            status_stats,
            priority_stats,
            due_date_stats,
            tag_stats,
            priority_status_matrix,
            total_count,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::value_objects::{DueDate, TaskDescription, TaskId, TaskTitle};
    use crate::domain::tag::value_objects::TagId;
    use chrono::{Duration, Utc};

    #[test]
    fn test_calculate_stats_empty_tasks() {
        // Arrange
        let tasks = vec![];
        let today = Utc::now().naive_utc().date();

        // Act
        let stats = TaskStatisticsService::calculate_stats(&tasks, today);

        // Assert
        assert_eq!(stats.total_count(), 0);
        assert_eq!(stats.status_count(&Status::Pending), 0);
        assert_eq!(stats.priority_count(&Priority::Medium), 0);
    }

    #[test]
    fn test_calculate_stats_status_counts() {
        // Arrange
        let today = Utc::now().naive_utc().date();
        let tasks = vec![
            create_test_task(Status::Pending, Priority::Medium, None),
            create_test_task(Status::Pending, Priority::High, None),
            create_test_task(Status::Completed, Priority::Low, None),
        ];

        // Act
        let stats = TaskStatisticsService::calculate_stats(&tasks, today);

        // Assert
        assert_eq!(stats.total_count(), 3);
        assert_eq!(stats.status_count(&Status::Pending), 2);
        assert_eq!(stats.status_count(&Status::Completed), 1);
        assert_eq!(stats.status_count(&Status::InProgress), 0);
    }

    #[test]
    fn test_calculate_stats_priority_counts() {
        // Arrange
        let today = Utc::now().naive_utc().date();
        let tasks = vec![
            create_test_task(Status::Pending, Priority::High, None),
            create_test_task(Status::Pending, Priority::High, None),
            create_test_task(Status::Completed, Priority::Low, None),
            create_test_task(Status::InProgress, Priority::Medium, None),
        ];

        // Act
        let stats = TaskStatisticsService::calculate_stats(&tasks, today);

        // Assert
        assert_eq!(stats.total_count(), 4);
        assert_eq!(stats.priority_count(&Priority::High), 2);
        assert_eq!(stats.priority_count(&Priority::Low), 1);
        assert_eq!(stats.priority_count(&Priority::Medium), 1);
        assert_eq!(stats.priority_count(&Priority::Critical), 0);
    }

    #[test]
    fn test_calculate_stats_due_date_counts() {
        // Arrange
        let today = Utc::now().naive_utc().date();
        let overdue_date = today - Duration::days(1);
        let today_date = today;
        let this_week_date = today + Duration::days(3);

        let tasks = vec![
            create_test_task(
                Status::Pending,
                Priority::High,
                Some(DueDate::new(overdue_date).unwrap()),
            ),
            create_test_task(
                Status::Pending,
                Priority::Medium,
                Some(DueDate::new(today_date).unwrap()),
            ),
            create_test_task(
                Status::Pending,
                Priority::Low,
                Some(DueDate::new(this_week_date).unwrap()),
            ),
            create_test_task(Status::Pending, Priority::Medium, None),
        ];

        // Act
        let stats = TaskStatisticsService::calculate_stats(&tasks, today);

        // Assert
        assert_eq!(stats.due_date_count(&DueDateStatus::Overdue), 1);
        assert_eq!(stats.due_date_count(&DueDateStatus::DueToday), 1);
        assert_eq!(stats.due_date_count(&DueDateStatus::DueThisWeek), 1);
        assert_eq!(stats.due_date_count(&DueDateStatus::NoDueDate), 1);
    }

    #[test]
    fn test_calculate_stats_priority_status_matrix() {
        // Arrange
        let today = Utc::now().naive_utc().date();
        let tasks = vec![
            create_test_task(Status::Pending, Priority::High, None),
            create_test_task(Status::Pending, Priority::High, None),
            create_test_task(Status::Completed, Priority::High, None),
            create_test_task(Status::InProgress, Priority::Medium, None),
        ];

        // Act
        let stats = TaskStatisticsService::calculate_stats(&tasks, today);

        // Assert
        assert_eq!(
            stats.priority_status_count(&Priority::High, &Status::Pending),
            2
        );
        assert_eq!(
            stats.priority_status_count(&Priority::High, &Status::Completed),
            1
        );
        assert_eq!(
            stats.priority_status_count(&Priority::Medium, &Status::InProgress),
            1
        );
        assert_eq!(
            stats.priority_status_count(&Priority::Low, &Status::Pending),
            0
        );
    }

    // Helper function to create test tasks
    fn create_test_task(
        status: Status,
        priority: Priority,
        due_date: Option<DueDate>,
    ) -> TaskAggregate {
        let title = TaskTitle::new("Test Task").unwrap();
        let description = TaskDescription::new("").unwrap();
        TaskAggregate::new(title, description, status, priority, vec![], due_date)
    }
}
