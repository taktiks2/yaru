use crate::{
    cli::{Filter, FilterKey},
    display::create_task_table,
    repository::task::TaskRepository,
    repository::Repository,
    task::{Status, Task},
};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// 全てのタスクを一覧表示
pub async fn list_tasks(
    db: &DatabaseConnection,
    filters: Option<Vec<Filter>>,
) -> Result<()> {
    // リポジトリから全タスクを取得
    let task_repo = TaskRepository::new(db);
    let mut tasks = task_repo.find_all().await?;

    // フィルタリングを適用
    if let Some(filters) = filters {
        for filter in filters {
            tasks = apply_filter(tasks, &filter)?;
        }
    }

    if tasks.is_empty() {
        println!("タスクはありません。");
        return Ok(());
    }

    let table = create_task_table(&tasks);
    println!("{table}");

    Ok(())
}

/// フィルタを適用してタスクリストを絞り込む
fn apply_filter(tasks: Vec<Task>, filter: &Filter) -> Result<Vec<Task>> {
    match filter.key {
        FilterKey::Status => {
            let status = match Status::from_filter_value(&filter.value) {
                Ok(s) => s,
                Err(_) => anyhow::bail!("無効なステータス値です: {}", &filter.value),
            };
            Ok(tasks
                .into_iter()
                .filter(|task| task.status == status)
                .collect())
        }
    }
}
