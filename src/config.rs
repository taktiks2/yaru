use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub task_file: PathBuf,
    pub tag_file: PathBuf,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            task_file: get_default_task_path().unwrap_or_else(|_| PathBuf::from("tasks.json")),
            tag_file: get_default_tag_path().unwrap_or_else(|_| PathBuf::from("tags.json")),
        }
    }
}

/// yaruの設定ディレクトリパスを取得
fn get_yaru_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME環境変数が設定されていません")?;
    Ok(PathBuf::from(home).join(".config").join("yaru"))
}

/// 設定ファイルのパスを取得
fn get_config_path() -> Result<PathBuf> {
    Ok(get_yaru_dir()?.join("config.toml"))
}

/// デフォルトのタスクファイルパスを取得
fn get_default_task_path() -> Result<PathBuf> {
    Ok(get_yaru_dir()?.join("tasks.json"))
}

/// デフォルトのタグファイルパスを取得
fn get_default_tag_path() -> Result<PathBuf> {
    Ok(get_yaru_dir()?.join("tags.json"))
}

/// 設定を読み込む
///
/// 設定ファイルが存在する場合はそれを読み込み、存在しない場合はデフォルト設定を返す
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;

    if config_path.exists() {
        load_config_from_file(&config_path)
    } else {
        Ok(Config::default())
    }
}

/// 指定されたパスから設定ファイルを読み込む
pub fn load_config_from_file(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("設定ファイルの読み込みに失敗しました: {}", path.display()))?;
    toml::from_str(&content).context("設定ファイルのパースに失敗しました")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        use std::env;

        // デフォルト設定が ~/.config/yaru/tasks.json を使用することを確認
        let config = Config::default();

        // HOME環境変数が設定されている場合は絶対パスを確認
        if let Ok(home) = env::var("HOME") {
            let expected_path = PathBuf::from(home)
                .join(".config")
                .join("yaru")
                .join("tasks.json");
            assert_eq!(config.storage.task_file, expected_path);
        } else {
            // HOME環境変数がない場合はフォールバック値を確認
            assert_eq!(config.storage.task_file, PathBuf::from("tasks.json"));
        }
    }

    #[test]
    fn test_config_deserialize_from_toml() {
        // TOML文字列からConfigをデシリアライズできることを確認
        let toml_str = r#"
[storage]
task_file = "/custom/path/tasks.json"
tag_file = "/custom/path/tags.json"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(
            config.storage.task_file,
            PathBuf::from("/custom/path/tasks.json")
        );
        assert_eq!(
            config.storage.tag_file,
            PathBuf::from("/custom/path/tags.json")
        );
    }

    #[test]
    fn test_config_serialize_to_toml() {
        // ConfigをTOML文字列にシリアライズできることを確認
        let config = Config {
            storage: StorageConfig {
                task_file: PathBuf::from("/test/path/tasks.json"),
                tag_file: PathBuf::from("/test/path/tags.json"),
            },
        };
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("task_file"));
        assert!(toml_str.contains("/test/path/tasks.json"));
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
task_file = "/custom/path/tasks.json"
tag_file = "/custom/path/tags.json"
"#;
        fs::write(&config_file, config_content).unwrap();

        // ファイルから設定を読み込む
        let config = load_config_from_file(&config_file).unwrap();
        assert_eq!(
            config.storage.task_file,
            PathBuf::from("/custom/path/tasks.json")
        );
        assert_eq!(
            config.storage.tag_file,
            PathBuf::from("/custom/path/tags.json")
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
task_file = "/custom/path/tasks.json"
"#;
        fs::write(&config_file, invalid_content).unwrap();

        // エラーが返されることを確認
        let result = load_config_from_file(&config_file);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_config_path() {
        use std::env;

        // HOME環境変数が設定されている場合、正しいパスが返されることを確認
        let home = env::var("HOME").unwrap();
        let config_path = get_config_path().unwrap();
        let expected_path = PathBuf::from(home)
            .join(".config")
            .join("yaru")
            .join("config.toml");
        assert_eq!(config_path, expected_path);
    }

    #[test]
    fn test_load_config_with_existing_file() {
        use std::fs;
        use tempfile::TempDir;

        // 一時ディレクトリを作成
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(".config").join("yaru");
        fs::create_dir_all(&config_dir).unwrap();
        let config_file = config_dir.join("config.toml");

        // テスト用の設定ファイルを作成
        let config_content = r#"
[storage]
task_file = "/existing/path/tasks.json"
tag_file = "/existing/path/tags.json"
"#;
        fs::write(&config_file, config_content).unwrap();

        // 設定ファイルが存在する場合、正しく読み込まれることを確認
        // Note: この関数は実装後にHOME環境変数を設定する必要がある
        let config = load_config_from_file(&config_file).unwrap();
        assert_eq!(
            config.storage.task_file,
            PathBuf::from("/existing/path/tasks.json")
        );
        assert_eq!(
            config.storage.tag_file,
            PathBuf::from("/existing/path/tags.json")
        );
    }

    #[test]
    fn test_load_config_with_nonexistent_file() {
        use std::env;

        // 設定ファイルが存在しない場合、デフォルト値が返されることを確認
        let config = load_config().unwrap();

        // HOME環境変数が設定されている場合は絶対パスを確認
        if let Ok(home) = env::var("HOME") {
            let expected_path = PathBuf::from(home)
                .join(".config")
                .join("yaru")
                .join("tasks.json");
            assert_eq!(config.storage.task_file, expected_path);
        } else {
            // HOME環境変数がない場合はフォールバック値を確認
            assert_eq!(config.storage.task_file, PathBuf::from("tasks.json"));
        }
    }
}
