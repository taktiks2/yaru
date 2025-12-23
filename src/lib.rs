mod cli;
mod commands;
mod display;
mod error;
mod json;
mod repository;
mod todo;

use clap::{Parser, error::ErrorKind};
use cli::{Args, Commands};
use commands::{add_todo, delete_todo, list_todos};
use error::YaruError;
use repository::{JsonTodoRepository, TodoRepository};

/// アプリケーションのエントリーポイント
///
/// コマンドライン引数をパースし、適切なコマンドを実行します。
pub fn run() -> Result<(), YaruError> {
    let args = Args::try_parse().map_err(|e| {
        if e.kind() == ErrorKind::InvalidSubcommand {
            YaruError::InvalidSubcommand
        } else {
            YaruError::ClapError(e)
        }
    })?;

    // データファイルが存在することを確認
    let repo = JsonTodoRepository::default();
    repo.ensure_data_exists()?;

    handle_command(args)
}

/// コマンドを実行
fn handle_command(args: Args) -> Result<(), YaruError> {
    match args.command {
        Commands::List => list_todos(),
        Commands::Add { title, status } => add_todo(title, status),
        Commands::Delete { id } => delete_todo(id),
    }
}
