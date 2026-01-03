use crate::{
    domain::tag::{aggregate::TagAggregate, repository::TagRepository, value_objects::TagId},
    interface::persistence::sea_orm::mapper::TagMapper,
};
use anyhow::Result;
use async_trait::async_trait;
use entity::{prelude::{Tags, TaskTags}, tags, task_tags};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};

/// SeaORM実装のTagRepository
pub struct SeaOrmTagRepository {
    db: DatabaseConnection,
}

impl SeaOrmTagRepository {
    /// 新しいSeaOrmTagRepositoryを作成
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TagRepository for SeaOrmTagRepository {
    async fn find_by_id(&self, id: &TagId) -> Result<Option<TagAggregate>> {
        let tag_model = Tags::find_by_id(id.value()).one(&self.db).await?;

        match tag_model {
            Some(model) => {
                let aggregate = TagMapper::to_domain(model)?;
                Ok(Some(aggregate))
            }
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<TagAggregate>> {
        let tag_models = Tags::find().all(&self.db).await?;

        let mut aggregates = Vec::new();
        for model in tag_models {
            let aggregate = TagMapper::to_domain(model)?;
            aggregates.push(aggregate);
        }

        Ok(aggregates)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<TagAggregate>> {
        let tag_model = Tags::find()
            .filter(tags::Column::Name.eq(name))
            .one(&self.db)
            .await?;

        match tag_model {
            Some(model) => {
                let aggregate = TagMapper::to_domain(model)?;
                Ok(Some(aggregate))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, tag: TagAggregate) -> Result<TagAggregate> {
        // タグの保存（IDが0の場合は新規作成、それ以外は更新）
        let tag_to_save = if tag.id().value() == 0 {
            // 新規作成
            let active_model = TagMapper::to_active_model_for_insert(&tag);
            let saved_model = active_model.insert(&self.db).await?;

            TagMapper::to_domain(saved_model)?
        } else {
            // 既存IDがある場合は更新
            self.update(tag).await?
        };

        Ok(tag_to_save)
    }

    async fn update(&self, tag: TagAggregate) -> Result<TagAggregate> {
        // 既存のタグを取得
        let existing = Tags::find_by_id(tag.id().value())
            .one(&self.db)
            .await?;

        if existing.is_none() {
            anyhow::bail!("タグID {}は存在しません", tag.id().value());
        }

        // タグを更新
        let active_model = TagMapper::to_active_model_for_update(&tag);
        let updated_model = active_model.update(&self.db).await?;

        let aggregate = TagMapper::to_domain(updated_model)?;

        Ok(aggregate)
    }

    async fn delete(&self, id: &TagId) -> Result<bool> {
        // タグが使用されているかチェック（RESTRICT制約）
        let task_count = TaskTags::find()
            .filter(task_tags::Column::TagId.eq(id.value()))
            .count(&self.db)
            .await?;

        if task_count > 0 {
            anyhow::bail!(
                "タグID {}は{}個のタスクで使用されているため削除できません",
                id.value(),
                task_count
            );
        }

        let result = Tags::delete_by_id(id.value())
            .exec(&self.db)
            .await?;

        Ok(result.rows_affected > 0)
    }
}
