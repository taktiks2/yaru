mod cli;
mod commands;
mod config;
mod display;
mod entity;
mod json;
mod repository;
mod sqlite_repository;
mod tag;
mod task;

use anyhow::{Context, Result};
use clap::{error::ErrorKind, Parser};
use cli::{Args, Commands, TagCommands, TaskCommands};
use commands::{
    tag::{add_tag, delete_tag, list_tags, show_tag},
    task::{add_task, delete_task, list_tasks, show_task},
};
use config::load_config;
use migration::MigratorTrait;
use sea_orm::Database;
use sqlite_repository::SqliteRepository;

/// アプリケーションのエントリーポイント
///
/// コマンドライン引数をパースし、適切なコマンドを実行します。
#[tokio::main]
pub async fn run() -> Result<()> {
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            if e.kind() == ErrorKind::InvalidSubcommand {
                anyhow::bail!("無効なサブコマンドです。使用可能なコマンド: task, tag");
            } else {
                return Err(e.into());
            }
        }
    };

    // 設定を読み込む
    let config = load_config()?;

    // データベース接続を確立
    let db = Database::connect(&config.storage.database_url)
        .await
        .context("データベース接続に失敗しました")?;

    // マイグレーション実行
    migration::Migrator::up(&db, None)
        .await
        .context("マイグレーション実行に失敗しました")?;

    // リポジトリ作成
    let task_repo = SqliteRepository::new(db.clone());
    let tag_repo = SqliteRepository::new(db.clone());

    handle_command(args, task_repo, tag_repo)?;

    // 接続を明示的に閉じる
    db.close().await?;

    Ok(())
}

/// コマンドを実行
fn handle_command(
    args: Args,
    task_repo: SqliteRepository<task::Task>,
    tag_repo: SqliteRepository<tag::Tag>,
) -> Result<()> {
    match args.command {
        Commands::Task { command } => handle_task_command(command, task_repo, tag_repo),
        Commands::Tag { command } => handle_tag_command(command, tag_repo, task_repo),
    }
}

/// タスクコマンドを実行
fn handle_task_command(
    command: TaskCommands,
    task_repo: SqliteRepository<task::Task>,
    tag_repo: SqliteRepository<tag::Tag>,
) -> Result<()> {
    match command {
        TaskCommands::List { filter } => list_tasks(&task_repo, filter),
        TaskCommands::Show { id } => show_task(&task_repo, &tag_repo, id),
        TaskCommands::Add {
            title,
            description,
            status,
            priority,
            tags,
        } => add_task(
            &task_repo,
            &tag_repo,
            title,
            description,
            status,
            priority,
            tags,
        ),
        TaskCommands::Delete { id } => delete_task(&task_repo, id),
    }
}

/// タグコマンドを実行
fn handle_tag_command(
    command: TagCommands,
    tag_repo: SqliteRepository<tag::Tag>,
    task_repo: SqliteRepository<task::Task>,
) -> Result<()> {
    match command {
        TagCommands::Add { name, description } => add_tag(&tag_repo, name, description),
        TagCommands::Show { id } => show_tag(&tag_repo, id),
        TagCommands::List => list_tags(&tag_repo),
        TagCommands::Delete { id } => delete_tag(&tag_repo, &task_repo, id),
    }
}
