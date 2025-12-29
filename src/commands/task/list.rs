use crate::{
    cli::{Filter, FilterKey},
    display::create_task_table,
    repository::Repository,
    task::{Status, Task},
};
use anyhow::Result;

/// 全てのタスクを一覧表示
pub fn list_tasks(repo: &impl Repository<Task>, filters: Option<Vec<Filter>>) -> Result<()> {
    let mut tasks = repo.load()?;

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
            let status = Status::from_filter_value(&filter.value)
                .map_err(|_| anyhow::anyhow!("無効なステータス値です: {}", &filter.value))?;
            Ok(tasks
                .into_iter()
                .filter(|task| task.status == status)
                .collect())
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_list_tasks_empty() {
        // 空のタスクリストを表示する場合のテスト
        // Note: このテストは実装後に適切な形に修正する必要がある
        // 現在はコンパイルが通ることを確認するためのプレースホルダー
    }

    #[test]
    fn test_list_tasks_with_items() {
        // タスクが存在する場合の表示テスト
        // Note: このテストは実装後に適切な形に修正する必要がある
    }
}
