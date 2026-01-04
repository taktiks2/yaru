use crate::{
    application::dto::{CreateTagDTO, TagDTO},
    domain::tag::{
        aggregate::TagAggregate,
        repository::TagRepository,
        value_objects::{TagDescription, TagName},
    },
};
use anyhow::Result;
use std::sync::Arc;

/// AddTagUseCase - タグ作成のユースケース
///
/// 新しいタグを作成します。
pub struct AddTagUseCase {
    tag_repository: Arc<dyn TagRepository>,
}

impl AddTagUseCase {
    /// 新しいAddTagUseCaseを作成
    pub fn new(tag_repository: Arc<dyn TagRepository>) -> Self {
        Self { tag_repository }
    }

    /// タグを作成する
    ///
    /// # Arguments
    /// * `dto` - タグ作成時の入力DTO
    ///
    /// # Returns
    /// * `Ok(TagDTO)` - 作成されたタグ
    /// * `Err` - エラーが発生した場合
    pub async fn execute(&self, dto: CreateTagDTO) -> Result<TagDTO> {
        // Value Objectsを作成
        let name = TagName::new(dto.name)?;
        let description = TagDescription::new(dto.description.unwrap_or_default())?;

        // Aggregateを作成
        let tag = TagAggregate::new(name, description);

        // リポジトリに保存
        let saved_tag = self.tag_repository.save(tag).await?;

        // DTOに変換して返す
        Ok(TagDTO::from(saved_tag))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::persistence::in_memory::InMemoryTagRepository;

    #[tokio::test]
    async fn test_add_tag_with_description() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = AddTagUseCase::new(tag_repo);

        let dto = CreateTagDTO {
            name: "重要".to_string(),
            description: Some("重要なタスク用".to_string()),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.name, "重要");
        assert_eq!(tag.description, Some("重要なタスク用".to_string()));
    }

    #[tokio::test]
    async fn test_add_tag_without_description() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = AddTagUseCase::new(tag_repo);

        let dto = CreateTagDTO {
            name: "緊急".to_string(),
            description: None,
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.name, "緊急");
        assert_eq!(tag.description, None);
    }

    #[tokio::test]
    async fn test_add_tag_with_empty_name() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = AddTagUseCase::new(tag_repo);

        let dto = CreateTagDTO {
            name: "".to_string(),
            description: None,
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_tag_with_whitespace_only_name() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = AddTagUseCase::new(tag_repo);

        let dto = CreateTagDTO {
            name: "   ".to_string(),
            description: None,
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_tag_with_too_long_name() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = AddTagUseCase::new(tag_repo);

        // TagNameは最大50文字なので51文字のタグ名を試す
        let long_name = "あ".repeat(51);
        let dto = CreateTagDTO {
            name: long_name,
            description: None,
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
    }
}
