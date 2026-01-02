use crate::{
    display::create_task_detail_table,
    repository::{Repository, task::TaskRepository},
};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// タスク詳細表示のパラメータ
pub struct ShowTaskParams {
    pub id: i32,
}

/// 指定されたIDのタスク詳細を表示
pub async fn show_task(db: &DatabaseConnection, params: ShowTaskParams) -> Result<()> {
    // リポジトリからタスクを検索
    let task_repo = TaskRepository::new(db);
    let Some(task) = task_repo.find_by_id(params.id).await? else {
        anyhow::bail!("ID {} のタスクが見つかりません", params.id);
    };

    // all_tagsの取得と渡しを削除
    let table = create_task_detail_table(&task);
    println!("{table}");

    Ok(())
}
