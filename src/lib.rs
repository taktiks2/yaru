mod cli;
mod commands;
mod config;
mod display;
mod json;
mod repository;
mod task;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, error::ErrorKind};
use cli::{Args, Commands};
use commands::{add_task, delete_task, list_tasks};
use config::load_config;
use repository::{JsonTaskRepository, TaskRepository};

/// アプリケーションのエントリーポイント
///
/// コマンドライン引数をパースし、適切なコマンドを実行します。
pub fn run() -> Result<()> {
    let args = Args::try_parse().map_err(|e| {
        if e.kind() == ErrorKind::InvalidSubcommand {
            anyhow!("無効なサブコマンドです。使用可能なコマンド: list, add, delete")
        } else {
            e.into()
        }
    })?;

    // 設定を読み込む
    let config = load_config()?;

    // データファイルが存在することを確認
    let repo = JsonTaskRepository::new(&config.storage.task_file);
    repo.ensure_data_exists()
        .context("データファイルの初期化に失敗しました")?;

    handle_command(args, repo)
}

/// コマンドを実行
fn handle_command(args: Args, repo: JsonTaskRepository) -> Result<()> {
    match args.command {
        Commands::List { filter } => list_tasks(&repo, filter),
        Commands::Add {
            title,
            description,
            status,
            priority,
        } => add_task(&repo, title, description, status, priority),
        Commands::Delete { id } => delete_task(&repo, id),
    }
}
