use crate::domain::task::{
    aggregate::TaskAggregate,
    value_objects::{Priority, Status},
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// タグ参照情報を表すDTO
///
/// タスク表示時に必要な最小限のタグ情報（IDと名前のみ）を保持します。
/// TagDTOとは異なり、description, created_at, updated_atは含みません。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagInfo {
    pub id: i32,
    pub name: String,
}

/// タスクの読み取り専用表現（DTO）
///
/// Use CaseからPresentation層への出力に使用されます。
/// Value Objectではなくプリミティブ型を使用し、シリアライズ可能にします。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskDTO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub tags: Vec<TagInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// タスク作成時の入力DTO
///
/// CLI/TUI → Use Caseへの入力に使用されます。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateTaskDTO {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub tags: Vec<i32>,
    pub due_date: Option<NaiveDate>,
}

/// タスク更新時の入力DTO
///
/// すべてのフィールドがオプションで、部分更新をサポートします。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateTaskDTO {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub tags: Option<Vec<i32>>,
    pub due_date: Option<NaiveDate>,
}

// TaskAggregateからTaskDTOへの変換
//
// 注意: tagsフィールドは空のVecとして初期化されます。
// タグ情報の解決はユースケース層で行います（TagRepositoryを使用）。
impl From<TaskAggregate> for TaskDTO {
    fn from(task: TaskAggregate) -> Self {
        Self {
            id: task.id().value(),
            title: task.title().value().to_string(),
            description: if task.description().value().is_empty() {
                None
            } else {
                Some(task.description().value().to_string())
            },
            status: status_to_string(task.status()),
            priority: priority_to_string(task.priority()),
            tags: Vec::new(), // タグ情報はユースケース層で設定
            created_at: *task.created_at(),
            updated_at: *task.updated_at(),
            due_date: task.due_date().as_ref().map(|dd| dd.value()),
            completed_at: *task.completed_at(),
        }
    }
}

// Statusを文字列に変換
fn status_to_string(status: &Status) -> String {
    match status {
        Status::Pending => "pending".to_string(),
        Status::InProgress => "in_progress".to_string(),
        Status::Completed => "completed".to_string(),
    }
}

// Priorityを文字列に変換
fn priority_to_string(priority: &Priority) -> String {
    match priority {
        Priority::Low => "low".to_string(),
        Priority::Medium => "medium".to_string(),
        Priority::High => "high".to_string(),
        Priority::Critical => "critical".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::{
        aggregate::TaskAggregate,
        value_objects::{Priority, Status, TaskDescription, TaskTitle},
    };

    #[test]
    fn test_task_dto_from_aggregate() {
        // TaskAggregateを作成
        let task = TaskAggregate::new(
            TaskTitle::new("テストタスク").unwrap(),
            TaskDescription::new("テスト説明").unwrap(),
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );

        // TaskDTOに変換
        let dto = TaskDTO::from(task.clone());

        // 検証
        assert_eq!(dto.id, 0); // デフォルトID
        assert_eq!(dto.title, "テストタスク");
        assert_eq!(dto.description, Some("テスト説明".to_string()));
        assert_eq!(dto.status, "pending");
        assert_eq!(dto.priority, "high");
        assert_eq!(dto.tags, Vec::<TagInfo>::new()); // From実装では空
        assert_eq!(dto.due_date, None);
        assert_eq!(dto.completed_at, None);
        // created_at, updated_atは現在時刻なので、存在することだけを確認
    }

    #[test]
    fn test_task_dto_from_aggregate_with_tags() {
        use crate::domain::tag::value_objects::TagId;

        let task = TaskAggregate::new(
            TaskTitle::new("タグ付きタスク").unwrap(),
            TaskDescription::new("説明").unwrap(),
            Status::InProgress,
            Priority::Medium,
            vec![TagId::new(1).unwrap(), TagId::new(2).unwrap()],
            None,
        );

        let dto = TaskDTO::from(task);

        // From実装ではタグ情報は空（ユースケース層で設定される）
        assert_eq!(dto.tags, Vec::<TagInfo>::new());
        assert_eq!(dto.status, "in_progress");
        assert_eq!(dto.priority, "medium");
    }

    #[test]
    fn test_task_dto_from_completed_task() {
        let mut task = TaskAggregate::new(
            TaskTitle::new("完了タスク").unwrap(),
            TaskDescription::new("説明").unwrap(),
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );

        // タスクを完了
        task.complete().unwrap();

        let dto = TaskDTO::from(task);

        assert_eq!(dto.status, "completed");
        assert!(dto.completed_at.is_some());
    }

    #[test]
    fn test_create_task_dto_minimal() {
        let dto = CreateTaskDTO {
            title: "新しいタスク".to_string(),
            description: None,
            status: None,
            priority: None,
            tags: vec![],
            due_date: None,
        };

        assert_eq!(dto.title, "新しいタスク");
        assert_eq!(dto.description, None);
    }

    #[test]
    fn test_create_task_dto_full() {
        use chrono::NaiveDate;

        let dto = CreateTaskDTO {
            title: "詳細タスク".to_string(),
            description: Some("詳細説明".to_string()),
            status: Some("pending".to_string()),
            priority: Some("high".to_string()),
            tags: vec![1, 2, 3],
            due_date: Some(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()),
        };

        assert_eq!(dto.title, "詳細タスク");
        assert_eq!(dto.description, Some("詳細説明".to_string()));
        assert_eq!(dto.status, Some("pending".to_string()));
        assert_eq!(dto.priority, Some("high".to_string()));
        assert_eq!(dto.tags, vec![1, 2, 3]);
        assert!(dto.due_date.is_some());
    }

    #[test]
    fn test_update_task_dto_default() {
        let dto = UpdateTaskDTO::default();

        assert_eq!(dto.title, None);
        assert_eq!(dto.description, None);
        assert_eq!(dto.status, None);
        assert_eq!(dto.priority, None);
        assert_eq!(dto.tags, None);
        assert_eq!(dto.due_date, None);
    }

    #[test]
    fn test_update_task_dto_partial() {
        let dto = UpdateTaskDTO {
            title: Some("更新タイトル".to_string()),
            status: Some("completed".to_string()),
            ..Default::default()
        };

        assert_eq!(dto.title, Some("更新タイトル".to_string()));
        assert_eq!(dto.status, Some("completed".to_string()));
        assert_eq!(dto.description, None);
        assert_eq!(dto.priority, None);
    }
}
