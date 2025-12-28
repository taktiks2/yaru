use crate::{
    display::format::{format_local_time, truncate_text},
    todo::Todo,
};
use comfy_table::Table;

/// Todoのテーブルを作成
pub fn create_todo_table(todos: &[Todo]) -> Table {
    let mut table = Table::new();
    table.set_header(vec![
        "ID",
        "タイトル",
        "説明",
        "ステータス",
        "優先度",
        "作成日",
        "更新日",
    ]);

    for todo in todos {
        table.add_row(vec![
            todo.id.to_string(),
            truncate_text(&todo.title, 20),
            truncate_text(&todo.description, 20),
            todo.status.to_string(),
            todo.priority.to_string(),
            format_local_time(&todo.created_at),
            format_local_time(&todo.updated_at),
        ]);
    }

    table
}

/// 単一のTodoをテーブルとして表示
pub fn create_single_todo_table(todo: &Todo) -> Table {
    let mut table = Table::new();
    table.set_header(vec![
        "ID",
        "タイトル",
        "説明",
        "ステータス",
        "優先度",
        "作成日",
        "更新日",
    ]);

    table.add_row(vec![
        todo.id.to_string(),
        truncate_text(&todo.title, 20),
        truncate_text(&todo.description, 20),
        todo.status.to_string(),
        todo.priority.to_string(),
        format_local_time(&todo.created_at),
        format_local_time(&todo.updated_at),
    ]);

    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::{Priority, Status};

    #[test]
    fn test_create_todo_table_empty() {
        let todos: Vec<Todo> = vec![];
        let table = create_todo_table(&todos);

        // ヘッダーのみ存在することを確認
        let table_str = table.to_string();
        assert!(table_str.contains("ID"));
        assert!(table_str.contains("タイトル"));
        assert!(table_str.contains("ステータス"));
    }

    #[test]
    fn test_create_todo_table_with_todos() {
        let todos = vec![
            Todo::new(1, "テストタスク1", "", Status::Pending, Priority::Medium),
            Todo::new(2, "テストタスク2", "", Status::Completed, Priority::Medium),
        ];
        let table = create_todo_table(&todos);

        let table_str = table.to_string();
        assert!(table_str.contains("1"));
        assert!(table_str.contains("テストタスク1"));
        assert!(table_str.contains("2"));
        assert!(table_str.contains("テストタスク2"));
    }

    #[test]
    fn test_create_single_todo_table() {
        let todo = Todo::new(1, "新しいタスク", "", Status::InProgress, Priority::Medium);
        let table = create_single_todo_table(&todo);

        let table_str = table.to_string();
        assert!(table_str.contains("1"));
        assert!(table_str.contains("新しいタスク"));
        assert!(table_str.contains("進行中"));
    }

    #[test]
    fn test_create_todo_table_with_different_statuses() {
        let todos = vec![
            Todo::new(1, "保留中タスク", "", Status::Pending, Priority::Medium),
            Todo::new(2, "進行中タスク", "", Status::InProgress, Priority::Medium),
            Todo::new(3, "完了タスク", "", Status::Completed, Priority::Medium),
        ];
        let table = create_todo_table(&todos);

        let table_str = table.to_string();
        assert!(table_str.contains("保留中"));
        assert!(table_str.contains("進行中"));
        assert!(table_str.contains("完了"));
    }

    #[test]
    fn test_create_todo_table_includes_description() {
        // テーブルにdescription列が含まれていることを確認
        let todos = vec![Todo::new(
            1,
            "タスク1",
            "これは説明文です",
            Status::Pending,
            Priority::Medium,
        )];
        let table = create_todo_table(&todos);

        let table_str = table.to_string();
        assert!(table_str.contains("説明"));
        assert!(table_str.contains("これは説明文です"));
    }

    #[test]
    fn test_create_todo_table_truncates_long_description() {
        // 長い説明文が切り詰められることを確認
        let long_desc = "これは非常に長い説明文です。この説明文は30文字を超えているため切り詰められるはずです。さらに長くしています。";
        let todos = vec![Todo::new(
            1,
            "タスク",
            long_desc,
            Status::Pending,
            Priority::Medium,
        )];
        let table = create_todo_table(&todos);

        let table_str = table.to_string();
        // 切り詰められた説明文が含まれている
        assert!(table_str.contains("..."));
        // 元の長い説明文がそのまま含まれていないことを確認
        assert!(!table_str.contains(long_desc));
    }
}
