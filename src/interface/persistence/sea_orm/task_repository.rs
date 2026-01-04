use crate::{
    domain::task::{
        aggregate::TaskAggregate, repository::TaskRepository, specification::TaskSpecification,
        value_objects::TaskId,
    },
    interface::persistence::sea_orm::mapper::TaskMapper,
};
use anyhow::Result;
use async_trait::async_trait;
use entity::{
    prelude::{Tags, TaskTags, Tasks},
    task_tags,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

/// SeaORM実装のTaskRepository
pub struct SeaOrmTaskRepository {
    db: DatabaseConnection,
}

impl SeaOrmTaskRepository {
    /// 新しいSeaOrmTaskRepositoryを作成
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// タスクのタグ関連付けを更新（既存を削除して新規作成）
    async fn update_task_tags(&self, task_id: i32, tag_ids: &[i32]) -> Result<()> {
        // 既存のタグ関連付けを削除
        TaskTags::delete_many()
            .filter(task_tags::Column::TaskId.eq(task_id))
            .exec(&self.db)
            .await?;

        // 新しいタグ関連付けを一括作成（N+1問題の回避）
        if !tag_ids.is_empty() {
            let task_tag_models: Vec<task_tags::ActiveModel> = tag_ids
                .iter()
                .map(|tag_id| task_tags::ActiveModel {
                    task_id: Set(task_id),
                    tag_id: Set(*tag_id),
                })
                .collect();

            TaskTags::insert_many(task_tag_models)
                .exec(&self.db)
                .await?;
        }

        Ok(())
    }
}

#[async_trait]
impl TaskRepository for SeaOrmTaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskAggregate>> {
        let result = Tasks::find_by_id(id.value())
            .find_with_related(Tags)
            .all(&self.db)
            .await?;

        if result.is_empty() {
            return Ok(None);
        }

        let (task_model, tags) = &result[0];
        let tag_ids: Vec<i32> = tags.iter().map(|tag| tag.id).collect();
        let aggregate = TaskMapper::to_domain(task_model.clone(), tag_ids)?;
        Ok(Some(aggregate))
    }

    async fn find_all(&self) -> Result<Vec<TaskAggregate>> {
        // find_with_relatedを使って一括取得（N+1問題の回避）
        let tasks_with_tags = Tasks::find()
            .find_with_related(Tags)
            .all(&self.db)
            .await?;

        let aggregates = tasks_with_tags
            .into_iter()
            .map(|(task_model, tags)| {
                let tag_ids: Vec<i32> = tags.iter().map(|tag| tag.id).collect();
                TaskMapper::to_domain(task_model, tag_ids)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(aggregates)
    }

    async fn find_by_specification(
        &self,
        spec: Box<dyn TaskSpecification>,
    ) -> Result<Vec<TaskAggregate>> {
        // すべてのタスクを取得してメモリ上でフィルタリング
        // 将来的にはSeaORM用のクエリビルダーで最適化可能
        let all_tasks = self.find_all().await?;
        let filtered_tasks: Vec<TaskAggregate> = all_tasks
            .into_iter()
            .filter(|task| spec.is_satisfied_by(task))
            .collect();

        Ok(filtered_tasks)
    }

    async fn save(&self, task: TaskAggregate) -> Result<TaskAggregate> {
        // タスクの保存（IDが0の場合は新規作成、それ以外は更新）
        let task_to_save = if task.id().value() == 0 {
            // 新規作成
            let active_model = TaskMapper::to_active_model_for_insert(&task);
            let saved_model = active_model.insert(&self.db).await?;

            // タグの関連付けを保存
            let tag_ids: Vec<i32> = task.tags().iter().map(|tag_id| tag_id.value()).collect();
            self.update_task_tags(saved_model.id, &tag_ids).await?;

            // 保存されたタスクを取得して返す
            let result = Tasks::find_by_id(saved_model.id)
                .find_with_related(Tags)
                .all(&self.db)
                .await?;
            let (task_model, tags) = &result[0];
            let tag_ids: Vec<i32> = tags.iter().map(|tag| tag.id).collect();
            TaskMapper::to_domain(task_model.clone(), tag_ids)?
        } else {
            // 既存IDがある場合は更新
            self.update(task).await?
        };

        Ok(task_to_save)
    }

    async fn update(&self, task: TaskAggregate) -> Result<TaskAggregate> {
        // 既存のタスクを取得
        let existing = Tasks::find_by_id(task.id().value()).one(&self.db).await?;

        if existing.is_none() {
            anyhow::bail!("タスクID {}は存在しません", task.id().value());
        }

        // タスクを更新
        let active_model = TaskMapper::to_active_model_for_update(&task);
        let updated_model = active_model.update(&self.db).await?;

        // タグの関連付けを更新
        let tag_ids: Vec<i32> = task.tags().iter().map(|tag_id| tag_id.value()).collect();
        self.update_task_tags(updated_model.id, &tag_ids).await?;

        // 更新されたタスクを取得して返す
        let result = Tasks::find_by_id(updated_model.id)
            .find_with_related(Tags)
            .all(&self.db)
            .await?;
        let (task_model, tags) = &result[0];
        let tag_ids: Vec<i32> = tags.iter().map(|tag| tag.id).collect();
        let aggregate = TaskMapper::to_domain(task_model.clone(), tag_ids)?;

        Ok(aggregate)
    }

    async fn delete(&self, id: &TaskId) -> Result<bool> {
        let result = Tasks::delete_by_id(id.value()).exec(&self.db).await?;

        // task_tagsは CASCADE DELETE で自動削除される

        Ok(result.rows_affected > 0)
    }
}
