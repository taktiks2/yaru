use anyhow::Result;

/// タスクのタイトルを表すValue Object
///
/// タイトルは1文字以上100文字以下の文字列である必要があります。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTitle(String);

impl TaskTitle {
    /// 新しいTaskTitleを作成
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            anyhow::bail!("タイトルは空にできません");
        }

        if value.len() > 100 {
            anyhow::bail!("タイトルは100文字以内にしてください");
        }

        Ok(Self(value))
    }

    /// タイトルの値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

// テストのみを先に作成（TDD）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_title_valid() {
        let title = TaskTitle::new("有効なタイトル").unwrap();
        assert_eq!(title.value(), "有効なタイトル");
    }

    #[test]
    fn test_task_title_single_char() {
        let title = TaskTitle::new("a").unwrap();
        assert_eq!(title.value(), "a");
    }

    #[test]
    fn test_task_title_max_length() {
        let long_title = "a".repeat(100);
        let title = TaskTitle::new(&long_title).unwrap();
        assert_eq!(title.value(), long_title);
    }

    #[test]
    fn test_task_title_empty() {
        let result = TaskTitle::new("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("空"));
    }

    #[test]
    fn test_task_title_whitespace_only() {
        let result = TaskTitle::new("   ");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("空"));
    }

    #[test]
    fn test_task_title_too_long() {
        let too_long = "a".repeat(101);
        let result = TaskTitle::new(&too_long);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("100文字"));
    }

    #[test]
    fn test_task_title_trim() {
        // 先頭・末尾の空白は保持される
        let title = TaskTitle::new("  タイトル  ").unwrap();
        assert_eq!(title.value(), "  タイトル  ");
    }

    #[test]
    fn test_task_title_equality() {
        let title1 = TaskTitle::new("同じタイトル").unwrap();
        let title2 = TaskTitle::new("同じタイトル").unwrap();
        assert_eq!(title1, title2);
    }

    #[test]
    fn test_task_title_clone() {
        let title1 = TaskTitle::new("タイトル").unwrap();
        let title2 = title1.clone();
        assert_eq!(title1, title2);
    }
}
