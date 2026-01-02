use crate::domain::task::{Priority, Status};
use chrono::NaiveDate;
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

/// 日付文字列をパースする関数
///
/// # 引数
/// - `s`: YYYY-MM-DD形式の日付文字列
///
/// # 戻り値
/// - `Ok(NaiveDate)`: パースに成功した場合
/// - `Err(String)`: パースに失敗した場合、エラーメッセージを返す
fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        format!(
            "日付のパースに失敗しました: {}。YYYY-MM-DD形式で入力してください",
            e
        )
    })
}

/// 自然数（正の整数）をパースする関数
///
/// # 引数
/// - `s`: 整数を表す文字列
///
/// # 戻り値
/// - `Ok(i32)`: パースに成功し、1以上の整数の場合
/// - `Err(String)`: パースに失敗した場合、または0以下の場合、エラーメッセージを返す
fn parse_positive_id(s: &str) -> Result<i32, String> {
    let id: i32 = s
        .parse()
        .map_err(|_| format!("IDは整数である必要があります: {}", s))?;

    if id <= 0 {
        return Err(format!("IDは1以上の自然数である必要があります: {}", id));
    }

    Ok(id)
}

/// 空でない文字列をパースする関数
///
/// # 引数
/// - `s`: 文字列
///
/// # 戻り値
/// - `Ok(String)`: 空でない文字列の場合
/// - `Err(String)`: 空文字列または空白のみの場合、エラーメッセージを返す
fn parse_non_empty_string(s: &str) -> Result<String, String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err("空文字列は指定できません".to_string());
    }
    Ok(s.to_string())
}

/// タスク管理アプリケーションのコマンドライン引数
#[derive(Parser, Debug)]
#[command(
    name = "yaru",
    version,
    about = "シンプルなタスク管理CLI",
    long_about = "yaru は軽量で使いやすいコマンドラインタスク管理ツールです。\nタスクの追加、一覧表示、削除が簡単に行えます。"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

/// 実行可能なコマンド
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// タスク管理コマンド
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },
    /// タグ管理コマンド
    Tag {
        #[command(subcommand)]
        command: TagCommands,
    },
}

/// タスク管理用のサブコマンド
#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    /// タスク一覧を表示
    List {
        /// フィルタ条件（例: status:done, status:pending）
        #[arg(short, long, value_parser = clap::value_parser!(Filter))]
        filter: Option<Vec<Filter>>,
    },
    /// タスクの詳細を表示
    Show {
        /// 詳細表示するタスクのID
        #[arg(value_parser = parse_positive_id)]
        id: i32,
    },
    /// 新しいタスクを追加
    Add {
        /// タスクのタイトル
        #[arg(value_parser = parse_non_empty_string)]
        title: Option<String>,
        /// タスクの説明
        #[arg(short, long, value_parser = parse_non_empty_string)]
        description: Option<String>,
        /// タスクの状態
        #[arg(short, long)]
        status: Option<Status>,
        /// タスクの優先度
        #[arg(short, long)]
        priority: Option<Priority>,
        /// タスクに紐づけるタグのID（カンマ区切り）
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<i32>>,
        /// タスクの期限（YYYY-MM-DD形式）
        #[arg(long, value_parser = parse_date)]
        due_date: Option<NaiveDate>,
    },
    /// 指定されたIDのタスクを削除
    Delete {
        /// 削除するタスクのID
        #[arg(value_parser = parse_positive_id)]
        id: i32,
    },
    /// タスクを編集
    Edit {
        /// 編集するタスクのID
        #[arg(value_parser = parse_positive_id)]
        id: i32,
        /// タスクのタイトル
        #[arg(short, long, value_parser = parse_non_empty_string)]
        title: Option<String>,
        /// タスクの説明
        #[arg(short, long, value_parser = parse_non_empty_string)]
        description: Option<String>,
        /// タスクの状態
        #[arg(short, long)]
        status: Option<Status>,
        /// タスクの優先度
        #[arg(short, long)]
        priority: Option<Priority>,
        /// タスクに紐づけるタグのID（カンマ区切り、完全置換）
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<i32>>,
        /// タスクの期限（YYYY-MM-DD形式）
        #[arg(long, value_parser = parse_date)]
        due_date: Option<NaiveDate>,
        /// 期限をクリア
        #[arg(long, conflicts_with = "due_date")]
        clear_due_date: bool,
    },
}

/// タグ管理用のサブコマンド
#[derive(Subcommand, Debug)]
pub enum TagCommands {
    /// タグ一覧を表示
    List,
    /// タグの詳細を表示
    Show {
        /// 詳細表示するタグのID
        #[arg(value_parser = parse_positive_id)]
        id: i32,
    },
    /// 新しいタグを追加
    Add {
        /// タグの名前
        #[arg(value_parser = parse_non_empty_string)]
        name: Option<String>,
        /// タグの説明
        #[arg(short, long, value_parser = parse_non_empty_string)]
        description: Option<String>,
    },
    /// 指定されたIDのタグを削除
    Delete {
        /// 削除するタグのID
        #[arg(value_parser = parse_positive_id)]
        id: i32,
    },
    /// タグを編集
    Edit {
        /// 編集するタグのID
        #[arg(value_parser = parse_positive_id)]
        id: i32,
        /// タグの名前
        #[arg(short, long, value_parser = parse_non_empty_string)]
        name: Option<String>,
        /// タグの説明
        #[arg(short, long, value_parser = parse_non_empty_string)]
        description: Option<String>,
    },
}
