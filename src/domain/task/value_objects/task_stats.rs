use crate::domain::tag::value_objects::TagId;
use crate::domain::task::value_objects::{DueDateStatus, Priority, Status};
use std::collections::HashMap;

/// タスクの統計情報を表すValue Object
///
/// 統計情報は不変のデータとして扱われます。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStats {
    /// ステータス別統計
    status_stats: HashMap<Status, usize>,
    /// 優先度別統計
    priority_stats: HashMap<Priority, usize>,
    /// 期限関連統計
    due_date_stats: HashMap<DueDateStatus, usize>,
    /// タグ別統計（None: タグなし、Some(TagId): 該当タグID）
    tag_stats: HashMap<Option<TagId>, usize>,
    /// 優先度×ステータス クロス集計
    priority_status_matrix: HashMap<(Priority, Status), usize>,
    /// 全体統計
    total_count: usize,
}

impl TaskStats {
    /// 新しいTaskStatsを作成
    pub fn new(
        status_stats: HashMap<Status, usize>,
        priority_stats: HashMap<Priority, usize>,
        due_date_stats: HashMap<DueDateStatus, usize>,
        tag_stats: HashMap<Option<TagId>, usize>,
        priority_status_matrix: HashMap<(Priority, Status), usize>,
        total_count: usize,
    ) -> Self {
        Self {
            status_stats,
            priority_stats,
            due_date_stats,
            tag_stats,
            priority_status_matrix,
            total_count,
        }
    }

    /// 総タスク数を取得
    pub fn total_count(&self) -> usize {
        self.total_count
    }

    /// ステータス別タスク数を取得
    pub fn status_count(&self, status: &Status) -> usize {
        self.status_stats.get(status).copied().unwrap_or(0)
    }

    /// 優先度別タスク数を取得
    pub fn priority_count(&self, priority: &Priority) -> usize {
        self.priority_stats.get(priority).copied().unwrap_or(0)
    }

    /// 期限ステータス別タスク数を取得
    pub fn due_date_count(&self, due_date_status: &DueDateStatus) -> usize {
        self.due_date_stats
            .get(due_date_status)
            .copied()
            .unwrap_or(0)
    }

    /// 優先度×ステータスのクロス集計を取得
    pub fn priority_status_count(&self, priority: &Priority, status: &Status) -> usize {
        self.priority_status_matrix
            .get(&(*priority, *status))
            .copied()
            .unwrap_or(0)
    }

    /// タグ別統計の生のHashMapを取得
    pub fn tag_stats(&self) -> &HashMap<Option<TagId>, usize> {
        &self.tag_stats
    }
}

// テストのみを先に作成（TDD）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_stats_new() {
        let status_stats = HashMap::new();
        let priority_stats = HashMap::new();
        let due_date_stats = HashMap::new();
        let tag_stats = HashMap::new();
        let priority_status_matrix = HashMap::new();

        let stats = TaskStats::new(
            status_stats.clone(),
            priority_stats.clone(),
            due_date_stats.clone(),
            tag_stats.clone(),
            priority_status_matrix.clone(),
            0,
        );

        assert_eq!(stats.total_count(), 0);
    }

    #[test]
    fn test_task_stats_with_data() {
        let mut status_stats = HashMap::new();
        status_stats.insert(Status::Pending, 5);
        status_stats.insert(Status::InProgress, 3);
        status_stats.insert(Status::Completed, 2);

        let mut priority_stats = HashMap::new();
        priority_stats.insert(Priority::Low, 2);
        priority_stats.insert(Priority::High, 8);

        let stats = TaskStats::new(
            status_stats,
            priority_stats,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            10,
        );

        assert_eq!(stats.total_count(), 10);
        assert_eq!(stats.status_count(&Status::Pending), 5);
        assert_eq!(stats.status_count(&Status::InProgress), 3);
        assert_eq!(stats.status_count(&Status::Completed), 2);
        assert_eq!(stats.priority_count(&Priority::Low), 2);
        assert_eq!(stats.priority_count(&Priority::High), 8);
    }

    #[test]
    fn test_task_stats_status_count_missing() {
        let stats = TaskStats::new(
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            0,
        );

        assert_eq!(stats.status_count(&Status::Pending), 0);
    }

    #[test]
    fn test_task_stats_due_date_count() {
        let mut due_date_stats = HashMap::new();
        due_date_stats.insert(DueDateStatus::Overdue, 3);
        due_date_stats.insert(DueDateStatus::DueToday, 1);

        let stats = TaskStats::new(
            HashMap::new(),
            HashMap::new(),
            due_date_stats,
            HashMap::new(),
            HashMap::new(),
            4,
        );

        assert_eq!(stats.due_date_count(&DueDateStatus::Overdue), 3);
        assert_eq!(stats.due_date_count(&DueDateStatus::DueToday), 1);
        assert_eq!(stats.due_date_count(&DueDateStatus::NoDueDate), 0);
    }

    #[test]
    fn test_task_stats_priority_status_matrix() {
        let mut matrix = HashMap::new();
        matrix.insert((Priority::High, Status::Pending), 5);
        matrix.insert((Priority::Low, Status::Completed), 2);

        let stats = TaskStats::new(
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            matrix,
            7,
        );

        assert_eq!(
            stats.priority_status_count(&Priority::High, &Status::Pending),
            5
        );
        assert_eq!(
            stats.priority_status_count(&Priority::Low, &Status::Completed),
            2
        );
        assert_eq!(
            stats.priority_status_count(&Priority::Medium, &Status::InProgress),
            0
        );
    }

    #[test]
    fn test_task_stats_clone() {
        let stats1 = TaskStats::new(
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            5,
        );
        let stats2 = stats1.clone();
        assert_eq!(stats1, stats2);
    }
}
