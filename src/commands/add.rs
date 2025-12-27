use crate::{
    display::create_single_todo_table,
    repository::TodoRepository,
    todo::{Status, Todo},
};
use anyhow::{Context, Result};
use inquire::Text;

/// 新しいTodoを追加
pub fn add_todo(
    repo: &impl TodoRepository,
    title: Option<String>,
    status: Option<Status>,
) -> Result<()> {
    let title = match title {
        Some(t) => t,
        None => Text::new("タスクのタイトルを入力してください")
            .prompt()
            .context("タスクのタイトルの入力に失敗しました")?,
    };

    let status = status.unwrap_or(Status::Pending);

    let mut todos = repo.load_todos()?;
    let new_id = repo.find_next_id(&todos);
    let new_todo = Todo::new(new_id, &title, status);

    todos.push(new_todo.clone());
    repo.save_todos(&todos)?;

    println!("タスクを登録しました。");

    let table = create_single_todo_table(&new_todo);
    println!("{table}");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add_todo_with_title_and_status() {
        // タイトルとステータスを指定してTodoを追加する場合のテスト
        // Note: このテストは実装後に適切な形に修正する必要がある
    }

    #[test]
    fn test_add_todo_with_only_title() {
        // タイトルのみを指定してTodoを追加する場合のテスト
        // デフォルトステータスがPendingになることを確認
    }

    #[test]
    fn test_add_todo_without_title() {
        // タイトルなしで追加する場合のテスト
        // 対話的入力が必要になるため、統合テストで実装する
    }
}
