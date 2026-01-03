use anyhow::Result;

use crate::application::dto::{stats_dto::StatsDTO, tag_dto::TagDTO, task_dto::TaskDTO};

/// プレゼンテーション層の抽象トレイト
///
/// CLI/TUIの両方に対応できるよう、プレゼンテーションロジックを抽象化します。
pub trait Presenter: Send + Sync {
    /// タスク一覧を表示
    fn present_task_list(&self, tasks: &[TaskDTO]) -> Result<()>;

    /// タスク詳細を表示
    fn present_task_detail(&self, task: &TaskDTO) -> Result<()>;

    /// タグ一覧を表示
    fn present_tag_list(&self, tags: &[TagDTO]) -> Result<()>;

    /// タグ詳細を表示
    fn present_tag_detail(&self, tag: &TagDTO) -> Result<()>;

    /// 統計情報を表示
    fn present_stats(&self, stats: &StatsDTO) -> Result<()>;

    /// 成功メッセージを表示
    fn present_success(&self, message: &str) -> Result<()>;

    /// エラーメッセージを表示
    fn present_error(&self, error: &str) -> Result<()>;

    /// 確認メッセージを表示し、ユーザーの入力を取得
    fn confirm(&self, message: &str, default: bool) -> Result<bool>;
}

/// CLIプレゼンター
///
/// コマンドラインインターフェース用のプレゼンター実装。
/// テーブル形式でデータを表示します。
pub struct CliPresenter;

impl CliPresenter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CliPresenter {
    fn default() -> Self {
        Self::new()
    }
}

impl Presenter for CliPresenter {
    fn present_task_list(&self, tasks: &[TaskDTO]) -> Result<()> {
        use crate::interface::cli::display::create_task_table;

        if tasks.is_empty() {
            println!("タスクがありません");
        } else {
            println!("タスク一覧 ({}件):", tasks.len());
            let table = create_task_table(tasks);
            println!("{}", table);
        }

        Ok(())
    }

    fn present_task_detail(&self, task: &TaskDTO) -> Result<()> {
        use crate::interface::cli::display::create_task_detail_table;

        let table = create_task_detail_table(task);
        println!("{}", table);

        Ok(())
    }

    fn present_tag_list(&self, tags: &[TagDTO]) -> Result<()> {
        use crate::interface::cli::display::create_tag_table;

        if tags.is_empty() {
            println!("タグがありません");
        } else {
            println!("タグ一覧 ({}件):", tags.len());
            let table = create_tag_table(tags);
            println!("{}", table);
        }

        Ok(())
    }

    fn present_tag_detail(&self, tag: &TagDTO) -> Result<()> {
        use crate::interface::cli::display::create_tag_detail_table;

        let table = create_tag_detail_table(tag);
        println!("{}", table);

        Ok(())
    }

    fn present_stats(&self, stats: &StatsDTO) -> Result<()> {
        use crate::interface::cli::display::create_stats_table;

        let table = create_stats_table(stats);
        println!("{}", table);

        Ok(())
    }

    fn present_success(&self, message: &str) -> Result<()> {
        println!("{}", message);
        Ok(())
    }

    fn present_error(&self, error: &str) -> Result<()> {
        eprintln!("エラー: {}", error);
        Ok(())
    }

    fn confirm(&self, message: &str, default: bool) -> Result<bool> {
        use inquire::Confirm;

        let result = Confirm::new(message)
            .with_default(default)
            .prompt()
            .unwrap_or(false);

        Ok(result)
    }
}
