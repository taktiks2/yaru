use chrono::Utc;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Todoタスクを表す構造体
///
/// # フィールド
/// - `id`: タスクの一意な識別子
/// - `title`: タスクのタイトル
/// - `status`: タスクのステータス
/// - `created_at`: タスクの作成日時
/// - `updated_at`: タスクの更新日時
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
    pub fn new(id: u64, title: &str, status: Status) -> Self {
        Self {
            id,
            title: title.to_string(),
            status,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ValueEnum)]
pub enum Status {
    Pending,
    Completed,
    InProgress,
}

impl Status {
    /// フィルタ値から Status を生成
    ///
    /// # 引数
    /// - `value`: フィルタ値（例: "done", "pending", "in_progress"）
    ///
    /// # 戻り値
    /// - `Ok(Status)`: 変換に成功した場合
    /// - `Err(String)`: 無効な値の場合
    pub fn from_filter_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_str() {
            "pending" | "todo" => Ok(Status::Pending),
            "completed" | "done" => Ok(Status::Completed),
            "inprogress" | "in_progress" | "progress" => Ok(Status::InProgress),
            _ => Err(format!("Invalid status value: '{}'", value)),
        }
    }
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
