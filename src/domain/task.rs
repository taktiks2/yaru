use crate::domain::tag::Tag;
use chrono::{DateTime, NaiveDate, Utc};
use clap::ValueEnum;
use entity::{tags, tasks};
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
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,
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
    /// - `due_date`: タスクの期限（オプション）
    ///
    /// # 戻り値
    /// 現在時刻（UTC）を`created_at`と`updated_at`に設定した新しいTaskインスタンス
    /// `completed_at`は常にNoneで初期化される
    pub fn new(
        id: i32,
        title: &str,
        description: &str,
        status: Status,
        priority: Priority,
        tags: Vec<Tag>,
        due_date: Option<NaiveDate>,
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
            due_date,
            completed_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, ValueEnum)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
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
            "inprogress" | "in_progress" | "progress" => Ok(Status::InProgress),
            "completed" | "done" => Ok(Status::Completed),
            _ => Err(format!("Invalid status value: '{}'", value)),
        }
    }
}

/// データベースの値からStatusへの変換
impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Pending" => Ok(Status::Pending),
            "InProgress" => Ok(Status::InProgress),
            "Completed" => Ok(Status::Completed),
            _ => Err(format!("Invalid status in database: '{}'", value)),
        }
    }
}

/// データベース用の文字列表現への変換
impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        match self {
            Status::Pending => "Pending",
            Status::InProgress => "InProgress",
            Status::Completed => "Completed",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "保留中"),
            Status::InProgress => write!(f, "進行中"),
            Status::Completed => write!(f, "完了"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// データベースの値からPriorityへの変換
impl TryFrom<&str> for Priority {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Low" => Ok(Priority::Low),
            "Medium" => Ok(Priority::Medium),
            "High" => Ok(Priority::High),
            "Critical" => Ok(Priority::Critical),
            _ => Err(format!("Invalid priority in database: '{}'", value)),
        }
    }
}

/// データベース用の文字列表現への変換
impl AsRef<str> for Priority {
    fn as_ref(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Critical => "Critical",
        }
    }
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

/// tasks::ModelとVec<tags::Model>からTaskへの変換
impl TryFrom<(tasks::Model, Vec<tags::Model>)> for Task {
    type Error = String;

    fn try_from(
        (model, tag_models): (tasks::Model, Vec<tags::Model>),
    ) -> Result<Self, Self::Error> {
        let status = Status::try_from(model.status.as_str())?;
        let priority = Priority::try_from(model.priority.as_str())?;
        let tags: Vec<Tag> = tag_models.into_iter().map(|t| t.into()).collect();

        Ok(Task {
            id: model.id,
            title: model.title,
            description: model.description,
            status,
            priority,
            tags,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
            due_date: model.due_date,
            // completed_at は Option<DateTimeWithTimeZone> のため、
            // Option の中身 (DateTimeWithTimeZone) だけを DateTime<Utc> に変換する必要がある。
            // Option<T> -> Option<U> は自動変換されないため、map を使って
            // Some の場合のみ dt.into() を適用し、None はそのまま None にする。
            completed_at: model.completed_at.map(|dt| dt.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tag::Tag;
    use chrono::Utc;
    use entity::{tags, tasks};
    use sea_orm::prelude::DateTimeWithTimeZone;

    #[test]
    fn test_task_new_with_tags() {
        let tag1 = Tag::new(1, "重要", "");
        let tag2 = Tag::new(2, "緊急", "");
        let tags = vec![tag1.clone(), tag2.clone()];

        let task = Task::new(
            1,
            "テストタスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            tags.clone(),
            None,
        );

        assert_eq!(task.tags.len(), 2);
        assert_eq!(task.tags[0].id, 1);
        assert_eq!(task.tags[1].id, 2);
    }

    #[test]
    fn test_task_serialization_with_tags() {
        let tag = Tag::new(1, "テスト", "説明");
        let task = Task::new(
            1,
            "タスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![tag],
            None,
        );

        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tags.len(), 1);
        assert_eq!(deserialized.tags[0].name, "テスト");
    }

    #[test]
    fn test_try_from_tasks_model_without_tags() {
        let now: DateTimeWithTimeZone = Utc::now().into();
        let task_model = tasks::Model {
            id: 1,
            title: "テストタスク".to_string(),
            description: "説明".to_string(),
            status: "Pending".to_string(),
            priority: "Medium".to_string(),
            created_at: now,
            updated_at: now,
            due_date: None,
            completed_at: None,
        };

        let tag_models: Vec<tags::Model> = vec![];

        let result = Task::try_from((task_model, tag_models));
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "テストタスク");
        assert_eq!(task.description, "説明");
        assert_eq!(task.status, Status::Pending);
        assert_eq!(task.priority, Priority::Medium);
        assert_eq!(task.tags.len(), 0);
    }

    #[test]
    fn test_try_from_tasks_model_with_tags() {
        let now: DateTimeWithTimeZone = Utc::now().into();
        let task_model = tasks::Model {
            id: 1,
            title: "タグ付きタスク".to_string(),
            description: "説明".to_string(),
            status: "InProgress".to_string(),
            priority: "High".to_string(),
            created_at: now,
            updated_at: now,
            due_date: None,
            completed_at: None,
        };

        let tag_models = vec![
            tags::Model {
                id: 1,
                name: "重要".to_string(),
                description: "重要なタスク".to_string(),
                created_at: now,
                updated_at: now,
            },
            tags::Model {
                id: 2,
                name: "緊急".to_string(),
                description: "緊急タスク".to_string(),
                created_at: now,
                updated_at: now,
            },
        ];

        let result = Task::try_from((task_model, tag_models));
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "タグ付きタスク");
        assert_eq!(task.status, Status::InProgress);
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.tags.len(), 2);
        assert_eq!(task.tags[0].id, 1);
        assert_eq!(task.tags[0].name, "重要");
        assert_eq!(task.tags[1].id, 2);
        assert_eq!(task.tags[1].name, "緊急");
    }

