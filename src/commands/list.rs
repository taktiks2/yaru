use crate::{
    display::create_todo_table,
    error::YaruError,
    repository::{JsonTodoRepository, TodoRepository},
};

/// 全てのTodoを一覧表示
pub fn list_todos() -> Result<(), YaruError> {
    let repo = JsonTodoRepository::default();
    let todos = repo.load_todos()?;

    if todos.is_empty() {
        println!("タスクはありません。");
        return Ok(());
    }

    let table = create_todo_table(&todos);
    println!("{table}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_todos_empty() {
        // 空のTodoリストを表示する場合のテスト
        // Note: このテストは実装後に適切な形に修正する必要がある
        // 現在はコンパイルが通ることを確認するためのプレースホルダー
    }

    #[test]
    fn test_list_todos_with_items() {
        // Todoが存在する場合の表示テスト
        // Note: このテストは実装後に適切な形に修正する必要がある
    }
}
