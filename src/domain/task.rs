use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

/// タスクを表す構造体
///
/// # フィールド
/// - `id`: タスクの一意な識別子
/// - `title`: タスクのタイトル
/// - `status`: タスクのステータス
/// - `created_at`: タスクの作成日時
/// - `updated_at`: タスクの更新日時
/// - `tags`: タスクに紐づくタグのIDリスト
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    #[serde(default)]
    pub tags: Vec<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// 新しいTaskを作成
    ///
    /// # 引数
    /// - `id`: タスクのID
    /// - `title`: タスクのタイトル
    /// - `description`: タスクの説明
    /// - `status`: タスクの状態
    /// - `priority`: タスクの優先度
    /// - `tags`: タスクに紐づくタグのIDリスト
    ///
    /// # 戻り値
    /// 現在時刻（UTC）を`created_at`と`updated_at`に設定した新しいTaskインスタンス
    pub fn new(
        id: i32,
        title: &str,
        description: &str,
        status: Status,
        priority: Priority,
        tags: Vec<i32>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            status,
            priority,
            tags,
            created_at: now,
            updated_at: now,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "低"),
            Priority::Medium => write!(f, "中"),
            Priority::High => write!(f, "高"),
            Priority::Critical => write!(f, "重大"),
        }
    }
}
