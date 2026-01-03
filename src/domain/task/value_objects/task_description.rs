use anyhow::Result;

/// タスクの説明を表すValue Object
///
/// 説明は任意の文字列です。空文字列も許可されます。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDescription(String);

impl TaskDescription {
    /// 新しいTaskDescriptionを作成
    pub fn new(value: impl Into<String>) -> Result<Self> {
        Ok(Self(value.into()))
    }

    /// 説明の値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Default for TaskDescription {
    fn default() -> Self {
        Self(String::new())
    }
}

// テストのみを先に作成（TDD）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_description_valid() {
        let desc = TaskDescription::new("これはタスクの説明です").unwrap();
        assert_eq!(desc.value(), "これはタスクの説明です");
    }

    #[test]
    fn test_task_description_empty() {
        let desc = TaskDescription::new("").unwrap();
        assert_eq!(desc.value(), "");
    }

    #[test]
    fn test_task_description_long_text() {
        let long_text = "a".repeat(1000);
        let desc = TaskDescription::new(&long_text).unwrap();
        assert_eq!(desc.value(), long_text);
    }

    #[test]
    fn test_task_description_multiline() {
        let multiline = "行1\n行2\n行3";
        let desc = TaskDescription::new(multiline).unwrap();
        assert_eq!(desc.value(), multiline);
    }

    #[test]
    fn test_task_description_whitespace() {
        let desc = TaskDescription::new("   ").unwrap();
        assert_eq!(desc.value(), "   ");
    }

    #[test]
    fn test_task_description_equality() {
        let desc1 = TaskDescription::new("同じ説明").unwrap();
        let desc2 = TaskDescription::new("同じ説明").unwrap();
        assert_eq!(desc1, desc2);
    }

    #[test]
    fn test_task_description_clone() {
        let desc1 = TaskDescription::new("説明").unwrap();
        let desc2 = desc1.clone();
        assert_eq!(desc1, desc2);
    }

    #[test]
    fn test_task_description_default() {
        let desc = TaskDescription::default();
        assert_eq!(desc.value(), "");
    }
}
