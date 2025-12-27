mod cli;
mod commands;
mod config;
mod display;
mod json;
mod repository;
mod todo;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, error::ErrorKind};
use cli::{Args, Commands};
use commands::{add_todo, delete_todo, list_todos};
use config::load_config;
use repository::{JsonTodoRepository, TodoRepository};

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
    let repo = JsonTodoRepository::new(&config.storage.todo_file);
    repo.ensure_data_exists()
        .context("データファイルの初期化に失敗しました")?;

    handle_command(args, repo)
}

/// コマンドを実行
fn handle_command(args: Args, repo: JsonTodoRepository) -> Result<()> {
    match args.command {
        Commands::List { filter } => list_todos(&repo, filter),
        Commands::Add { title, status } => add_todo(&repo, title, status),
        Commands::Delete { id } => delete_todo(&repo, id),
    }
}
