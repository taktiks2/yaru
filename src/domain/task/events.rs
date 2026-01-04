//! # ドメインイベントモジュール
//!
//! タスクに関連するドメインイベントを定義します。
//!
//! ## DomainEventの使い方
//!
//! ### 1. イベントの作成
//!
//! ```rust,ignore
//! use crate::domain::task::events::TaskCreated;
//! use crate::domain::task::value_objects::{TaskId, TaskTitle};
//!
//! // イベントインスタンスの作成
//! let task_id = TaskId::new(1).unwrap();
//! let title = TaskTitle::new("新しいタスク").unwrap();
//! let event = TaskCreated::new(task_id, title);
//! ```
//!
//! ### 2. 集約ルートでのイベント発火
//!
//! ```rust,ignore
//! // TaskAggregateでイベントを発火する例
//! impl TaskAggregate {
//!     pub fn create(id: TaskId, title: TaskTitle) -> Self {
//!         let mut task = Self::new(id, title.clone(), /* ... */);
//!
//!         // イベントを作成してドメインイベントリストに追加
//!         let event = TaskCreated::new(id, title);
//!         task.add_domain_event(Box::new(event));
//!
//!         task
//!     }
//!
//!     fn add_domain_event(&mut self, event: Box<dyn DomainEvent>) {
//!         self.domain_events.push(event);
//!     }
//! }
//! ```
//!
//! ### 3. イベントの取得と処理
//!
//! ```rust,ignore
//! // リポジトリやユースケースでイベントを取得して処理
//! let mut task = task_repository.find_by_id(&task_id).await?;
//!
//! // 何らかの操作を実行
//! task.complete(Utc::now())?;
//!
//! // 発火したイベントを取得
//! let events = task.take_domain_events();
//!
//! // イベントハンドラに渡して処理
//! for event in events {
//!     event_bus.publish(event).await?;
//! }
//! ```
//!
//! ### 4. イベントの型判定とダウンキャスト
//!
//! ```rust,ignore
//! use crate::domain::task::events::{DomainEvent, TaskCompleted};
//!
//! fn handle_event(event: &dyn DomainEvent) {
//!     // as_any()を使って具体的な型にダウンキャスト
//!     if let Some(task_completed) = event.as_any().downcast_ref::<TaskCompleted>() {
//!         println!("タスク {} が完了しました", task_completed.task_id.value());
//!     }
//! }
//! ```
//!
//! ### 5. イベントハンドラの実装例
//!
//! ```rust,ignore
//! use crate::domain::task::events::{DomainEvent, TaskCompleted, TaskCreated};
//! use async_trait::async_trait;
//! use anyhow::Result;
//!
//! // イベントハンドラのトレイト定義
//! #[async_trait]
//! trait EventHandler: Send + Sync {
//!     async fn handle(&self, event: &dyn DomainEvent) -> Result<()>;
//! }
//!
//! // タスク完了時にメールを送信するハンドラ
//! struct TaskCompletedEmailHandler {
//!     email_service: Arc<dyn EmailService>,
//! }
//!
//! #[async_trait]
//! impl EventHandler for TaskCompletedEmailHandler {
//!     async fn handle(&self, event: &dyn DomainEvent) -> Result<()> {
//!         // TaskCompletedイベントのみ処理
//!         if let Some(task_completed) = event.as_any().downcast_ref::<TaskCompleted>() {
//!             // メール送信
//!             self.email_service.send_task_completion_notification(
//!                 task_completed.task_id,
//!                 task_completed.completed_at,
//!             ).await?;
//!
//!             println!("タスク完了通知メールを送信しました: {}", task_completed.task_id.value());
//!         }
//!         Ok(())
//!     }
//! }
//!
//! // 重要なイベントをログに記録するハンドラ
//! struct EventLoggingHandler {
//!     logger: Arc<dyn Logger>,
//! }
//!
//! #[async_trait]
//! impl EventHandler for EventLoggingHandler {
//!     async fn handle(&self, event: &dyn DomainEvent) -> Result<()> {
//!         // すべてのイベントをログに記録
//!         self.logger.info(&format!(
//!             "[DomainEvent] {} - occurred_at: {}",
//!             event.event_name(),
//!             event.occurred_at()
//!         ));
//!
//!         // 特定のイベントは詳細ログを記録
//!         if let Some(task_completed) = event.as_any().downcast_ref::<TaskCompleted>() {
//!             self.logger.info(&format!(
//!                 "[TaskCompleted] task_id: {}, completed_at: {}",
//!                 task_completed.task_id.value(),
//!                 task_completed.completed_at
//!             ));
//!         } else if let Some(task_created) = event.as_any().downcast_ref::<TaskCreated>() {
//!             self.logger.info(&format!(
//!                 "[TaskCreated] task_id: {}, title: {}",
//!                 task_created.task_id.value(),
//!                 task_created.title.value()
//!             ));
//!         }
//!
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ### 6. イベントバスでの複数ハンドラの実行
//!
//! ```rust,ignore
//! use crate::domain::task::events::DomainEvent;
//! use anyhow::Result;
//!
//! // イベントバス - 複数のハンドラを管理
//! struct EventBus {
//!     handlers: Vec<Arc<dyn EventHandler>>,
//! }
//!
//! impl EventBus {
//!     pub fn new() -> Self {
//!         Self {
//!             handlers: Vec::new(),
//!         }
//!     }
//!
//!     pub fn register(&mut self, handler: Arc<dyn EventHandler>) {
//!         self.handlers.push(handler);
//!     }
//!
//!     pub async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<()> {
//!         // 登録されたすべてのハンドラでイベントを処理
//!         for handler in &self.handlers {
//!             handler.handle(event.as_ref()).await?;
//!         }
//!         Ok(())
//!     }
//! }
//!
//! // 使用例
//! async fn example_usage() -> Result<()> {
//!     // イベントバスのセットアップ
//!     let mut event_bus = EventBus::new();
//!
//!     // ハンドラを登録
//!     event_bus.register(Arc::new(EventLoggingHandler::new()));
//!     event_bus.register(Arc::new(TaskCompletedEmailHandler::new()));
//!
//!     // タスク完了処理
//!     let mut task = task_repository.find_by_id(&task_id).await?;
//!     task.complete(Utc::now())?;
//!
//!     // 変更を保存
//!     task_repository.save(&task).await?;
//!
//!     // イベントを発行（ログ記録とメール送信が実行される）
//!     for event in task.take_domain_events() {
//!         event_bus.publish(event).await?;
//!     }
//!
//!     Ok(())
//! }
//! ```

use crate::domain::{
    tag::value_objects::TagId,
    task::value_objects::{TaskId, TaskTitle},
};
use chrono::{DateTime, Utc};
use std::fmt::Debug;

/// DomainEvent trait - ドメインイベントの基底トレイト
#[allow(dead_code)]
pub trait DomainEvent: Debug + Send + Sync {
    /// イベントが発生した日時を取得
    fn occurred_at(&self) -> DateTime<Utc>;

    /// イベントの名前を取得
    fn event_name(&self) -> &str;

    /// Any型へのダウンキャストのためのメソッド
    fn as_any(&self) -> &dyn std::any::Any;
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
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
