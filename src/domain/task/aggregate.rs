use crate::domain::{
    tag::value_objects::TagId,
    task::{
        events::{TaskCompleted, TaskTagAdded, TaskTagRemoved, TaskTitleChanged},
        value_objects::{DueDate, Priority, Status, TaskDescription, TaskId, TaskTitle},
    },
};
use anyhow::{Result, bail};
use chrono::{DateTime, Utc};
use std::any::Any;

/// TaskAggregate の再構築用パラメータ
///
/// リポジトリからTaskAggregateを再構築する際に使用するパラメータをまとめた構造体です。
#[derive(Debug)]
pub struct TaskReconstructParams {
    pub id: TaskId,
    pub title: TaskTitle,
    pub description: TaskDescription,
    pub status: Status,
    pub priority: Priority,
    pub tags: Vec<TagId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DueDate>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// TaskAggregate - タスクのAggregate Root
///
/// タスクのビジネスルールを実装し、不変条件を保護します。
#[derive(Debug)]
pub struct TaskAggregate {
    id: TaskId,
    title: TaskTitle,
    description: TaskDescription,
    status: Status,
    priority: Priority,
    tags: Vec<TagId>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    due_date: Option<DueDate>,
    completed_at: Option<DateTime<Utc>>,
    // Domain Events
    domain_events: Vec<Box<dyn Any + Send + Sync>>,
}

impl Clone for TaskAggregate {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            status: self.status,
            priority: self.priority,
            tags: self.tags.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            due_date: self.due_date,
            completed_at: self.completed_at,
            // domain_eventsはクローン時には空にする
            domain_events: Vec::new(),
        }
    }
}

impl PartialEq for TaskAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.title == other.title
            && self.description == other.description
            && self.status == other.status
            && self.priority == other.priority
            && self.tags == other.tags
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at
            && self.due_date == other.due_date
            && self.completed_at == other.completed_at
        // domain_eventsは比較しない
    }
}

