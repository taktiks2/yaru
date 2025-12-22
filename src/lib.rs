mod cli;
mod error;
mod json;
mod todo;

use chrono::{DateTime, Local};
use clap::{Parser, error::ErrorKind};
use cli::{Args, Commands};
use comfy_table::Table;
use dialoguer::Input;
use error::YaruError;
use json::{load_json, save_json};
use std::path::Path;
use todo::{Status, Todo};

const PATH_TO_JSON: &str = "todo.json";

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

    ensure_data_file_exists()?;
    handle_command(args)
}

/// データファイルが存在しない場合は空のJSONファイルを作成
fn ensure_data_file_exists() -> Result<(), YaruError> {
    if !Path::new(PATH_TO_JSON).exists() {
        save_json(PATH_TO_JSON, &Vec::<Todo>::new())?;
    }
    Ok(())
}

/// コマンドを実行
fn handle_command(args: Args) -> Result<(), YaruError> {
    match args.command {
        Commands::List => list_todos(),
        Commands::Add { title, status } => add_todo(title, status),
        Commands::Delete { id } => delete_todo(id),
    }
}

/// 全てのTodoを一覧表示
fn list_todos() -> Result<(), YaruError> {
    let todos: Vec<Todo> = load_json(PATH_TO_JSON)?;

    if todos.is_empty() {
        println!("タスクはありません。");
        return Ok(());
    }

    let mut table = Table::new();

    table.set_header(vec!["ID", "タイトル", "ステータス", "作成日", "更新日"]);

    for todo in todos {
        table.add_row(vec![
            todo.id.to_string(),
            todo.title,
            todo.status.to_string(),
            format_local_time(&todo.created_at),
            format_local_time(&todo.updated_at),
        ]);
    }

    println!("{table}");

    Ok(())
}

/// 新しいTodoを追加
fn add_todo(title: Option<String>, status: Option<Status>) -> Result<(), YaruError> {
    let title = match title {
        Some(t) => t,
        None => Input::new()
            .with_prompt("タスクのタイトルを入力してください")
            .interact_text()
            .map_err(|e| YaruError::IoError { source: e.into() })?,
    };

    let status = match status {
        Some(s) => s,
        None => Status::Pending,
    };

    let mut todos = load_json::<Vec<Todo>>(PATH_TO_JSON)?;
    let new_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
    let new_todo = Todo::new(new_id, &title, status);

    todos.push(new_todo.clone());
    save_json(PATH_TO_JSON, &todos)?;

    println!("タスクを登録しました。");

    let mut table = Table::new();

    table.set_header(vec!["ID", "タイトル", "ステータス", "作成日", "更新日"]);

    table.add_row(vec![
        new_todo.id.to_string(),
        new_todo.title,
        new_todo.status.to_string(),
        format_local_time(&new_todo.created_at),
        format_local_time(&new_todo.updated_at),
    ]);

    println!("{table}");

    Ok(())
}

/// 指定されたIDのTodoを削除
fn delete_todo(id: u64) -> Result<(), YaruError> {
    let todos = load_json::<Vec<Todo>>(PATH_TO_JSON)?;
    let initial_count = todos.len();
    let filtered_todos: Vec<Todo> = todos.into_iter().filter(|todo| todo.id != id).collect();

    if initial_count == filtered_todos.len() {
        println!("ID {} のタスクが見つかりませんでした。", id);
        return Ok(());
    }

    save_json(PATH_TO_JSON, &filtered_todos)?;
    println!("タスクを削除しました。");
    Ok(())
}

/// UTC時間の文字列を現地時間に変換してフォーマット
fn format_local_time(utc_time_str: &str) -> String {
    DateTime::parse_from_rfc3339(utc_time_str)
        .map(|dt| {
            dt.with_timezone(&Local)
                .format("%Y-%m-%d %H:%M")
                .to_string()
        })
        .unwrap_or_else(|_| utc_time_str.to_string())
}
