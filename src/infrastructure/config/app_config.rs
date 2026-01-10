use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub database_url: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            database_url: get_default_database_url()
                .unwrap_or_else(|_| "sqlite://yaru.db?mode=rwc".to_string()),
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

/// デフォルトのデータベースURLを取得
fn get_default_database_url() -> Result<String> {
    let db_path = get_yaru_dir()?.join("yaru.db");
    Ok(format!("sqlite://{}?mode=rwc", db_path.display()))
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
        .with_context(|| format!("Failed to load config file: {}", path.display()))?;
    toml::from_str(&content).context("Failed to parse config file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        use std::env;

        // デフォルト設定が ~/.config/yaru/yaru.db を使用することを確認
        let config = Config::default();

        // HOME環境変数が設定されている場合は絶対パスを確認
        if let Ok(home) = env::var("HOME") {
            let expected_path = PathBuf::from(home)
                .join(".config")
                .join("yaru")
                .join("yaru.db");
            let expected_url = format!("sqlite://{}?mode=rwc", expected_path.display());
            assert_eq!(config.storage.database_url, expected_url);
        } else {
            // HOME環境変数がない場合はフォールバック値を確認
            assert_eq!(config.storage.database_url, "sqlite://yaru.db?mode=rwc");
        }
    }

    #[test]
    fn test_config_deserialize_from_toml() {
        // TOML文字列からConfigをデシリアライズできることを確認
        let toml_str = r#"
[storage]
database_url = "sqlite:///custom/path/yaru.db?mode=rwc"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(
            config.storage.database_url,
            "sqlite:///custom/path/yaru.db?mode=rwc"
        );
    }

    #[test]
    fn test_config_serialize_to_toml() {
        // ConfigをTOML文字列にシリアライズできることを確認
        let config = Config {
            storage: StorageConfig {
                database_url: "sqlite://test.db?mode=rwc".to_string(),
            },
        };
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("database_url"));
        assert!(toml_str.contains("sqlite://test.db?mode=rwc"));
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
database_url = "sqlite:///custom/path/yaru.db?mode=rwc"
"#;
        fs::write(&config_file, config_content).unwrap();

        // ファイルから設定を読み込む
        let config = load_config_from_file(&config_file).unwrap();
        assert_eq!(
            config.storage.database_url,
            "sqlite:///custom/path/yaru.db?mode=rwc"
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
database_url = "sqlite://test.db?mode=rwc"
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
database_url = "sqlite:///existing/path/yaru.db?mode=rwc"
"#;
        fs::write(&config_file, config_content).unwrap();

        // 設定ファイルが存在する場合、正しく読み込まれることを確認
        let config = load_config_from_file(&config_file).unwrap();
        assert_eq!(
            config.storage.database_url,
            "sqlite:///existing/path/yaru.db?mode=rwc"
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
                .join("yaru.db");
            let expected_url = format!("sqlite://{}?mode=rwc", expected_path.display());
            assert_eq!(config.storage.database_url, expected_url);
        } else {
            // HOME環境変数がない場合はフォールバック値を確認
            assert_eq!(config.storage.database_url, "sqlite://yaru.db?mode=rwc");
        }
    }
}
