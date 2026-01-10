use chrono::NaiveDate;
use clap::{Parser, Subcommand, ValueEnum};
use std::str::FromStr;

use crate::domain::task::specification::SearchField;
use crate::domain::task::value_objects::{Priority, Status};

/// フィルタ条件を表す構造体
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Filter {
    pub key: FilterKey,
    pub value: String,
}

/// フィルタキーの種類
#[derive(Debug, Clone, PartialEq)]
pub enum FilterKey {
    /// ステータスフィルタ
    Status,
    /// 優先度フィルタ
    Priority,
    /// タグフィルタ
    Tag,
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
            "priority" => FilterKey::Priority,
            "tag" => FilterKey::Tag,
            _ => return Err(format!("Unknown filter key: '{}'", parts[0])),
        };

        Ok(Filter {
            key,
            value: parts[1].to_string(),
        })
    }
}

/// ソートキーの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    /// 優先度でソート
    Priority,
    /// 期限日でソート
    DueDate,
    /// 作成日でソート
    CreatedAt,
}

impl FromStr for SortKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "priority" => Ok(Self::Priority),
            "due_date" => Ok(Self::DueDate),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(format!("Unknown sort key: '{}'", s)),
        }
    }
}

/// ソート順序
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    /// 昇順
    Asc,
    /// 降順
    Desc,
}

impl FromStr for Order {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err(format!("Unknown order: '{}'", s)),
        }
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
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse date: {}. Please use YYYY-MM-DD format", e))
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
        .map_err(|_| format!("ID must be an integer: {}", s))?;

    if id <= 0 {
        return Err(format!("ID must be a positive integer (>= 1): {}", id));
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
        return Err("Empty string is not allowed".to_string());
    }
    Ok(s.to_string())
}

/// タスク管理アプリケーションのコマンドライン引数
#[derive(Parser, Debug)]
#[command(
    name = "yaru",
    version,
    about = "Simple task management CLI",
    long_about = "yaru is a lightweight and easy-to-use command-line task management tool.\nYou can easily add, list, and delete tasks."
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// 実行可能なコマンド
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Task management commands
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },
    /// Tag management commands
    Tag {
        #[command(subcommand)]
        command: TagCommands,
    },
}

/// 検索対象フィールド（CLI引数用）
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SearchFieldArg {
    /// タイトルのみ
    Title,
    /// 説明のみ
    Description,
    /// タイトルと説明の両方
    All,
}

impl From<SearchFieldArg> for SearchField {
    fn from(arg: SearchFieldArg) -> Self {
        match arg {
            SearchFieldArg::Title => Self::Title,
            SearchFieldArg::Description => Self::Description,
            SearchFieldArg::All => Self::All,
        }
    }
}

/// タスク管理用のサブコマンド
#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    /// List all tasks
    List {
        /// Filter conditions (e.g., status:done, status:pending)
        #[arg(short, long, value_parser = clap::value_parser!(Filter))]
        filter: Option<Vec<Filter>>,
        /// Sort key (priority, due_date, created_at)
        #[arg(short, long)]
        sort: Option<SortKey>,
        /// Sort order (asc, desc)
        #[arg(short, long)]
        order: Option<Order>,
    },
    /// Show task details
    Show {
        /// Task ID to show
        #[arg(value_parser = parse_positive_id)]
        id: i32,
    },
    /// Add a new task
    Add {
        /// Task title
        #[arg(value_parser = parse_non_empty_string)]
        title: Option<String>,
        /// Task description
        #[arg(short, long, value_parser = parse_non_empty_string)]
        description: Option<String>,
        /// Task status
        #[arg(short, long)]
        status: Option<Status>,
        /// Task priority
        #[arg(short, long)]
        priority: Option<Priority>,
        /// Tag IDs to attach (comma-separated)
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<i32>>,
        /// Task due date (YYYY-MM-DD format)
        #[arg(long, value_parser = parse_date)]
        due_date: Option<NaiveDate>,
    },
    /// Delete a task by ID
    Delete {
        /// Task ID to delete
        #[arg(value_parser = parse_positive_id)]
        id: i32,
    },
    /// Edit a task
    Edit {
        /// Task ID to edit
        #[arg(value_parser = parse_positive_id)]
        id: i32,
        /// Task title
        #[arg(short, long, value_parser = parse_non_empty_string)]
        title: Option<String>,
        /// Task description
        #[arg(short, long, value_parser = parse_non_empty_string)]
        description: Option<String>,
        /// Task status
        #[arg(short, long)]
        status: Option<Status>,
        /// Task priority
        #[arg(short, long)]
        priority: Option<Priority>,
        /// Tag IDs to attach (comma-separated, replaces existing)
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<i32>>,
        /// Task due date (YYYY-MM-DD format)
        #[arg(long, value_parser = parse_date)]
        due_date: Option<NaiveDate>,
        /// Clear due date
        #[arg(long, conflicts_with = "due_date")]
        clear_due_date: bool,
    },
    /// Show task statistics
    Stats,
    /// Search tasks by keyword
    Search {
        /// Search keywords (space-separated for AND condition)
        /// Will prompt in interactive mode if omitted
        keywords: Option<String>,

        /// Search target field (title, description, all)
        #[arg(short, long, default_value = "all")]
        field: SearchFieldArg,
    },
}

