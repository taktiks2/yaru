use crate::{
    display::create_task_detail_table,
    repository::{tag::TagRepository, task::TaskRepository, Repository},
};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// 指定されたIDのタスク詳細を表示
pub async fn show_task(db: &DatabaseConnection, id: u64) -> Result<()> {
    // リポジトリからタスクを検索
    let task_repo = TaskRepository::new(db);
    let Some(task) = task_repo.find_by_id(id).await? else {
        anyhow::bail!("ID {} のタスクが見つかりません", id);
    };

    // 全タグを取得して表示
    let tag_repo = TagRepository::new(db);
    let tags = tag_repo.find_all().await?;

    let table = create_task_detail_table(&task, &tags);
    println!("{table}");

    Ok(())
}

