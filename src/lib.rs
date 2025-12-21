mod cli;
mod error;
mod json;
mod todo;

use clap::{Parser, error::ErrorKind};
use cli::{Args, Commands};
use error::YaruError;
use json::{load_json, save_json};
use std::path::Path;
use todo::Todo;

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
        Commands::Add { title } => add_todo(&title),
        Commands::Delete { id } => delete_todo(id),
    }
}

/// 全てのTodoを一覧表示
fn list_todos() -> Result<(), YaruError> {
    let todos = load_json::<Vec<Todo>>(PATH_TO_JSON)?;

    if todos.is_empty() {
        println!("タスクはありません。");
        return Ok(());
    }

    for todo in todos {
        println!("ID: {}, タイトル: {}", todo.id, todo.title);
    }
    Ok(())
}

/// 新しいTodoを追加
fn add_todo(title: &str) -> Result<(), YaruError> {
    let mut todos = load_json::<Vec<Todo>>(PATH_TO_JSON)?;
    let new_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
    let new_todo = Todo::new(new_id, title);

    todos.push(new_todo.clone());
    save_json(PATH_TO_JSON, &todos)?;

    println!("タスクを登録しました。");
    println!("ID: {}, タイトル: {}", new_todo.id, new_todo.title);
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
