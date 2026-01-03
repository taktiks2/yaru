#![allow(dead_code)]

use super::aggregate::TaskAggregate;
use super::value_objects::{Priority, Status, TaskId};
use crate::domain::tag::value_objects::TagId;

/// TaskSpecification trait - タスクの検索条件を抽象化
///
/// Specification Patternにより、複雑なクエリロジックを抽象化します。
/// メモリフィルタとSQLクエリの両方に対応可能な設計です。
pub trait TaskSpecification: Send + Sync {
    /// タスクが条件を満たすかを判定（メモリフィルタ用）
    ///
    /// # Arguments
    /// * `task` - 判定対象のタスク
    ///
    /// # Returns
    /// * `true` - 条件を満たす
    /// * `false` - 条件を満たさない
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool;

    /// 他のSpecificationとAND結合
    fn and(self: Box<Self>, other: Box<dyn TaskSpecification>) -> Box<dyn TaskSpecification>
    where
        Self: 'static + Sized,
    {
        Box::new(AndSpecification {
            left: self,
            right: other,
        })
    }

    /// 他のSpecificationとOR結合
    fn or(self: Box<Self>, other: Box<dyn TaskSpecification>) -> Box<dyn TaskSpecification>
    where
        Self: 'static + Sized,
    {
        Box::new(OrSpecification {
            left: self,
            right: other,
        })
    }
}

/// ステータスでフィルタリング
#[derive(Debug, Clone)]
pub struct TaskByStatus {
    status: Status,
}

impl TaskByStatus {
    pub fn new(status: Status) -> Self {
        Self { status }
    }
}

impl TaskSpecification for TaskByStatus {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.status() == &self.status
    }
}

/// 優先度でフィルタリング
#[derive(Debug, Clone)]
pub struct TaskByPriority {
    priority: Priority,
}

impl TaskByPriority {
    pub fn new(priority: Priority) -> Self {
        Self { priority }
    }
}

impl TaskSpecification for TaskByPriority {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.priority() == &self.priority
    }
}

/// タグでフィルタリング
#[derive(Debug, Clone)]
pub struct TaskByTag {
    tag_id: TagId,
}

impl TaskByTag {
    pub fn new(tag_id: TagId) -> Self {
        Self { tag_id }
    }
}

impl TaskSpecification for TaskByTag {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.tags().contains(&self.tag_id)
    }
}

/// 期限切れタスクでフィルタリング
#[derive(Debug, Clone)]
pub struct TaskOverdue;

impl TaskOverdue {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TaskOverdue {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskSpecification for TaskOverdue {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.is_overdue()
    }
}

/// IDでフィルタリング
#[derive(Debug, Clone)]
pub struct TaskById {
    id: TaskId,
}

impl TaskById {
    pub fn new(id: TaskId) -> Self {
        Self { id }
    }
}

impl TaskSpecification for TaskById {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.id() == &self.id
    }
}

/// AND条件
pub struct AndSpecification {
    left: Box<dyn TaskSpecification>,
    right: Box<dyn TaskSpecification>,
}

impl TaskSpecification for AndSpecification {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        self.left.is_satisfied_by(task) && self.right.is_satisfied_by(task)
    }
}

/// OR条件
pub struct OrSpecification {
    left: Box<dyn TaskSpecification>,
    right: Box<dyn TaskSpecification>,
}

impl TaskSpecification for OrSpecification {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        self.left.is_satisfied_by(task) || self.right.is_satisfied_by(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::value_objects::{TaskDescription, TaskTitle};
    use chrono::{Duration, Utc};

    #[test]
    fn test_task_by_status() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskByStatus::new(Status::Pending);

        // Act & Assert
        assert!(spec.is_satisfied_by(&task));

        let spec_completed = TaskByStatus::new(Status::Completed);
        assert!(!spec_completed.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_priority() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );
        let spec = TaskByPriority::new(Priority::High);

        // Act & Assert
        assert!(spec.is_satisfied_by(&task));

        let spec_low = TaskByPriority::new(Priority::Low);
        assert!(!spec_low.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_tag() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let tag_id = TagId::new(1).unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![tag_id],
            None,
        );
        let spec = TaskByTag::new(tag_id);

        // Act & Assert
        assert!(spec.is_satisfied_by(&task));

        let other_tag_id = TagId::new(2).unwrap();
        let spec_other = TaskByTag::new(other_tag_id);
        assert!(!spec_other.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_overdue() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let past_date = Utc::now().naive_utc().date() - Duration::days(1);
        let due_date = Some(super::super::value_objects::DueDate::new(past_date).unwrap());
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            due_date,
        );
        let spec = TaskOverdue::new();

        // Act & Assert
        assert!(spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_id() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskById::new(TaskId::new(0).unwrap());

        // Act & Assert
        assert!(spec.is_satisfied_by(&task));

        let spec_other = TaskById::new(TaskId::new(999).unwrap());
        assert!(!spec_other.is_satisfied_by(&task));
    }

    #[test]
    fn test_and_specification() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );

        let spec_status = Box::new(TaskByStatus::new(Status::Pending));
        let spec_priority: Box<dyn TaskSpecification> =
            Box::new(TaskByPriority::new(Priority::High));
        let and_spec = spec_status.and(spec_priority);

        // Act & Assert
        assert!(and_spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_or_specification() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );

        let spec_status = Box::new(TaskByStatus::new(Status::Completed));
        let spec_priority: Box<dyn TaskSpecification> =
            Box::new(TaskByPriority::new(Priority::High));
        let or_spec = spec_status.or(spec_priority);

        // Act & Assert
        assert!(or_spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_complex_specification() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let tag_id = TagId::new(1).unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![tag_id],
            None,
        );

        // (Status == Pending AND Priority == High) OR Tag == 1
        let spec_status: Box<dyn TaskSpecification> = Box::new(TaskByStatus::new(Status::Pending));
        let spec_priority: Box<dyn TaskSpecification> =
            Box::new(TaskByPriority::new(Priority::High));
        let spec_tag: Box<dyn TaskSpecification> = Box::new(TaskByTag::new(tag_id));

        let and_spec = AndSpecification {
            left: spec_status,
            right: spec_priority,
        };
        let complex_spec = OrSpecification {
            left: Box::new(and_spec),
            right: spec_tag,
        };

        // Act & Assert
        assert!(complex_spec.is_satisfied_by(&task));
    }
}
