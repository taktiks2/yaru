use crate::repository::TaskRepository;
use anyhow::Result;

/// 指定されたIDのタスクを削除
pub fn delete_task(repo: &impl TaskRepository, id: u64) -> Result<()> {
    let mut tasks = repo.load_tasks()?;
    let initial_count = tasks.len();
    tasks.retain(|task| task.id != id);

    if initial_count == tasks.len() {
        println!("ID {} のタスクが見つかりませんでした。", id);
        return Ok(());
    }

    repo.save_tasks(&tasks)?;
    println!("タスクを削除しました。");
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_delete_task_existing_id() {
        // 存在するIDのタスクを削除する場合のテスト
        // Note: このテストは実装後に適切な形に修正する必要がある
    }

    #[test]
    fn test_delete_task_non_existing_id() {
        // 存在しないIDを指定した場合のテスト
        // エラーメッセージが表示されることを確認
    }

    #[test]
    fn test_delete_task_empty_list() {
        // 空のリストからタスクを削除しようとする場合のテスト
    }
}
