use crate::repository::{task::TaskRepository, Repository};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// 指定されたIDのタスクを削除
pub async fn delete_task(db: &DatabaseConnection, id: i32) -> Result<()> {
    // リポジトリを使用して削除
    let task_repo = TaskRepository::new(db);
    let deleted = task_repo.delete(id).await?;

    if !deleted {
        anyhow::bail!("ID {id} のタスクが見つかりません");
    }

    println!("タスクを削除しました。");
    Ok(())
}