    #[test]
    fn test_try_from_invalid_status() {
        let now: DateTimeWithTimeZone = Utc::now().into();
        let task_model = tasks::Model {
            id: 1,
            title: "テストタスク".to_string(),
            description: "説明".to_string(),
            status: "InvalidStatus".to_string(),
            priority: "Medium".to_string(),
            created_at: now,
            updated_at: now,
            due_date: None,
            completed_at: None,
        };

        let tag_models: Vec<tags::Model> = vec![];

        let result = Task::try_from((task_model, tag_models));
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_invalid_priority() {
        let now: DateTimeWithTimeZone = Utc::now().into();
        let task_model = tasks::Model {
            id: 1,
            title: "テストタスク".to_string(),
            description: "説明".to_string(),
            status: "Pending".to_string(),
            priority: "InvalidPriority".to_string(),
            created_at: now,
            updated_at: now,
            due_date: None,
            completed_at: None,
        };

        let tag_models: Vec<tags::Model> = vec![];

        let result = Task::try_from((task_model, tag_models));
        assert!(result.is_err());
    }

    #[test]
    fn test_task_new_with_due_date() {
        let due_date = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        let task = Task::new(
            1,
            "期限付きタスク",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(due_date),
        );

        assert_eq!(task.due_date, Some(due_date));
        assert_eq!(task.completed_at, None);
    }

    #[test]
    fn test_task_new_without_due_date() {
        let task = Task::new(
            1,
            "期限なしタスク",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );

        assert_eq!(task.due_date, None);
        assert_eq!(task.completed_at, None);
    }

    #[test]
    fn test_try_from_tasks_model_with_due_date_and_completed_at() {
        use chrono::NaiveDate;

        let now: DateTimeWithTimeZone = Utc::now().into();
        let due_date = NaiveDate::from_ymd_opt(2026, 12, 31);

        let task_model = tasks::Model {
            id: 1,
            title: "完了タスク".to_string(),
            description: "".to_string(),
            status: "Completed".to_string(),
            priority: "Medium".to_string(),
            created_at: now,
            updated_at: now,
            due_date,
            completed_at: Some(now),
        };

        let tag_models: Vec<tags::Model> = vec![];

        let result = Task::try_from((task_model, tag_models));
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.due_date, due_date);
        assert!(task.completed_at.is_some());
    }
}
