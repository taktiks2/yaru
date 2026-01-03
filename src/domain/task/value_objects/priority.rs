use anyhow::Result;
use serde::{Deserialize, Serialize};

/// タスクの優先度を表すValue Object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Priority {
    /// 低
    Low = 1,
    /// 中
    Medium = 2,
    /// 高
    High = 3,
    /// 重大
    Critical = 4,
}

impl Priority {
    /// 文字列から変換
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "Low" => Ok(Priority::Low),
            "Medium" => Ok(Priority::Medium),
            "High" => Ok(Priority::High),
            "Critical" => Ok(Priority::Critical),
            _ => anyhow::bail!("無効な優先度: {}", s),
        }
    }

    /// 文字列表現を取得
    pub fn as_str(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Critical => "Critical",
        }
    }

    /// 日本語表示名を取得
    pub fn display_name(&self) -> &str {
        match self {
            Priority::Low => "低",
            Priority::Medium => "中",
            Priority::High => "高",
            Priority::Critical => "重大",
        }
    }
}

// テストのみを先に作成（TDD）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_from_string_low() {
        let priority = Priority::from_str("Low").unwrap();
        assert_eq!(priority, Priority::Low);
    }

    #[test]
    fn test_priority_from_string_medium() {
        let priority = Priority::from_str("Medium").unwrap();
        assert_eq!(priority, Priority::Medium);
    }

    #[test]
    fn test_priority_from_string_high() {
        let priority = Priority::from_str("High").unwrap();
        assert_eq!(priority, Priority::High);
    }

    #[test]
    fn test_priority_from_string_critical() {
        let priority = Priority::from_str("Critical").unwrap();
        assert_eq!(priority, Priority::Critical);
    }

    #[test]
    fn test_priority_from_string_invalid() {
        let result = Priority::from_str("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_priority_to_string() {
        assert_eq!(Priority::Low.as_str(), "Low");
        assert_eq!(Priority::Medium.as_str(), "Medium");
        assert_eq!(Priority::High.as_str(), "High");
        assert_eq!(Priority::Critical.as_str(), "Critical");
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(Priority::Low.display_name(), "低");
        assert_eq!(Priority::Medium.display_name(), "中");
        assert_eq!(Priority::High.display_name(), "高");
        assert_eq!(Priority::Critical.display_name(), "重大");
    }

    #[test]
    fn test_priority_copy() {
        let priority1 = Priority::Low;
        let priority2 = priority1; // Copy trait
        assert_eq!(priority1, priority2);
    }

    #[test]
    fn test_priority_equality() {
        assert_eq!(Priority::Low, Priority::Low);
        assert_ne!(Priority::Low, Priority::High);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Low < Priority::Medium);
        assert!(Priority::Medium < Priority::High);
        assert!(Priority::High < Priority::Critical);
    }

    #[test]
    fn test_priority_serialize() {
        let priority = Priority::Low;
        let json = serde_json::to_string(&priority).unwrap();
        assert_eq!(json, "\"Low\"");
    }

    #[test]
    fn test_priority_deserialize() {
        let priority: Priority = serde_json::from_str("\"High\"").unwrap();
        assert_eq!(priority, Priority::High);
    }
}
