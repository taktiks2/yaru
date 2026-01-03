mod application;
mod domain;
mod infrastructure;
mod interface;

use anyhow::{Context, Result};
use clap::Parser;
use std::sync::Arc;

use infrastructure::{DatabaseConnectionManager, load_config};
use interface::cli::args::{Args, Commands};
use interface::cli::{tag_handler, task_handler};
use interface::persistence::sea_orm::{SeaOrmTagRepository, SeaOrmTaskRepository};
use migration::MigratorTrait;

/// アプリケーションのエントリーポイント
///
/// コマンドライン引数をパースし、適切なコマンドを実行します。
pub async fn run() -> Result<()> {
    run_cli().await
}

/// CLIモードで実行
async fn run_cli() -> Result<()> {
    // CLI引数をパース
    let args = Args::parse();

    // 設定を読み込む
    let config = load_config()?;

    // データベース接続を確立
    let db = DatabaseConnectionManager::connect_from_config(&config)
        .await
        .context("データベース接続に失敗しました")?;

    // マイグレーション実行
    migration::Migrator::up(&db, None)
        .await
        .context("マイグレーション実行に失敗しました")?;

    // リポジトリを初期化
    let task_repo = Arc::new(SeaOrmTaskRepository::new(db.clone()));
    let tag_repo = Arc::new(SeaOrmTagRepository::new(db.clone()));

    // コマンド実行
    match args.command {
        Commands::Task { command } => {
            task_handler::handle_task_command(command, task_repo, tag_repo).await?
        }
        Commands::Tag { command } => tag_handler::handle_tag_command(command, tag_repo).await?,
    }

    // 接続を明示的に閉じる
    db.close().await?;

    Ok(())
}
