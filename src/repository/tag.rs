use crate::{domain::tag::Tag, repository::Repository};
use anyhow::{Context, Result};
use entity::{prelude::*, tags};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, EntityTrait, IntoActiveModel, Set,
};

/// Tag用のリポジトリ実装
pub struct TagRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> TagRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }
}

impl<'a> Repository<Tag> for TagRepository<'a> {
    async fn find_by_id(&self, id: i32) -> Result<Option<Tag>> {
        let tag_model = Tags::find_by_id(id)
            .one(self.db)
            .await
            .context("タグの検索に失敗しました")?;

        Ok(tag_model.map(Into::into))
    }

    async fn find_all(&self) -> Result<Vec<Tag>> {
        let tag_models = Tags::find()
            .all(self.db)
            .await
            .context("タグの読み込みに失敗しました")?;

        Ok(tag_models.into_iter().map(Into::into).collect())
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
            id: NotSet,
            name: Set(item.name.clone()),
            description: Set(item.description.clone()),
            created_at: NotSet,
            updated_at: NotSet,
        };

        let inserted = new_tag
            .insert(self.db)
            .await
            .context("タグの挿入に失敗しました")?;

        Ok(inserted.into())
    }

    async fn delete(&self, id: i32) -> Result<bool> {
        let result = Tags::delete_by_id(id)
            .exec(self.db)
            .await
            .context("タグの削除に失敗しました")?;

        Ok(result.rows_affected > 0)
    }

    async fn update(&self, item: &Tag) -> Result<Tag> {
        // タグが存在するか確認
        let current_tag = Tags::find_by_id(item.id)
            .one(self.db)
            .await
            .context("タグの取得に失敗しました")? // Resultのエラー処理
            .context("タグが存在しません")?; // OptionのNone処理

        // ActiveModelに変換して更新
        let mut active_model = current_tag.into_active_model();
        active_model.name = Set(item.name.clone());
        active_model.description = Set(item.description.clone());

        let updated = active_model
            .update(self.db)
            .await
            .context("タグの更新に失敗しました")?;

        Ok(updated.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::MigratorTrait;
    use sea_orm::Database;

    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:").await.unwrap();
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
        let important_tags = repo.search(|tag| tag.name.contains("重要")).await.unwrap();

        assert_eq!(important_tags.len(), 1);
        assert_eq!(important_tags[0].name, "重要");
    }

    #[tokio::test]
    async fn test_update_tag_basic() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        // タグを作成
        let new_tag = Tag::new(0, "元の名前", "元の説明");
        let created_tag = repo.create(&new_tag).await.unwrap();

        // タグを更新
        let updated_tag = Tag {
            id: created_tag.id,
            name: "新しい名前".to_string(),
            description: "新しい説明".to_string(),
            created_at: created_tag.created_at,
            updated_at: created_tag.updated_at,
        };

        let result = repo.update(&updated_tag).await.unwrap();

        // 更新内容を検証
        assert_eq!(result.id, created_tag.id);
        assert_eq!(result.name, "新しい名前");
        assert_eq!(result.description, "新しい説明");
        assert_eq!(result.created_at, created_tag.created_at); // 作成日時は変わらない
    }

    #[tokio::test]
    async fn test_update_tag_updated_at_changes() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        // タグを作成
        let new_tag = Tag::new(0, "テストタグ", "説明");
        let created_tag = repo.create(&new_tag).await.unwrap();
        println!("作成時のupdated_at: {:?}", created_tag.updated_at);

        // 少し待機してからタグを更新
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let updated_tag = Tag {
            id: created_tag.id,
            name: "更新後の名前".to_string(),
            description: created_tag.description.clone(),
            created_at: created_tag.created_at,
            updated_at: created_tag.updated_at,
        };

        repo.update(&updated_tag).await.unwrap();

        // AFTER UPDATEトリガーによる変更を確認するため、再度取得
        let result = repo.find_by_id(created_tag.id).await.unwrap().unwrap();
        println!("更新後のupdated_at: {:?}", result.updated_at);

        // updated_atが更新されたことを確認
        assert!(
            result.updated_at > created_tag.updated_at,
            "updated_atが更新されていません: created={:?}, updated={:?}",
            created_tag.updated_at,
            result.updated_at
        );
    }

    #[tokio::test]
    async fn test_update_tag_not_existing() {
        let db = setup_test_db().await;
        let repo = TagRepository::new(&db);

        // 存在しないタグを更新しようとする
        let non_existing_tag = Tag::new(999, "名前", "説明");

        let result = repo.update(&non_existing_tag).await;

        // エラーが返されることを確認
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("タグが存在しません")
        );
    }
}
