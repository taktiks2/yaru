use anyhow::Result;
use serde::{Deserialize, Serialize};

/// タスクのステータスを表すValue Object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
pub enum Status {
    /// 保留中
    Pending,
    /// 進行中
    InProgress,
    /// 完了
    Completed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Status {
    /// 文字列から変換
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "Pending" => Ok(Status::Pending),
            "InProgress" => Ok(Status::InProgress),
            "Completed" => Ok(Status::Completed),
            _ => anyhow::bail!("無効なステータス: {}", s),
        }
    }

    /// フィルタ値から変換
    pub fn from_filter_value(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "pending" | "todo" => Ok(Status::Pending),
            "in_progress" | "progress" => Ok(Status::InProgress),
            "completed" | "done" => Ok(Status::Completed),
            _ => anyhow::bail!("無効なフィルタ値: {}", s),
        }
    }

    /// 文字列表現を取得
    pub fn as_str(&self) -> &str {
        match self {
            Status::Pending => "Pending",
            Status::InProgress => "InProgress",
            Status::Completed => "Completed",
        }
    }

    /// 日本語表示名を取得
    pub fn display_name(&self) -> &str {
        match self {
            Status::Pending => "保留中",
            Status::InProgress => "進行中",
            Status::Completed => "完了",
        }
    }
}

// テストのみを先に作成（TDD）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_from_string_pending() {
        let status = Status::from_str("Pending").unwrap();
        assert_eq!(status, Status::Pending);
    }

    #[test]
    fn test_status_from_string_in_progress() {
        let status = Status::from_str("InProgress").unwrap();
        assert_eq!(status, Status::InProgress);
    }

    #[test]
    fn test_status_from_string_completed() {
        let status = Status::from_str("Completed").unwrap();
        assert_eq!(status, Status::Completed);
    }

    #[test]
    fn test_status_from_filter_value_pending() {
        let status = Status::from_filter_value("pending").unwrap();
        assert_eq!(status, Status::Pending);
    }

    #[test]
    fn test_status_from_filter_value_todo() {
        let status = Status::from_filter_value("todo").unwrap();
        assert_eq!(status, Status::Pending);
    }

    #[test]
    fn test_status_from_filter_value_in_progress() {
        let status = Status::from_filter_value("in_progress").unwrap();
        assert_eq!(status, Status::InProgress);
    }

    #[test]
    fn test_status_from_filter_value_progress() {
        let status = Status::from_filter_value("progress").unwrap();
        assert_eq!(status, Status::InProgress);
    }

    #[test]
    fn test_status_from_filter_value_completed() {
        let status = Status::from_filter_value("completed").unwrap();
        assert_eq!(status, Status::Completed);
    }

    #[test]
    fn test_status_from_filter_value_done() {
        let status = Status::from_filter_value("done").unwrap();
        assert_eq!(status, Status::Completed);
    }

    #[test]
    fn test_status_from_filter_value_invalid() {
        let result = Status::from_filter_value("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_status_to_string() {
        assert_eq!(Status::Pending.as_str(), "Pending");
        assert_eq!(Status::InProgress.as_str(), "InProgress");
        assert_eq!(Status::Completed.as_str(), "Completed");
    }

    #[test]
    fn test_status_display() {
        assert_eq!(Status::Pending.display_name(), "保留中");
        assert_eq!(Status::InProgress.display_name(), "進行中");
        assert_eq!(Status::Completed.display_name(), "完了");
    }

    #[test]
    fn test_status_copy() {
        let status1 = Status::Pending;
        let status2 = status1; // Copy trait
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_status_equality() {
        assert_eq!(Status::Pending, Status::Pending);
        assert_ne!(Status::Pending, Status::InProgress);
    }

    #[test]
    fn test_status_serialize() {
        let status = Status::Pending;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"Pending\"");
    }

    #[test]
    fn test_status_deserialize() {
        let status: Status = serde_json::from_str("\"Pending\"").unwrap();
        assert_eq!(status, Status::Pending);
    }
}
