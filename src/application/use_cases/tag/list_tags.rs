use crate::{application::dto::TagDTO, domain::tag::repository::TagRepository};
use anyhow::Result;
use std::sync::Arc;

/// ListTagsUseCase - タグ一覧取得のユースケース
///
/// 全タグの一覧を取得します。
pub struct ListTagsUseCase {
    tag_repository: Arc<dyn TagRepository>,
}

impl ListTagsUseCase {
    /// 新しいListTagsUseCaseを作成
    pub fn new(tag_repository: Arc<dyn TagRepository>) -> Self {
        Self { tag_repository }
    }

    /// 全タグを取得する
    ///
    /// # Returns
    /// * `Ok(Vec<TagDTO>)` - タグの一覧
    /// * `Err` - エラーが発生した場合
    pub async fn execute(&self) -> Result<Vec<TagDTO>> {
        let tags = self.tag_repository.find_all().await?;
        Ok(tags.into_iter().map(TagDTO::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tag::{
        aggregate::TagAggregate,
        value_objects::{TagDescription, TagName},
    };
    use crate::interface::persistence::in_memory::InMemoryTagRepository;

    #[tokio::test]
    async fn test_list_tags_empty() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = ListTagsUseCase::new(tag_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let tags = result.unwrap();
        assert_eq!(tags.len(), 0);
    }

    #[tokio::test]
    async fn test_list_tags_single() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag = TagAggregate::new(
            TagName::new("重要").unwrap(),
            TagDescription::new("重要なタスク用").unwrap(),
        );
        tag_repo.save(tag).await.unwrap();

        let use_case = ListTagsUseCase::new(tag_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let tags = result.unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "重要");
    }

    #[tokio::test]
    async fn test_list_tags_multiple() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag1 = TagAggregate::new(
            TagName::new("重要").unwrap(),
            TagDescription::new("重要なタスク用").unwrap(),
        );
        let tag2 = TagAggregate::new(
            TagName::new("緊急").unwrap(),
            TagDescription::new("").unwrap(),
        );
        let tag3 = TagAggregate::new(
            TagName::new("作業中").unwrap(),
            TagDescription::new("現在作業中のタスク").unwrap(),
        );

        tag_repo.save(tag1).await.unwrap();
        tag_repo.save(tag2).await.unwrap();
        tag_repo.save(tag3).await.unwrap();

        let use_case = ListTagsUseCase::new(tag_repo);

        // Act
        let result = use_case.execute().await;

        // Assert
        assert!(result.is_ok());
        let tags = result.unwrap();
        assert_eq!(tags.len(), 3);
    }
}
