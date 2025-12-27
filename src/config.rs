use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
}
