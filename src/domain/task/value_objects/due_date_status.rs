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

// テストのみを先に作成（TDD）
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
        assert_eq!(DueDateStatus::Overdue.display_name(), "期限切れ");
        assert_eq!(DueDateStatus::DueToday.display_name(), "今日期限");
        assert_eq!(DueDateStatus::DueThisWeek.display_name(), "今週期限");
        assert_eq!(DueDateStatus::NoDueDate.display_name(), "期限なし");
    }
}

impl DueDateStatus {
    /// 表示用の日本語名を取得
    pub fn display_name(&self) -> &str {
        match self {
            DueDateStatus::Overdue => "期限切れ",
            DueDateStatus::DueToday => "今日期限",
            DueDateStatus::DueThisWeek => "今週期限",
            DueDateStatus::NoDueDate => "期限なし",
        }
    }
}
