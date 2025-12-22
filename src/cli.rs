use clap::{Parser, Subcommand};

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
    List,
    /// 新しいタスクを追加
    Add {
        /// タスクのタイトル
        #[arg(short, long)]
        title: Option<String>,
    },
    /// 指定されたIDのタスクを削除
    Delete {
        /// 削除するタスクのID
        #[arg(short, long)]
        id: u64,
    },
}
