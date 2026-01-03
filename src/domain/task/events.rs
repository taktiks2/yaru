use chrono::{DateTime, Utc};

use super::value_objects::{TaskId, TaskTitle};
use crate::domain::tag::value_objects::TagId;

/// DomainEvent trait - ドメインイベントの基底トレイト
#[allow(dead_code)]
pub trait DomainEvent: Send + Sync {
    /// イベントが発生した日時を取得
    fn occurred_at(&self) -> DateTime<Utc>;

    /// イベントの名前を取得
    fn event_name(&self) -> &str;
}

/// TaskCreated - タスク作成イベント
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct TaskCreated {
    pub task_id: TaskId,
    pub title: TaskTitle,
    pub occurred_at: DateTime<Utc>,
}

impl TaskCreated {
    #[allow(dead_code)]
    pub fn new(task_id: TaskId, title: TaskTitle) -> Self {
        Self {
            task_id,
            title,
            occurred_at: Utc::now(),
        }
    }
}

impl DomainEvent for TaskCreated {
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_name(&self) -> &str {
        "TaskCreated"
    }
}

/// TaskCompleted - タスク完了イベント
#[derive(Debug, Clone, PartialEq)]
pub struct TaskCompleted {
    pub task_id: TaskId,
    pub completed_at: DateTime<Utc>,
    pub occurred_at: DateTime<Utc>,
}

impl TaskCompleted {
    pub fn new(task_id: TaskId, completed_at: DateTime<Utc>) -> Self {
        Self {
            task_id,
            completed_at,
            occurred_at: Utc::now(),
        }
    }
}

impl DomainEvent for TaskCompleted {
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_name(&self) -> &str {
        "TaskCompleted"
    }
}

/// TaskTitleChanged - タスクタイトル変更イベント
#[derive(Debug, Clone, PartialEq)]
pub struct TaskTitleChanged {
    pub task_id: TaskId,
    pub old_title: TaskTitle,
    pub new_title: TaskTitle,
    pub occurred_at: DateTime<Utc>,
}

impl TaskTitleChanged {
    pub fn new(task_id: TaskId, old_title: TaskTitle, new_title: TaskTitle) -> Self {
        Self {
            task_id,
            old_title,
            new_title,
            occurred_at: Utc::now(),
        }
    }
}

impl DomainEvent for TaskTitleChanged {
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_name(&self) -> &str {
        "TaskTitleChanged"
    }
}

/// TaskTagAdded - タスクタグ追加イベント
#[derive(Debug, Clone, PartialEq)]
pub struct TaskTagAdded {
    pub task_id: TaskId,
    pub tag_id: TagId,
    pub occurred_at: DateTime<Utc>,
}

impl TaskTagAdded {
    pub fn new(task_id: TaskId, tag_id: TagId) -> Self {
        Self {
            task_id,
            tag_id,
            occurred_at: Utc::now(),
        }
    }
}

impl DomainEvent for TaskTagAdded {
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_name(&self) -> &str {
        "TaskTagAdded"
    }
}

/// TaskTagRemoved - タスクタグ削除イベント
#[derive(Debug, Clone, PartialEq)]
pub struct TaskTagRemoved {
    pub task_id: TaskId,
    pub tag_id: TagId,
    pub occurred_at: DateTime<Utc>,
}

impl TaskTagRemoved {
    pub fn new(task_id: TaskId, tag_id: TagId) -> Self {
        Self {
            task_id,
            tag_id,
            occurred_at: Utc::now(),
        }
    }
}

impl DomainEvent for TaskTagRemoved {
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn event_name(&self) -> &str {
        "TaskTagRemoved"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_created_event() {
        // Arrange
        let task_id = TaskId::new(1).unwrap();
        let title = TaskTitle::new("新しいタスク").unwrap();

        // Act
        let event = TaskCreated::new(task_id, title.clone());

        // Assert
        assert_eq!(event.task_id, task_id);
        assert_eq!(event.title, title);
        assert_eq!(event.event_name(), "TaskCreated");
    }

    #[test]
    fn test_task_completed_event() {
        // Arrange
        let task_id = TaskId::new(1).unwrap();
        let completed_at = Utc::now();

        // Act
        let event = TaskCompleted::new(task_id, completed_at);

        // Assert
        assert_eq!(event.task_id, task_id);
        assert_eq!(event.completed_at, completed_at);
        assert_eq!(event.event_name(), "TaskCompleted");
    }

    #[test]
    fn test_task_title_changed_event() {
        // Arrange
        let task_id = TaskId::new(1).unwrap();
        let old_title = TaskTitle::new("古いタイトル").unwrap();
        let new_title = TaskTitle::new("新しいタイトル").unwrap();

        // Act
        let event = TaskTitleChanged::new(task_id, old_title.clone(), new_title.clone());

        // Assert
        assert_eq!(event.task_id, task_id);
        assert_eq!(event.old_title, old_title);
        assert_eq!(event.new_title, new_title);
        assert_eq!(event.event_name(), "TaskTitleChanged");
    }

    #[test]
    fn test_task_tag_added_event() {
        // Arrange
        let task_id = TaskId::new(1).unwrap();
        let tag_id = TagId::new(1).unwrap();

        // Act
        let event = TaskTagAdded::new(task_id, tag_id);

        // Assert
        assert_eq!(event.task_id, task_id);
        assert_eq!(event.tag_id, tag_id);
        assert_eq!(event.event_name(), "TaskTagAdded");
    }

    #[test]
    fn test_task_tag_removed_event() {
        // Arrange
        let task_id = TaskId::new(1).unwrap();
        let tag_id = TagId::new(1).unwrap();

        // Act
        let event = TaskTagRemoved::new(task_id, tag_id);

        // Assert
        assert_eq!(event.task_id, task_id);
        assert_eq!(event.tag_id, tag_id);
        assert_eq!(event.event_name(), "TaskTagRemoved");
    }
}
