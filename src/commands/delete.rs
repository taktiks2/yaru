use crate::repository::TodoRepository;
use anyhow::Result;

/// 指定されたIDのTodoを削除
pub fn delete_todo(repo: &impl TodoRepository, id: u64) -> Result<()> {
    let todos = repo.load_todos()?;
    let initial_count = todos.len();
    let filtered_todos: Vec<_> = todos.into_iter().filter(|todo| todo.id != id).collect();

    if initial_count == filtered_todos.len() {
        println!("ID {} のタスクが見つかりませんでした。", id);
        return Ok(());
    }

    repo.save_todos(&filtered_todos)?;
    println!("タスクを削除しました。");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_todo_existing_id() {
        // 存在するIDのTodoを削除する場合のテスト
        // Note: このテストは実装後に適切な形に修正する必要がある
    }

    #[test]
    fn test_delete_todo_non_existing_id() {
        // 存在しないIDを指定した場合のテスト
        // エラーメッセージが表示されることを確認
    }

    #[test]
    fn test_delete_todo_empty_list() {
        // 空のリストからTodoを削除しようとする場合のテスト
    }
}
