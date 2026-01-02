use crate::repository::{Repository, task::TaskRepository};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// タスク削除のパラメータ
pub struct DeleteTaskParams {
    pub id: i32,
}

/// 指定されたIDのタスクを削除
pub async fn delete_task(db: &DatabaseConnection, params: DeleteTaskParams) -> Result<()> {
    // リポジトリを使用して削除
    let task_repo = TaskRepository::new(db);
    let deleted = task_repo.delete(params.id).await?;

    if !deleted {
        anyhow::bail!("ID {} のタスクが見つかりません", params.id);
    }

    println!("タスクを削除しました。");
    Ok(())
}
