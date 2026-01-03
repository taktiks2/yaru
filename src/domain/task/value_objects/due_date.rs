use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// タスクの期限を表すValue Object
///
/// 期限はNaiveDateで表現されます。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DueDate(NaiveDate);

impl DueDate {
    /// 新しいDueDateを作成（スタブ - テスト失敗のため）
    pub fn new(_date: NaiveDate) -> Result<Self> {
        unimplemented!("DueDate::new() は未実装です")
    }

    /// 期限の値を取得（スタブ - テスト失敗のため）
    pub fn value(&self) -> NaiveDate {
        unimplemented!("DueDate::value() は未実装です")
    }

    /// 指定された日付より前かチェック（スタブ - テスト失敗のため）
    pub fn is_before(&self, _other: NaiveDate) -> bool {
        unimplemented!("DueDate::is_before() は未実装です")
    }

    /// 指定された日付より後かチェック（スタブ - テスト失敗のため）
    pub fn is_after(&self, _other: NaiveDate) -> bool {
        unimplemented!("DueDate::is_after() は未実装です")
    }
}

// テストのみを先に作成（TDD）
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_due_date_valid() {
        let date = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        let due_date = DueDate::new(date).unwrap();
        assert_eq!(due_date.value(), date);
    }

    #[test]
    fn test_due_date_past() {
        // 過去の日付も許可される
        let date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let due_date = DueDate::new(date).unwrap();
        assert_eq!(due_date.value(), date);
    }

    #[test]
    fn test_due_date_today() {
        let today = chrono::Local::now().date_naive();
        let due_date = DueDate::new(today).unwrap();
        assert_eq!(due_date.value(), today);
    }

    #[test]
    fn test_due_date_is_before() {
        let today = chrono::Local::now().date_naive();
        let yesterday = today - chrono::Duration::days(1);
        let due_date = DueDate::new(yesterday).unwrap();
        assert!(due_date.is_before(today));
        assert!(!due_date.is_before(yesterday));
    }

    #[test]
    fn test_due_date_is_after() {
        let today = chrono::Local::now().date_naive();
        let tomorrow = today + chrono::Duration::days(1);
        let due_date = DueDate::new(tomorrow).unwrap();
        assert!(due_date.is_after(today));
        assert!(!due_date.is_after(tomorrow));
    }

    #[test]
    fn test_due_date_equality() {
        let date = NaiveDate::from_ymd_opt(2026, 6, 15).unwrap();
        let due_date1 = DueDate::new(date).unwrap();
        let due_date2 = DueDate::new(date).unwrap();
        assert_eq!(due_date1, due_date2);
    }

    #[test]
    fn test_due_date_ordering() {
        let date1 = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        let due_date1 = DueDate::new(date1).unwrap();
        let due_date2 = DueDate::new(date2).unwrap();
        assert!(due_date1 < due_date2);
    }

    #[test]
    fn test_due_date_copy() {
        let date = NaiveDate::from_ymd_opt(2026, 5, 1).unwrap();
        let due_date1 = DueDate::new(date).unwrap();
        let due_date2 = due_date1; // Copy trait
        assert_eq!(due_date1, due_date2);
    }

    #[test]
    fn test_due_date_serialize() {
        let date = NaiveDate::from_ymd_opt(2026, 3, 15).unwrap();
        let due_date = DueDate::new(date).unwrap();
        let json = serde_json::to_string(&due_date).unwrap();
        assert_eq!(json, "\"2026-03-15\"");
    }

    #[test]
    fn test_due_date_deserialize() {
        let due_date: DueDate = serde_json::from_str("\"2026-03-15\"").unwrap();
        let expected_date = NaiveDate::from_ymd_opt(2026, 3, 15).unwrap();
        assert_eq!(due_date.value(), expected_date);
    }
}
