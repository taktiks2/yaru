use anyhow::{Result, bail};
use std::sync::{Arc, RwLock};

use crate::domain::tag::{
    aggregate::TagAggregate, repository::TagRepository, value_objects::TagId,
};

/// InMemoryTagRepository - テスト用のタグリポジトリ実装
///
/// メモリ上にタグを保持します。本番環境では使用しないでください。
#[derive(Clone)]
#[allow(dead_code)]
pub struct InMemoryTagRepository {
    tags: Arc<RwLock<Vec<TagAggregate>>>,
    next_id: Arc<RwLock<i32>>,
}

impl InMemoryTagRepository {
    /// 新しいInMemoryTagRepositoryを作成
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            tags: Arc::new(RwLock::new(Vec::new())),
            next_id: Arc::new(RwLock::new(1)),
        }
    }

    /// 次のIDを生成
    #[allow(dead_code)]
    fn generate_id(&self) -> Result<i32> {
        let mut next_id = self.next_id.write().unwrap();
        let id = *next_id;
        *next_id += 1;
        Ok(id)
    }
}

impl Default for InMemoryTagRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl TagRepository for InMemoryTagRepository {
    async fn find_by_id(&self, id: &TagId) -> Result<Option<TagAggregate>> {
        let tags = self.tags.read().unwrap();
        Ok(tags.iter().find(|t| t.id() == id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<TagAggregate>> {
        let tags = self.tags.read().unwrap();
        Ok(tags.clone())
    }

    async fn save(&self, tag: TagAggregate) -> Result<TagAggregate> {
        let tag_to_save = if tag.id().value() == 0 {
            let new_id = self.generate_id()?;
            tag.with_id(TagId::new(new_id)?)
        } else {
            tag
        };

        let mut tags = self.tags.write().unwrap();

        if let Some(index) = tags.iter().position(|t| t.id() == tag_to_save.id()) {
            tags[index] = tag_to_save.clone();
        } else {
            tags.push(tag_to_save.clone());
        }

        Ok(tag_to_save)
    }

    async fn update(&self, tag: TagAggregate) -> Result<TagAggregate> {
        let mut tags = self.tags.write().unwrap();

        if let Some(index) = tags.iter().position(|t| t.id() == tag.id()) {
            tags[index] = tag.clone();
            Ok(tag)
        } else {
            bail!("タグが見つかりません: {}", tag.id().value())
        }
    }

    async fn delete(&self, id: &TagId) -> Result<bool> {
        let mut tags = self.tags.write().unwrap();

        if let Some(index) = tags.iter().position(|t| t.id() == id) {
            tags.remove(index);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<TagAggregate>> {
        let tags = self.tags.read().unwrap();
        Ok(tags.iter().find(|t| t.name().value() == name).cloned())
    }
}
