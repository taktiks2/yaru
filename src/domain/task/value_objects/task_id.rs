use anyhow::Result;

/// タスクIDを表すValue Object
///
/// タスクIDは正の整数として扱われます。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(i32);

impl TaskId {
    /// 新しいTaskIdを作成（スタブ - テスト失敗のため）
    pub fn new(_value: i32) -> Result<Self> {
        unimplemented!("TaskId::new() は未実装です")
    }

    /// IDの値を取得（スタブ - テスト失敗のため）
    pub fn value(&self) -> i32 {
        unimplemented!("TaskId::value() は未実装です")
    }
}

// テストのみを先に作成（TDD）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_id_valid() {
        let id = TaskId::new(1).unwrap();
        assert_eq!(id.value(), 1);
    }

    #[test]
    fn test_task_id_zero() {
        let id = TaskId::new(0).unwrap();
        assert_eq!(id.value(), 0);
    }

    #[test]
    fn test_task_id_negative() {
        let result = TaskId::new(-1);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("タスクIDは0以上"));
    }

    #[test]
    fn test_task_id_equality() {
        let id1 = TaskId::new(5).unwrap();
        let id2 = TaskId::new(5).unwrap();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_task_id_copy() {
        let id1 = TaskId::new(10).unwrap();
        let id2 = id1; // Copy trait
        assert_eq!(id1, id2);
    }
}
