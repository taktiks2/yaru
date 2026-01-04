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
    prelude::{TaskTags, Tasks},
    task_tags,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use std::collections::HashMap;

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
        let tag_ids = TaskTags::find()
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
        let task_model = Tasks::find_by_id(id.value()).one(&self.db).await?;

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
        // 1. 全タスクを取得
        let task_models = Tasks::find().all(&self.db).await?;

        if task_models.is_empty() {
            return Ok(Vec::new());
        }

        // 2. 全タスクのIDを収集
        let task_ids: Vec<i32> = task_models.iter().map(|t| t.id).collect();

        // 3. 全タグ関連付けを一括取得（N+1問題の回避）
        let task_tags = TaskTags::find()
            .filter(task_tags::Column::TaskId.is_in(task_ids))
            .all(&self.db)
            .await?;

        // 4. タスクIDごとにタグIDをグループ化
        let mut tag_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for tt in task_tags {
            tag_map.entry(tt.task_id).or_default().push(tt.tag_id);
        }

        // 5. ドメインモデルに変換
        let mut aggregates = Vec::new();
        for model in task_models {
            let tag_ids = tag_map.get(&model.id).cloned().unwrap_or_default();
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
        // タスクの保存（IDが0の場合は新規作成、それ以外は更新）
        let task_to_save = if task.id().value() == 0 {
            // 新規作成
            let active_model = TaskMapper::to_active_model_for_insert(&task);
            let saved_model = active_model.insert(&self.db).await?;

            // タグの関連付けを保存
            let tag_ids: Vec<i32> = task.tags().iter().map(|tag_id| tag_id.value()).collect();
            self.update_task_tags(saved_model.id, &tag_ids).await?;

            // 保存されたタスクを取得して返す
            let tag_ids = self.get_tag_ids(saved_model.id).await?;
            TaskMapper::to_domain(saved_model, tag_ids)?
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
        let tag_ids = self.get_tag_ids(updated_model.id).await?;
        let aggregate = TaskMapper::to_domain(updated_model, tag_ids)?;

        Ok(aggregate)
    }

    async fn delete(&self, id: &TaskId) -> Result<bool> {
        let result = Tasks::delete_by_id(id.value()).exec(&self.db).await?;

        // task_tagsは CASCADE DELETE で自動削除される

        Ok(result.rows_affected > 0)
    }
}
