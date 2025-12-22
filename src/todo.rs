use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fmt;

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
    pub status: Status,
    pub created_at: String,
    pub updated_at: String,
}

impl Todo {
    /// 新しいTodoを作成
    ///
    /// # 引数
    /// - `id`: タスクのID
    /// - `title`: タスクのタイトル
    ///
    /// # 戻り値
    /// 現在時刻（UTC）を`created_at`と`updated_at`に設定した新しいTodoインスタンス
    pub fn new(id: u64, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            status: Status::Pending,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Pending,
    Completed,
    InProgress,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "保留中"),
            Status::Completed => write!(f, "完了"),
            Status::InProgress => write!(f, "進行中"),
        }
    }
}
