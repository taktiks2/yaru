use anyhow::Result;
use async_trait::async_trait;
use entity::{tags, task_tags};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, ColumnTrait, QueryFilter, PaginatorTrait};

use crate::domain::tag::{aggregate::TagAggregate, repository::TagRepository, value_objects::TagId};
use super::mapper::TagMapper;

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
        let tag_model = tags::Entity::find_by_id(id.value()).one(&self.db).await?;

        match tag_model {
            Some(model) => {
                let aggregate = TagMapper::to_domain(model)?;
                Ok(Some(aggregate))
            }
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<TagAggregate>> {
        let tag_models = tags::Entity::find().all(&self.db).await?;

        let mut aggregates = Vec::new();
        for model in tag_models {
            let aggregate = TagMapper::to_domain(model)?;
            aggregates.push(aggregate);
        }

        Ok(aggregates)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<TagAggregate>> {
        let tag_model = tags::Entity::find()
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
        // タグの保存（IDが0の場合は新規作成、それ以外はそのまま使う）
        let tag_to_save = if tag.id().value() == 0 {
            // 新規作成の場合、データベースに任せるためにIDを無視
            let mut active_model = TagMapper::to_active_model_for_insert(&tag);
            active_model.id = sea_orm::ActiveValue::NotSet;
            let saved_model = active_model.insert(&self.db).await?;

            TagMapper::to_domain(saved_model)?
        } else {
            // 既存IDがある場合はそのまま使用
            let active_model = TagMapper::to_active_model_for_insert(&tag);
            let saved_model = active_model.insert(&self.db).await?;

            TagMapper::to_domain(saved_model)?
        };

        Ok(tag_to_save)
    }

    async fn update(&self, tag: TagAggregate) -> Result<TagAggregate> {
        // 既存のタグを取得
        let existing = tags::Entity::find_by_id(tag.id().value())
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
        let task_count = task_tags::Entity::find()
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

        let result = tags::Entity::delete_by_id(id.value())
            .exec(&self.db)
            .await?;

        Ok(result.rows_affected > 0)
    }
}
