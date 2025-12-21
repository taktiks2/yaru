use chrono::Local;
use serde::{Deserialize, Serialize};

/// Todoタスクを表す構造体
///
/// # フィールド
/// - `id`: タスクの一意な識別子
/// - `title`: タスクのタイトル
/// - `created_at`: タスクの作成日時（RFC3339形式の文字列）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub created_at: String,
}

impl Todo {
    /// 新しいTodoを作成
    ///
    /// # 引数
    /// - `id`: タスクのID
    /// - `title`: タスクのタイトル
    ///
    /// # 戻り値
    /// 現在時刻を`created_at`に設定した新しいTodoインスタンス
    pub fn new(id: u64, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            created_at: Local::now().to_rfc3339(),
        }
    }
}
