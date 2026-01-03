use anyhow::{Context, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

/// データベース接続マネージャー
///
/// データベース接続の初期化と管理を行います。
pub struct DatabaseConnectionManager;

impl DatabaseConnectionManager {
    /// データベース接続を作成
    ///
    /// # 引数
    /// - `database_url`: データベース接続URL
    ///
    /// # 戻り値
    /// データベース接続オブジェクト
    pub async fn connect(database_url: &str) -> Result<DatabaseConnection> {
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false); // SQLログを無効化（必要に応じて有効化）

        Database::connect(opt)
            .await
            .context("データベースへの接続に失敗しました")
    }

    /// 設定からデータベース接続を作成
    ///
    /// # 引数
    /// - `config`: アプリケーション設定
    ///
    /// # 戻り値
    /// データベース接続オブジェクト
    pub async fn connect_from_config(
        config: &crate::infrastructure::config::Config,
    ) -> Result<DatabaseConnection> {
        Self::connect(&config.storage.database_url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::config::app_config::StorageConfig;
    use crate::infrastructure::config::Config;

    #[tokio::test]
    async fn test_connect_with_valid_url() {
        // SQLiteのインメモリデータベースに接続
        let db = DatabaseConnectionManager::connect("sqlite::memory:")
            .await
            .unwrap();

        // 接続が成功することを確認
        assert!(db.ping().await.is_ok());
    }

    #[tokio::test]
    async fn test_connect_from_config() {
        // テスト用の設定を作成
        let config = Config {
            storage: StorageConfig {
                database_url: "sqlite::memory:".to_string(),
            },
        };

        // 設定から接続を作成
        let db = DatabaseConnectionManager::connect_from_config(&config)
            .await
            .unwrap();

        // 接続が成功することを確認
        assert!(db.ping().await.is_ok());
    }

    #[tokio::test]
    async fn test_connect_with_invalid_url() {
        // 不正なURLで接続を試みる
        let result = DatabaseConnectionManager::connect("invalid://url").await;

        // エラーが返されることを確認
        assert!(result.is_err());
    }
}
