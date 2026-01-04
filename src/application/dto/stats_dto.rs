use crate::domain::task::value_objects::{DueDateStatus, Priority, Status, TaskStats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 統計情報の読み取り専用表現（DTO）
///
/// TaskStatsをPresentation層で扱いやすい形式に変換します。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatsDTO {
    /// ステータス別統計（キーは文字列）
    pub status_stats: HashMap<String, usize>,
    /// 優先度別統計（キーは文字列）
    pub priority_stats: HashMap<String, usize>,
    /// 期限関連統計（キーは文字列）
    pub due_date_stats: HashMap<String, usize>,
    /// タグ別統計
    pub tag_stats: HashMap<String, usize>,
    /// 優先度×ステータス クロス集計（キーは "priority:status" 形式）
    pub priority_status_matrix: HashMap<String, usize>,
    /// 全体統計
    pub total_count: usize,
}

// TaskStatsからStatsDTOへの変換
impl From<TaskStats> for StatsDTO {
    fn from(stats: TaskStats) -> Self {
        // ステータス別統計を文字列キーに変換
        let mut status_stats = HashMap::new();
        for status in [Status::Pending, Status::InProgress, Status::Completed] {
            let count = stats.status_count(&status);
            if count > 0 {
                status_stats.insert(status_to_string(&status), count);
            }
        }

        // 優先度別統計を文字列キーに変換
        let mut priority_stats = HashMap::new();
        for priority in [
            Priority::Low,
            Priority::Medium,
            Priority::High,
            Priority::Critical,
        ] {
            let count = stats.priority_count(&priority);
            if count > 0 {
                priority_stats.insert(priority_to_string(&priority), count);
            }
        }

        // 期限関連統計を文字列キーに変換
        let mut due_date_stats = HashMap::new();
        for due_date_status in [
            DueDateStatus::Overdue,
            DueDateStatus::DueToday,
            DueDateStatus::DueThisWeek,
            DueDateStatus::NoDueDate,
        ] {
            let count = stats.due_date_count(&due_date_status);
            if count > 0 {
                due_date_stats.insert(due_date_status_to_string(&due_date_status), count);
            }
        }

        // タグ別統計（すでに文字列キー）
        let mut tag_stats = HashMap::new();
        for tag_name in stats.all_tag_names() {
            tag_stats.insert(tag_name.clone(), stats.tag_count(&tag_name));
        }

        // 優先度×ステータス クロス集計を文字列キーに変換
        let mut priority_status_matrix = HashMap::new();
        for priority in [
            Priority::Low,
            Priority::Medium,
            Priority::High,
            Priority::Critical,
        ] {
            for status in [Status::Pending, Status::InProgress, Status::Completed] {
                let count = stats.priority_status_count(&priority, &status);
                if count > 0 {
                    let key = format!(
                        "{}:{}",
                        priority_to_string(&priority),
                        status_to_string(&status)
                    );
                    priority_status_matrix.insert(key, count);
                }
            }
        }

        Self {
            status_stats,
            priority_stats,
            due_date_stats,
            tag_stats,
            priority_status_matrix,
            total_count: stats.total_count(),
        }
    }
}

// ヘルパー関数
fn status_to_string(status: &Status) -> String {
    match status {
        Status::Pending => "pending".to_string(),
        Status::InProgress => "in_progress".to_string(),
        Status::Completed => "completed".to_string(),
    }
}

fn priority_to_string(priority: &Priority) -> String {
    match priority {
        Priority::Low => "low".to_string(),
        Priority::Medium => "medium".to_string(),
        Priority::High => "high".to_string(),
        Priority::Critical => "critical".to_string(),
    }
}

fn due_date_status_to_string(due_date_status: &DueDateStatus) -> String {
    match due_date_status {
        DueDateStatus::Overdue => "overdue".to_string(),
        DueDateStatus::DueToday => "due_today".to_string(),
        DueDateStatus::DueThisWeek => "due_this_week".to_string(),
        DueDateStatus::NoDueDate => "no_due_date".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::value_objects::{DueDateStatus, Priority, Status};

    #[test]
    fn test_stats_dto_from_task_stats() {
        let mut status_stats = HashMap::new();
        status_stats.insert(Status::Pending, 5);
        status_stats.insert(Status::Completed, 3);

        let mut priority_stats = HashMap::new();
        priority_stats.insert(Priority::High, 4);
        priority_stats.insert(Priority::Low, 4);

        let mut due_date_stats = HashMap::new();
        due_date_stats.insert(DueDateStatus::Overdue, 2);

        let mut tag_stats = HashMap::new();
        tag_stats.insert("重要".to_string(), 5);

        let mut priority_status_matrix = HashMap::new();
        priority_status_matrix.insert((Priority::High, Status::Pending), 3);

        let task_stats = TaskStats::new(
            status_stats,
            priority_stats,
            due_date_stats,
            tag_stats,
            priority_status_matrix,
            8,
        );

        let dto = StatsDTO::from(task_stats);

        assert_eq!(dto.total_count, 8);
        assert_eq!(dto.status_stats.get("pending"), Some(&5));
        assert_eq!(dto.status_stats.get("completed"), Some(&3));
        assert_eq!(dto.priority_stats.get("high"), Some(&4));
        assert_eq!(dto.priority_stats.get("low"), Some(&4));
        assert_eq!(dto.due_date_stats.get("overdue"), Some(&2));
        assert_eq!(dto.tag_stats.get("重要"), Some(&5));
        assert_eq!(dto.priority_status_matrix.get("high:pending"), Some(&3));
    }

    #[test]
    fn test_stats_dto_empty() {
        let task_stats = TaskStats::new(
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            0,
        );

        let dto = StatsDTO::from(task_stats);

        assert_eq!(dto.total_count, 0);
        assert!(dto.status_stats.is_empty());
        assert!(dto.priority_stats.is_empty());
        assert!(dto.due_date_stats.is_empty());
        assert!(dto.tag_stats.is_empty());
        assert!(dto.priority_status_matrix.is_empty());
    }

    #[test]
    fn test_stats_dto_only_total_count() {
        let mut status_stats = HashMap::new();
        status_stats.insert(Status::Pending, 10);

        let task_stats = TaskStats::new(
            status_stats,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            10,
        );

        let dto = StatsDTO::from(task_stats);

        assert_eq!(dto.total_count, 10);
        assert_eq!(dto.status_stats.get("pending"), Some(&10));
        assert!(dto.priority_stats.is_empty());
    }
}
