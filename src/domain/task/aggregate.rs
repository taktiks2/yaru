use anyhow::{bail, Result};
use chrono::{DateTime, Utc};

use super::value_objects::{
    DueDate, Priority, Status, TaskDescription, TaskId, TaskTitle,
};
use crate::domain::tag::value_objects::TagId;

/// TaskAggregate - タスクのAggregate Root
///
/// タスクのビジネスルールを実装し、不変条件を保護します。
#[derive(Debug, Clone, PartialEq)]
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
        }
    }

    /// タスクを完了します
    ///
    /// ステータスをCompletedに変更し、completed_atを現在時刻に設定します。
    /// 既に完了している場合は、何もしません。
    pub fn complete(&mut self) -> Result<()> {
        if self.status != Status::Completed {
            self.status = Status::Completed;
            self.completed_at = Some(Utc::now());
            self.updated_at = Utc::now();
        }
        Ok(())
    }

    /// タスクが期限切れかどうかを判定します
    ///
    /// 以下の条件で期限切れと判定されます：
    /// - 期限が設定されている
    /// - 現在の日付が期限を過ぎている
    /// - タスクが完了していない
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
        self.title = new_title;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// タスクにタグを追加します
    ///
    /// 既に同じタグが存在する場合はエラーを返します。
    pub fn add_tag(&mut self, tag_id: TagId) -> Result<()> {
        if self.tags.contains(&tag_id) {
            bail!("タグID {} は既に追加されています", tag_id.value());
        }
        self.tags.push(tag_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// タスクからタグを削除します
    ///
    /// 指定したタグが存在しない場合はエラーを返します。
    pub fn remove_tag(&mut self, tag_id: &TagId) -> Result<()> {
        let original_len = self.tags.len();
        self.tags.retain(|t| t != tag_id);

        if self.tags.len() == original_len {
            bail!("タグID {} は存在しません", tag_id.value());
        }

        self.updated_at = Utc::now();
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
            status.clone(),
            priority.clone(),
            tags.clone(),
            due_date.clone(),
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
        let first_completed_at = task.completed_at().clone();

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
        let result = task.add_tag(tag_id.clone());

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
            vec![tag_id.clone()],
            None,
        );

        // Act
        let result = task.add_tag(tag_id.clone());

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
            vec![tag_id.clone()],
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
}
