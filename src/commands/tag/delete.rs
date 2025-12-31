use crate::repository::{Repository, tag::TagRepository, task::TaskRepository};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// 指定されたIDのタグを削除
pub async fn delete_tag(db: &DatabaseConnection, id: i32) -> Result<()> {
    // 参照整合性チェック
    let task_repo = TaskRepository::new(db);
    let referenced_tasks = task_repo.search(|task| task.tags.contains(&id)).await?;

    if !referenced_tasks.is_empty() {
        anyhow::bail!(
            "このタグは {} 個のタスクで使用されているため削除できません。",
            referenced_tasks.len()
        );
    }

    // リポジトリを使用して削除
    let tag_repo = TagRepository::new(db);
    let deleted = tag_repo.delete(id).await?;

    if !deleted {
        anyhow::bail!("ID {} のタグが見つかりません", id);
    }

    println!("タグを削除しました。");
    Ok(())
}
