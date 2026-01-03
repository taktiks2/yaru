use anyhow::Result;
use std::sync::Arc;

use crate::domain::tag::{repository::TagRepository, value_objects::TagId};

/// DeleteTagUseCase - タグ削除のユースケース
///
/// 指定されたIDのタグを削除します。
pub struct DeleteTagUseCase {
    tag_repository: Arc<dyn TagRepository>,
}

impl DeleteTagUseCase {
    /// 新しいDeleteTagUseCaseを作成
    pub fn new(tag_repository: Arc<dyn TagRepository>) -> Self {
        Self { tag_repository }
    }

    /// タグを削除する
    ///
    /// # Arguments
    /// * `id` - 削除するタグのID
    ///
    /// # Returns
    /// * `Ok(())` - 削除成功
    /// * `Err` - エラーが発生した場合（タグが見つからない場合を含む）
    pub async fn execute(&self, id: i32) -> Result<()> {
        let tag_id = TagId::new(id)?;

        // タグの存在確認
        if self.tag_repository.find_by_id(&tag_id).await?.is_none() {
            anyhow::bail!("タグID {}は存在しません", id);
        }

        // タグを削除
        self.tag_repository.delete(&tag_id).await?;

        Ok(())
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
    async fn test_delete_tag_success() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag = TagAggregate::new(
            TagName::new("削除対象").unwrap(),
            TagDescription::new("").unwrap(),
        );
        let saved_tag = tag_repo.save(tag).await.unwrap();

        let use_case = DeleteTagUseCase::new(tag_repo.clone());

        // Act
        let result = use_case.execute(saved_tag.id().value()).await;

        // Assert
        assert!(result.is_ok());

        // 削除されたことを確認
        let found = tag_repo.find_by_id(saved_tag.id()).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_delete_tag_not_found() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = DeleteTagUseCase::new(tag_repo);

        // Act
        let result = use_case.execute(999).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("存在しません"));
    }

    #[tokio::test]
    async fn test_delete_tag_with_invalid_id() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = DeleteTagUseCase::new(tag_repo);

        // Act
        let result = use_case.execute(0).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_multiple_tags() {
        // Arrange
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let tag1 = TagAggregate::new(
            TagName::new("タグ1").unwrap(),
            TagDescription::new("").unwrap(),
        );
        let tag2 = TagAggregate::new(
            TagName::new("タグ2").unwrap(),
            TagDescription::new("").unwrap(),
        );

        let saved_tag1 = tag_repo.save(tag1).await.unwrap();
        let saved_tag2 = tag_repo.save(tag2).await.unwrap();

        let use_case = DeleteTagUseCase::new(tag_repo.clone());

        // Act - 最初のタグを削除
        let result1 = use_case.execute(saved_tag1.id().value()).await;

        // Assert
        assert!(result1.is_ok());

        // 2つ目のタグがまだ存在することを確認
        let found = tag_repo.find_by_id(saved_tag2.id()).await.unwrap();
        assert!(found.is_some());

        // Act - 2つ目のタグを削除
        let result2 = use_case.execute(saved_tag2.id().value()).await;

        // Assert
        assert!(result2.is_ok());

        // 両方削除されたことを確認
        let all_tags = tag_repo.find_all().await.unwrap();
        assert_eq!(all_tags.len(), 0);
    }
}
