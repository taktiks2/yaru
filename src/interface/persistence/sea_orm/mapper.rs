use anyhow::Result;
use chrono::Utc;
use entity::{tags, tasks};
use sea_orm::ActiveValue::Set;

use crate::domain::tag::{
    aggregate::{TagAggregate, TagReconstructParams},
    value_objects::{TagDescription, TagId, TagName},
};
use crate::domain::task::{
    aggregate::{TaskAggregate, TaskReconstructParams},
    value_objects::{DueDate, Priority, Status, TaskDescription, TaskId, TaskTitle},
};

/// TaskMapper - TaskAggregateとSeaORM Entityの相互変換
pub struct TaskMapper;

impl TaskMapper {
    /// SeaORM ModelからTaskAggregateに変換
    ///
    /// # Arguments
    /// * `task_model` - tasks::Model
    /// * `tag_ids` - タスクに紐づくタグIDのリスト
    pub fn to_domain(task_model: tasks::Model, tag_ids: Vec<i32>) -> Result<TaskAggregate> {
        // Status変換
        let status = match task_model.status.as_str() {
            "Pending" => Status::Pending,
            "InProgress" => Status::InProgress,
            "Completed" => Status::Completed,
            _ => anyhow::bail!("不明なステータス: {}", task_model.status),
        };

        // Priority変換
        let priority = match task_model.priority.as_str() {
            "Low" => Priority::Low,
            "Medium" => Priority::Medium,
            "High" => Priority::High,
            "Critical" => Priority::Critical,
            _ => anyhow::bail!("不明な優先度: {}", task_model.priority),
        };

        // TagId変換
        let tag_id_vos: Result<Vec<_>> = tag_ids
            .into_iter()
            .map(|id| {
                use crate::domain::tag::value_objects::TagId as TagIdVO;
                TagIdVO::new(id)
            })
            .collect();

        // DueDate変換
        let due_date = task_model.due_date.map(DueDate::new).transpose()?;

        // Aggregateを再構築
        let params = TaskReconstructParams {
            id: TaskId::new(task_model.id)?,
            title: TaskTitle::new(task_model.title)?,
            description: TaskDescription::new(task_model.description)?,
            status,
            priority,
            tags: tag_id_vos?,
            created_at: task_model.created_at.into(),
            updated_at: task_model.updated_at.into(),
            due_date,
            completed_at: task_model.completed_at.map(|dt| dt.into()),
        };

        Ok(TaskAggregate::reconstruct(params))
    }

    /// TaskAggregateからSeaORM ActiveModelに変換（新規作成用）
    pub fn to_active_model_for_insert(aggregate: &TaskAggregate) -> tasks::ActiveModel {
        tasks::ActiveModel {
            id: Set(aggregate.id().value()),
            title: Set(aggregate.title().value().to_string()),
            description: Set(aggregate.description().value().to_string()),
            status: Set(Self::status_to_string(aggregate.status())),
            priority: Set(Self::priority_to_string(aggregate.priority())),
            created_at: Set((*aggregate.created_at()).into()),
            updated_at: Set((*aggregate.updated_at()).into()),
            due_date: Set(aggregate.due_date().as_ref().map(|dd| dd.value())),
            completed_at: Set(aggregate.completed_at().map(|dt| dt.into())),
        }
    }

    /// TaskAggregateからSeaORM ActiveModelに変換（更新用）
    pub fn to_active_model_for_update(aggregate: &TaskAggregate) -> tasks::ActiveModel {
        tasks::ActiveModel {
            id: Set(aggregate.id().value()),
            title: Set(aggregate.title().value().to_string()),
            description: Set(aggregate.description().value().to_string()),
            status: Set(Self::status_to_string(aggregate.status())),
            priority: Set(Self::priority_to_string(aggregate.priority())),
            created_at: Set((*aggregate.created_at()).into()),
            updated_at: Set(Utc::now().into()),
            due_date: Set(aggregate.due_date().as_ref().map(|dd| dd.value())),
            completed_at: Set(aggregate.completed_at().map(|dt| dt.into())),
        }
    }

    fn status_to_string(status: &Status) -> String {
        match status {
            Status::Pending => "Pending".to_string(),
            Status::InProgress => "InProgress".to_string(),
            Status::Completed => "Completed".to_string(),
        }
    }

    fn priority_to_string(priority: &Priority) -> String {
        match priority {
            Priority::Low => "Low".to_string(),
            Priority::Medium => "Medium".to_string(),
            Priority::High => "High".to_string(),
            Priority::Critical => "Critical".to_string(),
        }
    }
}

/// TagMapper - TagAggregateとSeaORM Entityの相互変換
pub struct TagMapper;

impl TagMapper {
    /// SeaORM ModelからTagAggregateに変換
    pub fn to_domain(tag_model: tags::Model) -> Result<TagAggregate> {
        let params = TagReconstructParams {
            id: TagId::new(tag_model.id)?,
            name: TagName::new(tag_model.name)?,
            description: TagDescription::new(tag_model.description)?,
            created_at: tag_model.created_at.into(),
            updated_at: tag_model.updated_at.into(),
        };

        Ok(TagAggregate::reconstruct(params))
    }

    /// TagAggregateからSeaORM ActiveModelに変換（新規作成用）
    pub fn to_active_model_for_insert(aggregate: &TagAggregate) -> tags::ActiveModel {
        tags::ActiveModel {
            id: Set(aggregate.id().value()),
            name: Set(aggregate.name().value().to_string()),
            description: Set(aggregate.description().value().to_string()),
            created_at: Set((*aggregate.created_at()).into()),
            updated_at: Set((*aggregate.updated_at()).into()),
        }
    }

    /// TagAggregateからSeaORM ActiveModelに変換（更新用）
    pub fn to_active_model_for_update(aggregate: &TagAggregate) -> tags::ActiveModel {
        tags::ActiveModel {
            id: Set(aggregate.id().value()),
            name: Set(aggregate.name().value().to_string()),
            description: Set(aggregate.description().value().to_string()),
            created_at: Set((*aggregate.created_at()).into()),
            updated_at: Set(Utc::now().into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_task_mapper_to_domain() {
        // Arrange
        let task_model = tasks::Model {
            id: 1,
            title: "テストタスク".to_string(),
            description: "説明".to_string(),
            status: "Pending".to_string(),
            priority: "High".to_string(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            due_date: Some(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()),
            completed_at: None,
        };
        let tag_ids = vec![1, 2];

        // Act
        let result = TaskMapper::to_domain(task_model, tag_ids);

        // Assert
        assert!(result.is_ok());
        let aggregate = result.unwrap();
        assert_eq!(aggregate.id().value(), 1);
        assert_eq!(aggregate.title().value(), "テストタスク");
        assert_eq!(aggregate.tags().len(), 2);
    }

    #[test]
    fn test_tag_mapper_to_domain() {
        // Arrange
        let tag_model = tags::Model {
            id: 1,
            name: "重要".to_string(),
            description: "重要なタスク".to_string(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        };

        // Act
        let result = TagMapper::to_domain(tag_model);

        // Assert
        assert!(result.is_ok());
        let aggregate = result.unwrap();
        assert_eq!(aggregate.id().value(), 1);
        assert_eq!(aggregate.name().value(), "重要");
    }
}
