use crate::{
    cli::{Filter, FilterKey},
    display::create_task_table,
    domain::task::{Status, Task},
    repository::Repository,
    repository::task::TaskRepository,
};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// 全てのタスクを一覧表示
pub async fn list_tasks(db: &DatabaseConnection, filters: Option<Vec<Filter>>) -> Result<()> {
    let task_repo = TaskRepository::new(db);
    
    // フィルタがある場合はsearchを使用、ない場合はfind_allを使用
    let tasks = if let Some(filters) = filters {
        task_repo.search(|task| {
            filters.iter().all(|filter| match_filter(task, filter))
        }).await?
    } else {
        task_repo.find_all().await?
    };

    if tasks.is_empty() {
        println!("タスクはありません。");
        return Ok(());
    }

    let table = create_task_table(&tasks);
    println!("{table}");

    Ok(())
}

/// タスクがフィルタ条件にマッチするか判定
fn match_filter(task: &Task, filter: &Filter) -> bool {
    match filter.key {
        FilterKey::Status => {
            if let Ok(status) = Status::from_filter_value(&filter.value) {
                task.status == status
            } else {
                false
            }
        }
    }
}
