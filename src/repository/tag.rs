use crate::{
    domain::tag::Tag,
    entity::prelude::*,
    entity::tags,
    repository::Repository,
};
use anyhow::{Context, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

/// Tag用のリポジトリ実装
pub struct TagRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> TagRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Entityからドメインモデルへ変換
    fn entity_to_domain(&self, model: tags::Model) -> Tag {
        Tag {
            id: model.id as u64,
            name: model.name,
            description: model.description,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl<'a> Repository<Tag> for TagRepository<'a> {
    async fn find_by_id(&self, id: u64) -> Result<Option<Tag>> {
        let tag_model = Tags::find_by_id(id as i32)
            .one(self.db)
            .await
            .context("タグの検索に失敗しました")?;

        Ok(tag_model.map(|m| self.entity_to_domain(m)))
    }

    async fn find_all(&self) -> Result<Vec<Tag>> {
        let tag_models = Tags::find()
            .all(self.db)
            .await
            .context("タグの読み込みに失敗しました")?;

        Ok(tag_models
            .into_iter()
            .map(|m| self.entity_to_domain(m))
            .collect())
    }

    async fn search<F>(&self, predicate: F) -> Result<Vec<Tag>>
    where
        F: Fn(&Tag) -> bool,
    {
        let tags = self.find_all().await?;
        Ok(tags.into_iter().filter(predicate).collect())
    }

    async fn create(&self, item: &Tag) -> Result<Tag> {
        let new_tag = tags::ActiveModel {
            id: sea_orm::ActiveValue::NotSet, // AUTO INCREMENT
            name: Set(item.name.clone()),
            description: Set(item.description.clone()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };

        let inserted = new_tag
            .insert(self.db)
            .await
            .context("タグの挿入に失敗しました")?;

        Ok(self.entity_to_domain(inserted))
    }

    async fn delete(&self, id: u64) -> Result<bool> {
        let result = Tags::delete_by_id(id as i32)
            .exec(self.db)
            .await
            .context("タグの削除に失敗しました")?;

        Ok(result.rows_affected > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::MigratorTrait;
    use sea_orm::Database;

    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:")
            .await
            .unwrap();
        migration::Migrator::up(&db, None).await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_find_all_empty() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        let tags = repo.find_all().await.unwrap();
        assert_eq!(tags.len(), 0);
    }

    #[tokio::test]
    async fn test_create_tag() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        let new_tag = Tag::new(0, "重要", "重要なタスク");
        let created_tag = repo.create(&new_tag).await.unwrap();

        assert!(created_tag.id > 0);
        assert_eq!(created_tag.name, "重要");
        assert_eq!(created_tag.description, "重要なタスク");
    }

    #[tokio::test]
    async fn test_find_by_id_existing() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        // タグを作成
        let new_tag = Tag::new(0, "テストタグ", "説明");
        let created_tag = repo.create(&new_tag).await.unwrap();

        // IDで検索
        let found_tag = repo.find_by_id(created_tag.id).await.unwrap();

        assert!(found_tag.is_some());
        let tag = found_tag.unwrap();
        assert_eq!(tag.id, created_tag.id);
        assert_eq!(tag.name, "テストタグ");
    }

    #[tokio::test]
    async fn test_find_by_id_not_existing() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        let found_tag = repo.find_by_id(999).await.unwrap();

        assert!(found_tag.is_none());
    }

    #[tokio::test]
    async fn test_find_all_multiple_tags() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        // 複数のタグを作成
        let tag1 = Tag::new(0, "タグ1", "説明1");
        let tag2 = Tag::new(0, "タグ2", "説明2");
        let tag3 = Tag::new(0, "タグ3", "説明3");

        repo.create(&tag1).await.unwrap();
        repo.create(&tag2).await.unwrap();
        repo.create(&tag3).await.unwrap();

        // 全件取得
        let tags = repo.find_all().await.unwrap();

        assert_eq!(tags.len(), 3);
    }

    #[tokio::test]
    async fn test_delete_existing_tag() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        // タグを作成
        let new_tag = Tag::new(0, "削除テスト", "説明");
        let created_tag = repo.create(&new_tag).await.unwrap();

        // 削除
        let deleted = repo.delete(created_tag.id).await.unwrap();

        assert!(deleted);

        // 削除されたことを確認
        let found_tag = repo.find_by_id(created_tag.id).await.unwrap();
        assert!(found_tag.is_none());
    }

    #[tokio::test]
    async fn test_delete_not_existing_tag() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        let deleted = repo.delete(999).await.unwrap();

        assert!(!deleted);
    }

    #[tokio::test]
    async fn test_search_with_predicate() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        // 複数のタグを作成
        let tag1 = Tag::new(0, "重要", "重要なタスク");
        let tag2 = Tag::new(0, "緊急", "緊急タスク");
        let tag3 = Tag::new(0, "通常", "通常タスク");

        repo.create(&tag1).await.unwrap();
        repo.create(&tag2).await.unwrap();
        repo.create(&tag3).await.unwrap();

        // "重要"を含むタグを検索
        let important_tags = repo
            .search(|tag| tag.name.contains("重要"))
            .await
            .unwrap();

        assert_eq!(important_tags.len(), 1);
        assert_eq!(important_tags[0].name, "重要");
    }
}
