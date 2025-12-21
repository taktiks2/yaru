mod cli;
mod error;
mod json;
mod todo;

use anyhow::Result;

use chrono::Local;
pub use cli::{Args, Commands};
use error::YaruError;
use json::{load_json, save_json};
use todo::Todo;

const PATH_TO_JSON: &str = "todo.json";

pub fn run(args: Args) -> Result<()> {
    println!("{args:?}");

    match args.command {
        Commands::List => {
            let todos = load_json::<Vec<Todo>>(PATH_TO_JSON)?;
            for todo in todos {
                println!("ID: {}, タイトル: {}", todo.id, todo.title);
            }
        }
        Commands::Add { title } => {
            let todos = load_json::<Vec<Todo>>(PATH_TO_JSON)?;
            let new_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
            let new_todo = Todo {
                id: new_id,
                title: title.clone(),
                created_at: Local::now().to_rfc3339(),
            };
            save_json(PATH_TO_JSON, &{
                let mut updated_todos = todos;
                updated_todos.push(new_todo.clone());
                updated_todos
            })?;
            println!("タスクを登録しました。");
            println!("ID: {}, タイトル: {}", new_todo.id, new_todo.title);
        }
        Commands::Delete { id } => {
            let todos = load_json::<Vec<Todo>>(PATH_TO_JSON)?;
            save_json(
                PATH_TO_JSON,
                &todos
                    .into_iter()
                    .filter(|todo| todo.id != id)
                    .collect::<Vec<Todo>>(),
            )?;
            println!("タスクを削除しました。");
        }
    }

    Ok(())
}