/// タグ管理用のサブコマンド
#[derive(Subcommand, Debug)]
pub enum TagCommands {
    /// List all tags
    List,
    /// Show tag details
    Show {
        /// Tag ID to show
        #[arg(value_parser = parse_positive_id)]
        id: i32,
    },
    /// Add a new tag
    Add {
        /// Tag name
        #[arg(value_parser = parse_non_empty_string)]
        name: Option<String>,
        /// Tag description
        #[arg(short, long, value_parser = parse_non_empty_string)]
        description: Option<String>,
    },
    /// Delete a tag by ID
    Delete {
        /// Tag ID to delete
        #[arg(value_parser = parse_positive_id)]
        id: i32,
    },
    /// Edit a tag
    Edit {
        /// Tag ID to edit
        #[arg(value_parser = parse_positive_id)]
        id: i32,
        /// Tag name
        #[arg(short, long, value_parser = parse_non_empty_string)]
        name: Option<String>,
        /// Tag description
        #[arg(short, long, value_parser = parse_non_empty_string)]
        description: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_without_subcommand() {
        // 引数なしでパース可能
        let args = Args::try_parse_from(vec!["yaru"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.command.is_none());
    }

    #[test]
    fn test_args_with_subcommand() {
        // 引数ありでパース可能
        let args = Args::try_parse_from(vec!["yaru", "task", "list"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.command.is_some());
    }

    // SearchCommand のテストケース

    #[test]
    fn test_task_search_basic() {
        // 基本的な検索のパース
        let args = Args::try_parse_from(vec!["yaru", "task", "search", "買い物"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        if let Some(Commands::Task {
            command: TaskCommands::Search { keywords, field },
        }) = args.command
        {
            assert_eq!(keywords, Some("買い物".to_string()));
            assert_eq!(field, SearchFieldArg::All); // デフォルト
        } else {
            panic!("Expected Task::Search command");
        }
    }

    #[test]
    fn test_task_search_multiple_keywords() {
        // 複数キーワードのパース
        let args = Args::try_parse_from(vec!["yaru", "task", "search", "レポート 作成"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        if let Some(Commands::Task {
            command: TaskCommands::Search { keywords, field },
        }) = args.command
        {
            assert_eq!(keywords, Some("レポート 作成".to_string()));
            assert_eq!(field, SearchFieldArg::All);
        } else {
            panic!("Expected Task::Search command");
        }
    }

    #[test]
    fn test_task_search_field_title() {
        // フィールドオプション付きのパース
        let args =
            Args::try_parse_from(vec!["yaru", "task", "search", "買い物", "--field", "title"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        if let Some(Commands::Task {
            command: TaskCommands::Search { keywords, field },
        }) = args.command
        {
            assert_eq!(keywords, Some("買い物".to_string()));
            assert_eq!(field, SearchFieldArg::Title);
        } else {
            panic!("Expected Task::Search command");
        }
    }

    #[test]
    fn test_task_search_no_keywords() {
        // キーワード省略時（対話モード）のパース
        let args = Args::try_parse_from(vec!["yaru", "task", "search"]);
        assert!(args.is_ok());
        let args = args.unwrap();
        if let Some(Commands::Task {
            command: TaskCommands::Search { keywords, field },
        }) = args.command
        {
            assert_eq!(keywords, None); // キーワードなし
            assert_eq!(field, SearchFieldArg::All);
        } else {
            panic!("Expected Task::Search command");
        }
    }

    // Filter のテストケース

    #[test]
    fn test_filter_parse_status() {
        // ステータスフィルタのパース
        let filter = Filter::from_str("status:pending");
        assert!(filter.is_ok());
        let filter = filter.unwrap();
        assert_eq!(filter.key, FilterKey::Status);
        assert_eq!(filter.value, "pending");
    }

    #[test]
    fn test_filter_parse_priority() {
        // 優先度フィルタのパース
        let filter = Filter::from_str("priority:high");
        assert!(filter.is_ok());
        let filter = filter.unwrap();
        assert_eq!(filter.key, FilterKey::Priority);
        assert_eq!(filter.value, "high");
    }

    #[test]
    fn test_filter_parse_tag() {
        // タグフィルタのパース
        let filter = Filter::from_str("tag:仕事");
        assert!(filter.is_ok());
        let filter = filter.unwrap();
        assert_eq!(filter.key, FilterKey::Tag);
        assert_eq!(filter.value, "仕事");
    }

    #[test]
    fn test_filter_parse_invalid_format() {
        // 無効なフォーマット
        let filter = Filter::from_str("invalid");
        assert!(filter.is_err());
        assert!(filter.unwrap_err().contains("Invalid filter format"));
    }

    #[test]
    fn test_filter_parse_unknown_key() {
        // 未知のフィルタキー
        let filter = Filter::from_str("unknown:value");
        assert!(filter.is_err());
        assert!(filter.unwrap_err().contains("Unknown filter key"));
    }

    #[test]
    fn test_filter_parse_case_insensitive() {
        // 大文字小文字を無視
        let filter = Filter::from_str("STATUS:pending");
        assert!(filter.is_ok());
        let filter = filter.unwrap();
        assert_eq!(filter.key, FilterKey::Status);
    }

    // SortKey のテストケース

    #[test]
    fn test_sort_key_parse_priority() {
        // 優先度でソート
        let sort_key = SortKey::from_str("priority");
        assert!(sort_key.is_ok());
        assert_eq!(sort_key.unwrap(), SortKey::Priority);
    }

    #[test]
    fn test_sort_key_parse_due_date() {
        // 期限日でソート
        let sort_key = SortKey::from_str("due_date");
        assert!(sort_key.is_ok());
        assert_eq!(sort_key.unwrap(), SortKey::DueDate);
    }

    #[test]
    fn test_sort_key_parse_created_at() {
        // 作成日でソート
        let sort_key = SortKey::from_str("created_at");
        assert!(sort_key.is_ok());
        assert_eq!(sort_key.unwrap(), SortKey::CreatedAt);
    }

    #[test]
    fn test_sort_key_parse_case_insensitive() {
        // 大文字小文字を無視
        let sort_key = SortKey::from_str("PRIORITY");
        assert!(sort_key.is_ok());
        assert_eq!(sort_key.unwrap(), SortKey::Priority);
    }

    #[test]
    fn test_sort_key_parse_invalid() {
        // 無効なソートキー
        let sort_key = SortKey::from_str("invalid");
        assert!(sort_key.is_err());
    }

    // Order のテストケース

    #[test]
    fn test_order_parse_asc() {
        // 昇順
        let order = Order::from_str("asc");
        assert!(order.is_ok());
        assert_eq!(order.unwrap(), Order::Asc);
    }

    #[test]
    fn test_order_parse_desc() {
        // 降順
        let order = Order::from_str("desc");
        assert!(order.is_ok());
        assert_eq!(order.unwrap(), Order::Desc);
    }

    #[test]
    fn test_order_parse_case_insensitive() {
        // 大文字小文字を無視
        let order = Order::from_str("DESC");
        assert!(order.is_ok());
        assert_eq!(order.unwrap(), Order::Desc);
    }

    #[test]
    fn test_order_parse_invalid() {
        // 無効なオーダー
        let order = Order::from_str("invalid");
        assert!(order.is_err());
    }
}
