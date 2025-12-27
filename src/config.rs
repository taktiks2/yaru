use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub storage: StorageConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            storage: StorageConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub todo_file: PathBuf,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            todo_file: PathBuf::from("todo.json"),
        }
    }
}

/// 指定されたパスから設定ファイルを読み込む
pub fn load_config_from_file(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path).with_context(|| {
        format!(
            "設定ファイルの読み込みに失敗しました: {}",
            path.display()
        )
    })?;
    toml::from_str(&content).context("設定ファイルのパースに失敗しました")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        // デフォルト設定が "todo.json" を使用することを確認
        let config = Config::default();
        assert_eq!(config.storage.todo_file, PathBuf::from("todo.json"));
    }

    #[test]
    fn test_config_deserialize_from_toml() {
        // TOML文字列からConfigをデシリアライズできることを確認
        let toml_str = r#"
[storage]
todo_file = "/custom/path/todo.json"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(
            config.storage.todo_file,
            PathBuf::from("/custom/path/todo.json")
        );
    }

    #[test]
    fn test_config_serialize_to_toml() {
        // ConfigをTOML文字列にシリアライズできることを確認
        let config = Config {
            storage: StorageConfig {
                todo_file: PathBuf::from("/test/path/todo.json"),
            },
        };
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("todo_file"));
        assert!(toml_str.contains("/test/path/todo.json"));
    }

    #[test]
    fn test_load_config_from_file_success() {
        use std::fs;
        use tempfile::TempDir;

        // 一時ディレクトリを作成
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("config.toml");

        // テスト用の設定ファイルを作成
        let config_content = r#"
[storage]
todo_file = "/custom/path/todos.json"
"#;
        fs::write(&config_file, config_content).unwrap();

        // ファイルから設定を読み込む
        let config = load_config_from_file(&config_file).unwrap();
        assert_eq!(
            config.storage.todo_file,
            PathBuf::from("/custom/path/todos.json")
        );
    }

    #[test]
    fn test_load_config_from_file_invalid_toml() {
        use std::fs;
        use tempfile::TempDir;

        // 一時ディレクトリを作成
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("config.toml");

        // 不正なTOMLファイルを作成
        let invalid_content = r#"
[storage
todo_file = "/custom/path/todos.json"
"#;
        fs::write(&config_file, invalid_content).unwrap();

        // エラーが返されることを確認
        let result = load_config_from_file(&config_file);
        assert!(result.is_err());
    }
}
