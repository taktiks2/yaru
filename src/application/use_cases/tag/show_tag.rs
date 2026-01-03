use crate::{
    application::dto::TagDTO,
    domain::tag::{repository::TagRepository, value_objects::TagId},
};
use anyhow::Result;
use std::sync::Arc;

/// ShowTagUseCase - タグ詳細取得のユースケース
///
/// 指定されたIDのタグの詳細を取得します。
pub struct ShowTagUseCase {
    tag_repository: Arc<dyn TagRepository>,
}

impl ShowTagUseCase {
    /// 新しいShowTagUseCaseを作成
    pub fn new(tag_repository: Arc<dyn TagRepository>) -> Self {
        Self { tag_repository }
    }

    /// タグの詳細を取得する
    ///
    /// # Arguments
    /// * `id` - 取得するタグのID
    ///
    /// # Returns
    /// * `Ok(TagDTO)` - タグの詳細
    /// * `Err` - エラーが発生した場合（タグが見つからない場合を含む）
    pub async fn execute(&self, id: i32) -> Result<TagDTO> {
        let tag_id = TagId::new(id)?;

        let tag = self
            .tag_repository
            .find_by_id(&tag_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("タグID {}は存在しません", id))?;

        Ok(TagDTO::from(tag))
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
    async fn test_show_tag_success() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag = TagAggregate::new(
            TagName::new("重要").unwrap(),
            TagDescription::new("重要なタスク用").unwrap(),
        );
        let saved_tag = tag_repo.save(tag).await.unwrap();

        let use_case = ShowTagUseCase::new(tag_repo);

        // Act
        let result = use_case.execute(saved_tag.id().value()).await;

        // Assert
        assert!(result.is_ok());
        let tag_dto = result.unwrap();
        assert_eq!(tag_dto.id, saved_tag.id().value());
        assert_eq!(tag_dto.name, "重要");
        assert_eq!(tag_dto.description, Some("重要なタスク用".to_string()));
    }

    #[tokio::test]
    async fn test_show_tag_not_found() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = ShowTagUseCase::new(tag_repo);

        // Act
        let result = use_case.execute(999).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("存在しません"));
    }

    #[tokio::test]
    async fn test_show_tag_with_invalid_id() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = ShowTagUseCase::new(tag_repo);

        // Act
        let result = use_case.execute(0).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_show_tag_without_description() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag = TagAggregate::new(
            TagName::new("緊急").unwrap(),
            TagDescription::new("").unwrap(),
        );
        let saved_tag = tag_repo.save(tag).await.unwrap();

        let use_case = ShowTagUseCase::new(tag_repo);

        // Act
        let result = use_case.execute(saved_tag.id().value()).await;

        // Assert
        assert!(result.is_ok());
        let tag_dto = result.unwrap();
        assert_eq!(tag_dto.name, "緊急");
        assert_eq!(tag_dto.description, None);
    }
}
