use crate::{
    json::{load_json, save_json},
    todo::Todo,
};
use anyhow::Result;
use std::path::PathBuf;

/// Todoリポジトリのトレイト
/// データの永続化方法を抽象化し、異なる実装（JSON、SQLiteなど）を切り替え可能にする
pub trait TodoRepository {
    /// Todoリストを読み込む
    fn load_todos(&self) -> Result<Vec<Todo>>;

    /// Todoリストを保存する
    fn save_todos(&self, todos: &[Todo]) -> Result<()>;

    /// 次のIDを取得する
    fn find_next_id(&self, todos: &[Todo]) -> u64 {
        todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1
    }

    /// データファイルが存在することを確認（必要に応じて初期化）
    fn ensure_data_exists(&self) -> Result<()>;
}

/// JSON形式でTodoを保存するリポジトリ実装
pub struct JsonTodoRepository {
    file_path: PathBuf,
}

impl JsonTodoRepository {
    /// 新しいJsonTodoRepositoryインスタンスを作成
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
}

impl TodoRepository for JsonTodoRepository {
    fn load_todos(&self) -> Result<Vec<Todo>> {
        load_json(&self.file_path)
    }

    fn save_todos(&self, todos: &[Todo]) -> Result<()> {
        save_json(&self.file_path, todos)
    }

    fn ensure_data_exists(&self) -> Result<()> {
        if !self.file_path.exists() {
            save_json(&self.file_path, &Vec::<Todo>::new())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::{Priority, Status};
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_ensure_data_exists_creates_file() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("todo.json");

        // ファイルが存在しないことを確認
        assert!(!test_file.exists());

        // リポジトリを作成してファイルを初期化
        let repo = JsonTodoRepository::new(&test_file);
        repo.ensure_data_exists().unwrap();

        // ファイルが作成されたことを確認
        assert!(test_file.exists());

        // 空のJSONリストが保存されていることを確認
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "[]");
    }

    #[test]
    fn test_load_todos_empty_list() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("todo.json");

        // 空のJSONファイルを作成
        fs::write(&test_file, "[]").unwrap();

        // リポジトリを使って読み込み
        let repo = JsonTodoRepository::new(&test_file);
        let todos = repo.load_todos().unwrap();

        // 空のリストが返されることを確認
        assert_eq!(todos.len(), 0);
    }

    #[test]
    fn test_save_and_load_todos() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("todo.json");

        let todos = vec![
            Todo::new(1, "テストタスク1", "", Status::Pending, Priority::Medium),
            Todo::new(2, "テストタスク2", "", Status::Completed, Priority::Medium),
        ];

        // リポジトリを使って保存
        let repo = JsonTodoRepository::new(&test_file);
        repo.save_todos(&todos).unwrap();

        // ファイルが作成されたことを確認
        assert!(test_file.exists());

        // 保存したTodoを読み込み
        let loaded_todos = repo.load_todos().unwrap();

        // 正しく保存・読み込みできたことを確認
        assert_eq!(loaded_todos.len(), 2);
        assert_eq!(loaded_todos[0].id, 1);
        assert_eq!(loaded_todos[0].title, "テストタスク1");
        assert_eq!(loaded_todos[1].id, 2);
        assert_eq!(loaded_todos[1].title, "テストタスク2");
    }

    #[test]
    fn test_find_next_id_empty_list() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("todo.json");
        let repo = JsonTodoRepository::new(&test_file);

        let todos: Vec<Todo> = vec![];
        let next_id = repo.find_next_id(&todos);
        assert_eq!(next_id, 1);
    }

    #[test]
    fn test_find_next_id_with_existing_todos() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("todo.json");
        let repo = JsonTodoRepository::new(&test_file);

        let todos = vec![
            Todo::new(1, "タスク1", "", Status::Pending, Priority::Medium),
            Todo::new(3, "タスク3", "", Status::Pending, Priority::Medium),
            Todo::new(2, "タスク2", "", Status::Pending, Priority::Medium),
        ];
        let next_id = repo.find_next_id(&todos);
        assert_eq!(next_id, 4);
    }

    #[test]
    fn test_find_next_id_with_single_todo() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("todo.json");
        let repo = JsonTodoRepository::new(&test_file);

        let todos = vec![Todo::new(
            5,
            "タスク",
            "",
            Status::Pending,
            Priority::Medium,
        )];
        let next_id = repo.find_next_id(&todos);
        assert_eq!(next_id, 6);
    }

    #[test]
    fn test_repository_trait_with_custom_path() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("custom_todos.json");

        // カスタムパスでリポジトリを作成
        let repo = JsonTodoRepository::new(&test_file);

        // データが存在することを確認
        repo.ensure_data_exists().unwrap();
        assert!(test_file.exists());

        // Todoを保存
        let todos = vec![Todo::new(
            1,
            "カスタムパステスト",
            "",
            Status::Pending,
            Priority::Medium,
        )];
        repo.save_todos(&todos).unwrap();

        // Todoを読み込み
        let loaded_todos = repo.load_todos().unwrap();
        assert_eq!(loaded_todos.len(), 1);
        assert_eq!(loaded_todos[0].title, "カスタムパステスト");
    }
}
