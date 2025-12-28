use crate::todo::Status;
use clap::{Parser, Subcommand};
use std::str::FromStr;

/// フィルタ条件を表す構造体
#[derive(Debug, Clone)]
pub struct Filter {
    pub key: FilterKey,
    pub value: String,
}

/// フィルタキーの種類
#[derive(Debug, Clone, PartialEq)]
pub enum FilterKey {
    Status,
}

impl FromStr for Filter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid filter format: '{}'. Expected 'key:value'",
                s
            ));
        }

        let key = match parts[0].to_lowercase().as_str() {
            "status" => FilterKey::Status,
            _ => return Err(format!("Unknown filter key: '{}'", parts[0])),
        };

        Ok(Filter {
            key,
            value: parts[1].to_string(),
        })
    }
}

/// Todoアプリケーションのコマンドライン引数
#[derive(Parser, Debug)]
#[command(
    name = "yaru",
    version,
    about = "シンプルなTodoタスク管理CLI",
    long_about = "yaru は軽量で使いやすいコマンドラインTodo管理ツールです。\nタスクの追加、一覧表示、削除が簡単に行えます。"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

/// 実行可能なコマンド
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// タスク一覧を表示
    List {
        /// フィルタ条件（例: status:done, status:pending）
        #[arg(short, long, value_parser = clap::value_parser!(Filter))]
        filter: Option<Vec<Filter>>,
    },
    /// 新しいタスクを追加
    Add {
        /// タスクのタイトル
        #[arg(short, long)]
        title: Option<String>,
        /// タスクの説明
        #[arg(short, long)]
        description: Option<String>,
        /// タスクの状態
        #[arg(short, long)]
        status: Option<Status>,
    },
    /// 指定されたIDのタスクを削除
    Delete {
        /// 削除するタスクのID
        #[arg(short, long)]
        id: u64,
    },
}
