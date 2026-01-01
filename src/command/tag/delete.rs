use crate::repository::{Repository, tag::TagRepository};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// 指定されたIDのタグを削除
///
/// データベースの外部キー制約（ON DELETE RESTRICT）により、
/// 使用中のタグは自動的に削除が拒否されます。
pub async fn delete_tag(db: &DatabaseConnection, id: i32) -> Result<()> {
    // リポジトリを使用して削除
    let tag_repo = TagRepository::new(db);
    let deleted = tag_repo.delete(id).await.map_err(|e| {
        // SQLiteの外部キー制約エラーを検出
        if e.to_string().contains("FOREIGN KEY constraint failed") {
            anyhow::anyhow!("このタグは使用中のため削除できません")
        } else {
            e
        }
    })?;

    if !deleted {
        anyhow::bail!("ID {id} のタグが見つかりません");
    }

    println!("タグを削除しました。");
    Ok(())
}