impl TaskAggregate {
    /// 新しいタスクを作成します（ファクトリメソッド）
    pub fn new(
        title: TaskTitle,
        description: TaskDescription,
        status: Status,
        priority: Priority,
        tags: Vec<TagId>,
        due_date: Option<DueDate>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: TaskId::new(0).unwrap(), // デフォルトは0、リポジトリで新しいIDを割り当てる
            title,
            description,
            status,
            priority,
            tags,
            created_at: now,
            updated_at: now,
            due_date,
            completed_at: None,
            domain_events: Vec::new(),
        }
    }

    /// リポジトリからの再構築用ファクトリメソッド
    ///
    /// データベースから読み込んだデータをTaskAggregateに変換する際に使用します。
    /// ドメインイベントは空の状態で作成されます。
    pub fn reconstruct(params: TaskReconstructParams) -> Self {
        Self {
            id: params.id,
            title: params.title,
            description: params.description,
            status: params.status,
            priority: params.priority,
            tags: params.tags,
            created_at: params.created_at,
            updated_at: params.updated_at,
            due_date: params.due_date,
            completed_at: params.completed_at,
            domain_events: Vec::new(),
        }
    }

    /// IDを設定した新しいインスタンスを返す
    ///
    /// リポジトリがタスクを保存する際に新しいIDを割り当てるために使用します。
    /// 元のインスタンスは変更せず、新しいインスタンスを返します。
    #[allow(dead_code)]
    pub fn with_id(self, id: TaskId) -> Self {
        Self {
            id,
            title: self.title,
            description: self.description,
            status: self.status,
            priority: self.priority,
            tags: self.tags,
            created_at: self.created_at,
            updated_at: self.updated_at,
            due_date: self.due_date,
            completed_at: self.completed_at,
            domain_events: self.domain_events,
        }
    }

    /// タスクを完了します
    ///
    /// ステータスをCompletedに変更し、completed_atを現在時刻に設定します。
    /// 既に完了している場合は、何もしません。
    #[allow(dead_code)]
    pub fn complete(&mut self) -> Result<()> {
        if self.status != Status::Completed {
            self.status = Status::Completed;
            let now = Utc::now();
            self.completed_at = Some(now);
            self.updated_at = now;

            // Domain Event発行
            let event = TaskCompleted::new(self.id, now);
            self.domain_events.push(Box::new(event));
        }
        Ok(())
    }

    /// タスクが期限切れかどうかを判定します
    ///
    /// 以下の条件で期限切れと判定されます：
    /// - 期限が設定されている
    /// - 現在の日付が期限を過ぎている
    /// - タスクが完了していない
    #[allow(dead_code)]
    pub fn is_overdue(&self) -> bool {
        if self.status == Status::Completed {
            return false;
        }

        if let Some(due_date) = &self.due_date {
            let today = Utc::now().naive_utc().date();
            return due_date.value() < today;
        }

        false
    }

    /// タスクのタイトルを変更します
    pub fn change_title(&mut self, new_title: TaskTitle) -> Result<()> {
        let old_title = self.title.clone();
        self.title = new_title.clone();
        self.updated_at = Utc::now();

        // Domain Event発行
        let event = TaskTitleChanged::new(self.id, old_title, new_title);
        self.domain_events.push(Box::new(event));

        Ok(())
    }

    /// タスクの説明を変更します
    pub fn change_description(&mut self, new_description: TaskDescription) -> Result<()> {
        self.description = new_description;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// タスクのステータスを変更します
    ///
    /// Completedへの変更の場合はcomplete()メソッドを使用してください。
    pub fn change_status(&mut self, new_status: Status) -> Result<()> {
        self.status = new_status;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// タスクの優先度を変更します
    pub fn change_priority(&mut self, new_priority: Priority) -> Result<()> {
        self.priority = new_priority;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// タスクの期限日を変更します
    pub fn change_due_date(&mut self, new_due_date: Option<DueDate>) -> Result<()> {
        self.due_date = new_due_date;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// タスクのタグを置き換えます
    ///
    /// 既存のタグをすべて削除して、新しいタグリストで置き換えます。
    pub fn replace_tags(&mut self, new_tags: Vec<TagId>) -> Result<()> {
        self.tags = new_tags;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// タスクにタグを追加します
    ///
    /// 既に同じタグが存在する場合はエラーを返します。
    #[allow(dead_code)]
    pub fn add_tag(&mut self, tag_id: TagId) -> Result<()> {
        if self.tags.contains(&tag_id) {
            bail!("タグID {} は既に追加されています", tag_id.value());
        }
        self.tags.push(tag_id);
        self.updated_at = Utc::now();

        // Domain Event発行
        let event = TaskTagAdded::new(self.id, tag_id);
        self.domain_events.push(Box::new(event));

        Ok(())
    }

    /// タスクからタグを削除します
    ///
    /// 指定したタグが存在しない場合はエラーを返します。
    #[allow(dead_code)]
    pub fn remove_tag(&mut self, tag_id: &TagId) -> Result<()> {
        let original_len = self.tags.len();
        self.tags.retain(|t| t != tag_id);

        if self.tags.len() == original_len {
            bail!("タグID {} は存在しません", tag_id.value());
        }

        self.updated_at = Utc::now();

        // Domain Event発行
        let event = TaskTagRemoved::new(self.id, *tag_id);
        self.domain_events.push(Box::new(event));

        Ok(())
    }

    // Getters
    pub fn id(&self) -> &TaskId {
        &self.id
    }

    pub fn title(&self) -> &TaskTitle {
        &self.title
    }

    pub fn description(&self) -> &TaskDescription {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn priority(&self) -> &Priority {
        &self.priority
    }

    pub fn tags(&self) -> &Vec<TagId> {
        &self.tags
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn due_date(&self) -> &Option<DueDate> {
        &self.due_date
    }

    pub fn completed_at(&self) -> &Option<DateTime<Utc>> {
        &self.completed_at
    }

    /// ドメインイベントを取得します
    #[allow(dead_code)]
    pub fn domain_events(&self) -> &Vec<Box<dyn Any + Send + Sync>> {
        &self.domain_events
    }

    /// ドメインイベントをクリアします
    ///
    /// イベントハンドラで処理した後に呼び出されることを想定しています。
    #[allow(dead_code)]
    pub fn clear_events(&mut self) {
        self.domain_events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_new_task() {
        // Arrange
        let title = TaskTitle::new("新しいタスク").unwrap();
        let description = TaskDescription::new("説明文").unwrap();
        let status = Status::Pending;
        let priority = Priority::Medium;
        let tags = vec![];
        let due_date = None;

        // Act
        let task = TaskAggregate::new(
            title.clone(),
            description.clone(),
            status,
            priority,
            tags.clone(),
            due_date,
        );

        // Assert
        assert_eq!(task.title(), &title);
        assert_eq!(task.description(), &description);
        assert_eq!(task.status(), &status);
        assert_eq!(task.priority(), &priority);
        assert_eq!(task.tags(), &tags);
        assert_eq!(task.due_date(), &due_date);
        assert_eq!(task.completed_at(), &None);
    }

    #[test]
    fn test_complete_task() {
        // Arrange
        let title = TaskTitle::new("完了するタスク").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::InProgress,
            Priority::High,
            vec![],
            None,
        );

        // Act
        let result = task.complete();

        // Assert
        assert!(result.is_ok());
        assert_eq!(task.status(), &Status::Completed);
        assert!(task.completed_at().is_some());
    }

    #[test]
    fn test_complete_already_completed_task() {
        // Arrange
        let title = TaskTitle::new("既に完了したタスク").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Low,
            vec![],
            None,
        );
        task.complete().unwrap();
        let first_completed_at = *task.completed_at();

        // Act
        let result = task.complete();

        // Assert
        assert!(result.is_ok());
        assert_eq!(task.status(), &Status::Completed);
        assert_eq!(task.completed_at(), &first_completed_at);
    }

    #[test]
    fn test_is_overdue_with_no_due_date() {
        // Arrange
        let title = TaskTitle::new("期限なしタスク").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );

        // Act & Assert
        assert!(!task.is_overdue());
    }

    #[test]
    fn test_is_overdue_with_past_due_date() {
        // Arrange
        let title = TaskTitle::new("期限切れタスク").unwrap();
        let description = TaskDescription::new("").unwrap();
        let past_date = Utc::now().naive_utc().date() - Duration::days(1);
        let due_date = Some(DueDate::new(past_date).unwrap());
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            due_date,
        );

        // Act & Assert
        assert!(task.is_overdue());
    }

    #[test]
    fn test_is_overdue_with_future_due_date() {
        // Arrange
        let title = TaskTitle::new("未来の期限").unwrap();
        let description = TaskDescription::new("").unwrap();
        let future_date = Utc::now().naive_utc().date() + Duration::days(1);
        let due_date = Some(DueDate::new(future_date).unwrap());
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            due_date,
        );

        // Act & Assert
        assert!(!task.is_overdue());
    }

    #[test]
    fn test_is_overdue_completed_task() {
        // Arrange
        let title = TaskTitle::new("完了した期限切れタスク").unwrap();
        let description = TaskDescription::new("").unwrap();
        let past_date = Utc::now().naive_utc().date() - Duration::days(1);
        let due_date = Some(DueDate::new(past_date).unwrap());
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            due_date,
        );
        task.complete().unwrap();

        // Act & Assert
        assert!(!task.is_overdue());
    }

    #[test]
    fn test_change_title() {
        // Arrange
        let title = TaskTitle::new("元のタイトル").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let new_title = TaskTitle::new("新しいタイトル").unwrap();

        // Act
        let result = task.change_title(new_title.clone());

        // Assert
        assert!(result.is_ok());
        assert_eq!(task.title(), &new_title);
    }

    #[test]
    fn test_add_tag() {
        // Arrange
        let title = TaskTitle::new("タグ追加テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let tag_id = TagId::new(1).unwrap();

        // Act
        let result = task.add_tag(tag_id);

        // Assert
        assert!(result.is_ok());
        assert_eq!(task.tags().len(), 1);
        assert_eq!(task.tags()[0], tag_id);
    }

    #[test]
    fn test_add_duplicate_tag() {
        // Arrange
        let title = TaskTitle::new("重複タグテスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let tag_id = TagId::new(1).unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![tag_id],
            None,
        );

        // Act
        let result = task.add_tag(tag_id);

        // Assert
        assert!(result.is_err());
        assert_eq!(task.tags().len(), 1);
    }

    #[test]
    fn test_remove_tag() {
        // Arrange
        let title = TaskTitle::new("タグ削除テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let tag_id = TagId::new(1).unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![tag_id],
            None,
        );

        // Act
        let result = task.remove_tag(&tag_id);

        // Assert
        assert!(result.is_ok());
        assert_eq!(task.tags().len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_tag() {
        // Arrange
        let title = TaskTitle::new("存在しないタグ削除テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let tag_id = TagId::new(1).unwrap();

        // Act
        let result = task.remove_tag(&tag_id);

        // Assert
        assert!(result.is_err());
        assert_eq!(task.tags().len(), 0);
    }

    #[test]
    fn test_complete_emits_event() {
        // Arrange
        let title = TaskTitle::new("イベント発行テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );

        // Act
        task.complete().unwrap();

        // Assert
        assert_eq!(task.domain_events().len(), 1);
        assert!(
            task.domain_events()[0]
                .downcast_ref::<TaskCompleted>()
                .is_some()
        );
    }

    #[test]
    fn test_change_title_emits_event() {
        // Arrange
        let title = TaskTitle::new("元のタイトル").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let new_title = TaskTitle::new("新しいタイトル").unwrap();

        // Act
        task.change_title(new_title).unwrap();

        // Assert
        assert_eq!(task.domain_events().len(), 1);
        assert!(
            task.domain_events()[0]
                .downcast_ref::<TaskTitleChanged>()
                .is_some()
        );
    }

    #[test]
    fn test_add_tag_emits_event() {
        // Arrange
        let title = TaskTitle::new("タグ追加イベントテスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let tag_id = TagId::new(1).unwrap();

        // Act
        task.add_tag(tag_id).unwrap();

        // Assert
        assert_eq!(task.domain_events().len(), 1);
        assert!(
            task.domain_events()[0]
                .downcast_ref::<TaskTagAdded>()
                .is_some()
        );
    }

    #[test]
    fn test_remove_tag_emits_event() {
        // Arrange
        let title = TaskTitle::new("タグ削除イベントテスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let tag_id = TagId::new(1).unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![tag_id],
            None,
        );

        // Act
        task.remove_tag(&tag_id).unwrap();

        // Assert
        assert_eq!(task.domain_events().len(), 1);
        assert!(
            task.domain_events()[0]
                .downcast_ref::<TaskTagRemoved>()
                .is_some()
        );
    }

    #[test]
    fn test_clear_events() {
        // Arrange
        let title = TaskTitle::new("イベントクリアテスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        task.complete().unwrap();
        assert_eq!(task.domain_events().len(), 1);

        // Act
        task.clear_events();

        // Assert
        assert_eq!(task.domain_events().len(), 0);
    }

    #[test]
    fn test_change_status_to_completed_sets_completed_at() {
        // Arrange
        let title = TaskTitle::new("タスク").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );

        // Act
        let result = task.change_status(Status::Completed);

        // Assert
        assert!(result.is_ok());
        assert_eq!(task.status(), &Status::Completed);
        assert!(task.completed_at().is_some());
    }

    #[test]
    fn test_change_status_to_completed_emits_event() {
        // Arrange
        let title = TaskTitle::new("タスク").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::InProgress,
            Priority::Medium,
            vec![],
            None,
        );

        // Act
        task.change_status(Status::Completed).unwrap();

        // Assert
        assert_eq!(task.domain_events().len(), 1);
        assert!(
            task.domain_events()[0]
                .downcast_ref::<TaskCompleted>()
                .is_some()
        );
    }

    #[test]
    fn test_change_status_from_completed_clears_completed_at() {
        // Arrange
        let title = TaskTitle::new("タスク").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        task.complete().unwrap(); // 一度完了させる
        assert!(task.completed_at().is_some());

        // Act
        let result = task.change_status(Status::InProgress);

        // Assert
        assert!(result.is_ok());
        assert_eq!(task.status(), &Status::InProgress);
        assert!(task.completed_at().is_none());
    }

    #[test]
    fn test_change_status_to_non_completed_does_not_emit_event() {
        // Arrange
        let title = TaskTitle::new("タスク").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );

        // Act
        task.change_status(Status::InProgress).unwrap();

        // Assert
        assert_eq!(task.domain_events().len(), 0);
    }

    #[test]
    fn test_change_status_already_completed_does_not_change_completed_at() {
        // Arrange
        let title = TaskTitle::new("タスク").unwrap();
        let description = TaskDescription::new("").unwrap();
        let mut task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        task.complete().unwrap();
        let first_completed_at = *task.completed_at();

        // Act
        task.change_status(Status::Completed).unwrap();

        // Assert
        assert_eq!(task.completed_at(), &first_completed_at);
    }
}
