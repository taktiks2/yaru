use serde::{Deserialize, Serialize};

/// 期限の状況を表すValue Object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DueDateStatus {
    /// 期限切れ
    Overdue,
    /// 今日期限
    DueToday,
    /// 今週期限（7日以内）
    DueThisWeek,
    /// 期限なし
    NoDueDate,
}

impl DueDateStatus {
    /// Get display name
    #[allow(dead_code)]
    pub fn display_name(&self) -> &str {
        match self {
            DueDateStatus::Overdue => "Overdue",
            DueDateStatus::DueToday => "Due Today",
            DueDateStatus::DueThisWeek => "Due This Week",
            DueDateStatus::NoDueDate => "No Due Date",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_due_date_status_equality() {
        assert_eq!(DueDateStatus::Overdue, DueDateStatus::Overdue);
        assert_ne!(DueDateStatus::Overdue, DueDateStatus::DueToday);
    }

    #[test]
    fn test_due_date_status_copy() {
        let status1 = DueDateStatus::Overdue;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_due_date_status_serialize() {
        let status = DueDateStatus::Overdue;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"Overdue\"");
    }

    #[test]
    fn test_due_date_status_deserialize() {
        let status: DueDateStatus = serde_json::from_str("\"DueToday\"").unwrap();
        assert_eq!(status, DueDateStatus::DueToday);
    }

    #[test]
    fn test_due_date_status_display() {
        assert_eq!(DueDateStatus::Overdue.display_name(), "Overdue");
        assert_eq!(DueDateStatus::DueToday.display_name(), "Due Today");
        assert_eq!(DueDateStatus::DueThisWeek.display_name(), "Due This Week");
        assert_eq!(DueDateStatus::NoDueDate.display_name(), "No Due Date");
    }
}
