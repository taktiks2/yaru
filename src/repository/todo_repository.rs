use crate::{
    error::YaruError,
    json::{load_json, save_json},
    todo::Todo,
};
use std::path::Path;

const PATH_TO_JSON: &str = "todo.json";

/// データファイルが存在しない場合は空のJSONファイルを作成
pub fn ensure_data_file_exists() -> Result<(), YaruError> {
    if !Path::new(PATH_TO_JSON).exists() {
        save_json(PATH_TO_JSON, &Vec::<Todo>::new())?;
    }
    Ok(())
}

/// Todoリストを読み込む
pub fn load_todos() -> Result<Vec<Todo>, YaruError> {
    load_json(PATH_TO_JSON)
}

/// Todoリストを保存する
pub fn save_todos(todos: &[Todo]) -> Result<(), YaruError> {
    save_json(PATH_TO_JSON, todos)
}

/// 次のIDを取得する
pub fn find_next_id(todos: &[Todo]) -> u64 {
    todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::Status;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn setup_test_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_ensure_data_file_exists_creates_file() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("todo.json");

        // ファイルが存在しないことを確認
        assert!(!test_file.exists());

        // TODO: PATH_TO_JSONをテスト用のパスに変更する必要がある
        // この実装は後で改善する
    }

    #[test]
    fn test_load_todos_empty_list() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("todo.json");

        // 空のJSONファイルを作成
        fs::write(&test_file, "[]").unwrap();

        // TODO: PATH_TO_JSONをテスト用のパスに変更する必要がある
    }

    #[test]
    fn test_save_todos_creates_valid_json() {
        let test_dir = setup_test_dir();

        let todos = vec![
            Todo::new(1, "テストタスク1", Status::Pending),
            Todo::new(2, "テストタスク2", Status::Completed),
        ];

        // TODO: PATH_TO_JSONをテスト用のパスに変更する必要がある
    }

    #[test]
    fn test_find_next_id_empty_list() {
        let todos: Vec<Todo> = vec![];
        let next_id = find_next_id(&todos);
        assert_eq!(next_id, 1);
    }

    #[test]
    fn test_find_next_id_with_existing_todos() {
        let todos = vec![
            Todo::new(1, "タスク1", Status::Pending),
            Todo::new(3, "タスク3", Status::Pending),
            Todo::new(2, "タスク2", Status::Pending),
        ];
        let next_id = find_next_id(&todos);
        assert_eq!(next_id, 4);
    }

    #[test]
    fn test_find_next_id_with_single_todo() {
        let todos = vec![Todo::new(5, "タスク", Status::Pending)];
        let next_id = find_next_id(&todos);
        assert_eq!(next_id, 6);
    }
}
