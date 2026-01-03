use anyhow::Result;
use async_trait::async_trait;
use entity::{task_tags, tasks};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
    QueryFilter, QuerySelect, RelationTrait,
};

use crate::domain::task::{
    aggregate::TaskAggregate, repository::TaskRepository, specification::TaskSpecification,
    value_objects::TaskId,
};
use super::mapper::TaskMapper;

/// SeaORM実装のTaskRepository
pub struct SeaOrmTaskRepository {
    db: DatabaseConnection,
}

impl SeaOrmTaskRepository {
    /// 新しいSeaOrmTaskRepositoryを作成
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// タスクに紐づくタグIDのリストを取得
    async fn get_tag_ids(&self, task_id: i32) -> Result<Vec<i32>> {
        let tag_ids = task_tags::Entity::find()
            .filter(task_tags::Column::TaskId.eq(task_id))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|tt| tt.tag_id)
            .collect();

        Ok(tag_ids)
    }

    /// タスクのタグ関連付けを更新（既存を削除して新規作成）
    async fn update_task_tags(&self, task_id: i32, tag_ids: &[i32]) -> Result<()> {
        // 既存のタグ関連付けを削除
        task_tags::Entity::delete_many()
            .filter(task_tags::Column::TaskId.eq(task_id))
            .exec(&self.db)
            .await?;

        // 新しいタグ関連付けを作成
        for tag_id in tag_ids {
            let task_tag = task_tags::ActiveModel {
                task_id: Set(task_id),
                tag_id: Set(*tag_id),
            };
            task_tag.insert(&self.db).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl TaskRepository for SeaOrmTaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskAggregate>> {
        let task_model = tasks::Entity::find_by_id(id.value()).one(&self.db).await?;

        match task_model {
            Some(model) => {
                let tag_ids = self.get_tag_ids(model.id).await?;
                let aggregate = TaskMapper::to_domain(model, tag_ids)?;
                Ok(Some(aggregate))
            }
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<TaskAggregate>> {
        let task_models = tasks::Entity::find().all(&self.db).await?;

        let mut aggregates = Vec::new();
        for model in task_models {
            let tag_ids = self.get_tag_ids(model.id).await?;
            let aggregate = TaskMapper::to_domain(model, tag_ids)?;
            aggregates.push(aggregate);
        }

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
        // タスクの保存（IDが0の場合は新規作成、それ以外はそのまま使う）
        let task_to_save = if task.id().value() == 0 {
            // 新規作成の場合、データベースに任せるためにIDを無視
            let mut active_model = TaskMapper::to_active_model_for_insert(&task);
            active_model.id = sea_orm::ActiveValue::NotSet;
            let saved_model = active_model.insert(&self.db).await?;

            // タグの関連付けを保存
            let tag_ids: Vec<i32> = task.tags().iter().map(|tag_id| tag_id.value()).collect();
            self.update_task_tags(saved_model.id, &tag_ids).await?;

            // 保存されたタスクを取得して返す
            let tag_ids = self.get_tag_ids(saved_model.id).await?;
            TaskMapper::to_domain(saved_model, tag_ids)?
        } else {
            // 既存IDがある場合はそのまま使用
            let active_model = TaskMapper::to_active_model_for_insert(&task);
            let saved_model = active_model.insert(&self.db).await?;

            // タグの関連付けを保存
            let tag_ids: Vec<i32> = task.tags().iter().map(|tag_id| tag_id.value()).collect();
            self.update_task_tags(saved_model.id, &tag_ids).await?;

            // 保存されたタスクを取得して返す
            let tag_ids = self.get_tag_ids(saved_model.id).await?;
            TaskMapper::to_domain(saved_model, tag_ids)?
        };

        Ok(task_to_save)
    }

    async fn update(&self, task: TaskAggregate) -> Result<TaskAggregate> {
        // 既存のタスクを取得
        let existing = tasks::Entity::find_by_id(task.id().value())
            .one(&self.db)
            .await?;

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
        let tag_ids = self.get_tag_ids(updated_model.id).await?;
        let aggregate = TaskMapper::to_domain(updated_model, tag_ids)?;

        Ok(aggregate)
    }

    async fn delete(&self, id: &TaskId) -> Result<bool> {
        let result = tasks::Entity::delete_by_id(id.value())
            .exec(&self.db)
            .await?;

        // task_tagsは CASCADE DELETE で自動削除される

        Ok(result.rows_affected > 0)
    }
}
