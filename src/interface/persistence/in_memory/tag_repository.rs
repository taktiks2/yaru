#[cfg(test)]
use crate::domain::tag::{
    aggregate::TagAggregate, repository::TagRepository, value_objects::TagId,
};
#[cfg(test)]
use anyhow::{Result, bail};
#[cfg(test)]
use std::sync::{Arc, RwLock};

/// InMemoryTagRepository - テスト用のタグリポジトリ実装
///
/// メモリ上にタグを保持します。本番環境では使用しないでください。
#[derive(Clone)]
#[cfg(test)]
pub struct InMemoryTagRepository {
    tags: Arc<RwLock<Vec<TagAggregate>>>,
    next_id: Arc<RwLock<i32>>,
}

#[cfg(test)]
impl InMemoryTagRepository {
    /// 新しいInMemoryTagRepositoryを作成
    pub fn new() -> Self {
        Self {
            tags: Arc::new(RwLock::new(Vec::new())),
            next_id: Arc::new(RwLock::new(1)),
        }
    }

    /// 次のIDを生成
    fn generate_id(&self) -> Result<i32> {
        let mut next_id = self.next_id.write().unwrap();
        let id = *next_id;
        *next_id += 1;
        Ok(id)
    }
}

#[cfg(test)]
impl Default for InMemoryTagRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
#[cfg(test)]
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

    async fn find_by_ids(&self, ids: &[TagId]) -> Result<Vec<TagAggregate>> {
        let tags = self.tags.read().unwrap();
        let id_set: std::collections::HashSet<_> = ids.iter().map(|id| id.value()).collect();
        let result = tags
            .iter()
            .filter(|tag| id_set.contains(&tag.id().value()))
            .cloned()
            .collect();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tag::value_objects::{TagDescription, TagName};

    #[tokio::test]
    async fn test_find_by_ids_multiple_tags_found() {
        // Arrange
        let repo = InMemoryTagRepository::new();
        let tag1 = TagAggregate::new(
            TagName::new("タグ1").unwrap(),
            TagDescription::new("説明1").unwrap(),
        );
        let tag2 = TagAggregate::new(
            TagName::new("タグ2").unwrap(),
            TagDescription::new("説明2").unwrap(),
        );
        let saved_tag1 = repo.save(tag1).await.unwrap();
        let saved_tag2 = repo.save(tag2).await.unwrap();

        let ids = vec![*saved_tag1.id(), *saved_tag2.id()];

        // Act
        let result = repo.find_by_ids(&ids).await.unwrap();

        // Assert
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|t| t.id() == saved_tag1.id()));
        assert!(result.iter().any(|t| t.id() == saved_tag2.id()));
    }

    #[tokio::test]
    async fn test_find_by_ids_some_ids_not_found() {
        // Arrange
        let repo = InMemoryTagRepository::new();
        let tag = TagAggregate::new(
            TagName::new("タグ").unwrap(),
            TagDescription::new("説明").unwrap(),
        );
        let saved_tag = repo.save(tag).await.unwrap();

        let non_existent_id = TagId::new(9999).unwrap();
        let ids = vec![*saved_tag.id(), non_existent_id];

        // Act
        let result = repo.find_by_ids(&ids).await.unwrap();

        // Assert
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id(), saved_tag.id());
    }

    #[tokio::test]
    async fn test_find_by_ids_empty_array() {
        // Arrange
        let repo = InMemoryTagRepository::new();
        let ids: Vec<TagId> = vec![];

        // Act
        let result = repo.find_by_ids(&ids).await.unwrap();

        // Assert
        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn test_find_by_ids_duplicate_ids() {
        // Arrange
        let repo = InMemoryTagRepository::new();
        let tag = TagAggregate::new(
            TagName::new("タグ").unwrap(),
            TagDescription::new("説明").unwrap(),
        );
        let saved_tag = repo.save(tag).await.unwrap();

        let ids = vec![*saved_tag.id(), *saved_tag.id()];

        // Act
        let result = repo.find_by_ids(&ids).await.unwrap();

        // Assert
        // 重複は除外される
        assert_eq!(result.len(), 1);
    }
}
