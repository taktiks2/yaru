use crate::{
    application::dto::{TagDTO, UpdateTagDTO},
    domain::tag::{
        repository::TagRepository,
        value_objects::{TagDescription, TagId, TagName},
    },
};
use anyhow::Result;
use std::sync::Arc;

/// EditTagUseCase - タグ更新のユースケース
///
/// 既存のタグを部分更新します。
pub struct EditTagUseCase {
    tag_repository: Arc<dyn TagRepository>,
}

impl EditTagUseCase {
    /// 新しいEditTagUseCaseを作成
    pub fn new(tag_repository: Arc<dyn TagRepository>) -> Self {
        Self { tag_repository }
    }

    /// タグを更新する
    ///
    /// # Arguments
    /// * `id` - 更新するタグのID
    /// * `dto` - タグ更新時の入力DTO
    ///
    /// # Returns
    /// * `Ok(TagDTO)` - 更新されたタグ
    /// * `Err` - エラーが発生した場合
    pub async fn execute(&self, id: i32, dto: UpdateTagDTO) -> Result<TagDTO> {
        let tag_id = TagId::new(id)?;

        // タグを取得
        let mut tag = self
            .tag_repository
            .find_by_id(&tag_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("タグID {}は存在しません", id))?;

        // 名前の更新
        if let Some(name_str) = dto.name {
            let name = TagName::new(name_str)?;
            tag.change_name(name)?;
        }

        // 説明の更新
        if let Some(description_str) = dto.description {
            let description = TagDescription::new(description_str)?;
            tag.change_description(description)?;
        }

        // リポジトリに保存
        let updated_tag = self.tag_repository.update(tag).await?;

        // DTOに変換して返す
        Ok(TagDTO::from(updated_tag))
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
    async fn test_edit_tag_name() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag = TagAggregate::new(
            TagName::new("元の名前").unwrap(),
            TagDescription::new("説明").unwrap(),
        );
        let saved_tag = tag_repo.save(tag).await.unwrap();

        let use_case = EditTagUseCase::new(tag_repo.clone());

        let dto = UpdateTagDTO {
            name: Some("更新後の名前".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_tag.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_tag = result.unwrap();
        assert_eq!(updated_tag.name, "更新後の名前");
        assert_eq!(updated_tag.description, Some("説明".to_string())); // 変更なし
    }

    #[tokio::test]
    async fn test_edit_tag_description() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag = TagAggregate::new(
            TagName::new("タグ名").unwrap(),
            TagDescription::new("元の説明").unwrap(),
        );
        let saved_tag = tag_repo.save(tag).await.unwrap();

        let use_case = EditTagUseCase::new(tag_repo.clone());

        let dto = UpdateTagDTO {
            description: Some("更新後の説明".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_tag.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_tag = result.unwrap();
        assert_eq!(updated_tag.name, "タグ名"); // 変更なし
        assert_eq!(updated_tag.description, Some("更新後の説明".to_string()));
    }

    #[tokio::test]
    async fn test_edit_tag_both_fields() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag = TagAggregate::new(
            TagName::new("元の名前").unwrap(),
            TagDescription::new("元の説明").unwrap(),
        );
        let saved_tag = tag_repo.save(tag).await.unwrap();

        let use_case = EditTagUseCase::new(tag_repo.clone());

        let dto = UpdateTagDTO {
            name: Some("新しい名前".to_string()),
            description: Some("新しい説明".to_string()),
        };

        // Act
        let result = use_case.execute(saved_tag.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_tag = result.unwrap();
        assert_eq!(updated_tag.name, "新しい名前");
        assert_eq!(updated_tag.description, Some("新しい説明".to_string()));
    }

    #[tokio::test]
    async fn test_edit_tag_not_found() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = EditTagUseCase::new(tag_repo);

        let dto = UpdateTagDTO {
            name: Some("新しい名前".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(999, dto).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("存在しません"));
    }

    #[tokio::test]
    async fn test_edit_tag_with_invalid_id() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = EditTagUseCase::new(tag_repo);

        let dto = UpdateTagDTO {
            name: Some("新しい名前".to_string()),
            ..Default::default()
        };

        // Act
        let result = use_case.execute(0, dto).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_tag_with_empty_name() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag = TagAggregate::new(
            TagName::new("タグ名").unwrap(),
            TagDescription::new("").unwrap(),
        );
        let saved_tag = tag_repo.save(tag).await.unwrap();

        let use_case = EditTagUseCase::new(tag_repo);

        let dto = UpdateTagDTO {
            name: Some("".to_string()), // 空の名前
            ..Default::default()
        };

        // Act
        let result = use_case.execute(saved_tag.id().value(), dto).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_tag_empty_update() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag = TagAggregate::new(
            TagName::new("タグ名").unwrap(),
            TagDescription::new("説明").unwrap(),
        );
        let saved_tag = tag_repo.save(tag).await.unwrap();

        let use_case = EditTagUseCase::new(tag_repo.clone());

        let dto = UpdateTagDTO::default(); // 何も更新しない

        // Act
        let result = use_case.execute(saved_tag.id().value(), dto).await;

        // Assert
        assert!(result.is_ok());
        let updated_tag = result.unwrap();
        // 変更なし
        assert_eq!(updated_tag.name, "タグ名");
        assert_eq!(updated_tag.description, Some("説明".to_string()));
    }
}
