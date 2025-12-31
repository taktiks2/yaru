mod cli;
mod commands;
mod config;
mod display;
mod entity;
mod json;
mod repository;
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

/// アプリケーションのエントリーポイント
///
/// コマンドライン引数をパースし、適切なコマンドを実行します。
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

    // コマンド実行（DB接続を直接渡す）
    handle_command(args, &db).await?;

    // 接続を明示的に閉じる
    db.close().await?;

    Ok(())
}

/// コマンドを実行
async fn handle_command(args: Args, db: &sea_orm::DatabaseConnection) -> Result<()> {
    match args.command {
        Commands::Task { command } => handle_task_command(command, db).await,
        Commands::Tag { command } => handle_tag_command(command, db).await,
    }
}

/// タスクコマンドを実行
async fn handle_task_command(
    command: TaskCommands,
    db: &sea_orm::DatabaseConnection,
) -> Result<()> {
    match command {
        TaskCommands::List { filter } => list_tasks(db, filter).await,
        TaskCommands::Show { id } => show_task(db, id).await,
        TaskCommands::Add {
            title,
            description,
            status,
            priority,
            tags,
        } => add_task(db, title, description, status, priority, tags).await,
        TaskCommands::Delete { id } => delete_task(db, id).await,
    }
}

/// タグコマンドを実行
async fn handle_tag_command(command: TagCommands, db: &sea_orm::DatabaseConnection) -> Result<()> {
    match command {
        TagCommands::Add { name, description } => add_tag(db, name, description).await,
        TagCommands::Show { id } => show_tag(db, id).await,
        TagCommands::List => list_tags(db).await,
        TagCommands::Delete { id } => delete_tag(db, id).await,
    }
}
