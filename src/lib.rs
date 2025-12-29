mod cli;
mod commands;
mod config;
mod display;
mod json;
mod repository;
mod tag;
mod task;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, error::ErrorKind};
use cli::{Args, Commands, TagCommands};
use commands::{add_tag, add_task, delete_tag, delete_task, list_tags, list_tasks};
use config::load_config;
use repository::{JsonRepository, Repository};

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
    let task_repo = JsonRepository::new(&config.storage.task_file);
    task_repo
        .ensure_data_exists()
        .context("タスクファイルの初期化に失敗しました")?;

    let tag_repo = JsonRepository::new(&config.storage.tag_file);
    tag_repo
        .ensure_data_exists()
        .context("タグファイルの初期化に失敗しました")?;

    handle_command(args, task_repo, tag_repo)
}

/// コマンドを実行
fn handle_command(
    args: Args,
    task_repo: JsonRepository<task::Task>,
    tag_repo: JsonRepository<tag::Tag>,
) -> Result<()> {
    match args.command {
        Commands::List { filter } => list_tasks(&task_repo, filter),
        Commands::Add {
            title,
            description,
            status,
            priority,
            tags,
        } => add_task(&task_repo, &tag_repo, title, description, status, priority, tags),
        Commands::Delete { id } => delete_task(&task_repo, id),
        Commands::Tag { command } => handle_tag_command(command, tag_repo, task_repo),
    }
}

/// タグコマンドを実行
fn handle_tag_command(
    command: TagCommands,
    tag_repo: JsonRepository<tag::Tag>,
    task_repo: JsonRepository<task::Task>,
) -> Result<()> {
    match command {
        TagCommands::Add { name, description } => add_tag(&tag_repo, name, description),
        TagCommands::List => list_tags(&tag_repo),
        TagCommands::Delete { id } => delete_tag(&tag_repo, &task_repo, id),
    }
}
