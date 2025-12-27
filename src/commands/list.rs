use crate::{
    cli::{Filter, FilterKey},
    display::create_todo_table,
    error::YaruError,
    repository::{JsonTodoRepository, TodoRepository},
    todo::{Status, Todo},
};

/// 全てのTodoを一覧表示
pub fn list_todos(filters: Option<Vec<Filter>>) -> Result<(), YaruError> {
    let repo = JsonTodoRepository::default();
    let mut todos = repo.load_todos()?;

    // フィルタリングを適用
    if let Some(filters) = filters {
        for filter in filters {
            todos = apply_filter(todos, &filter)?;
        }
    }

    if todos.is_empty() {
        println!("タスクはありません。");
        return Ok(());
    }

    let table = create_todo_table(&todos);
    println!("{table}");

    Ok(())
}

/// フィルタを適用してTodoリストを絞り込む
fn apply_filter(todos: Vec<Todo>, filter: &Filter) -> Result<Vec<Todo>, YaruError> {
    match filter.key {
        FilterKey::Status => {
            let status = Status::from_filter_value(&filter.value)
                .map_err(|e| YaruError::InvalidInput(e))?;
            Ok(todos.into_iter().filter(|todo| todo.status == status).collect())
        }
    }
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
