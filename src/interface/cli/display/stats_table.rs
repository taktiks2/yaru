use crate::application::dto::stats_dto::StatsDTO;
use comfy_table::{Table, presets::UTF8_FULL};

/// 統計情報のテーブルを作成
pub fn create_stats_table(stats: &StatsDTO) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    // 全体統計
    table.add_row(vec!["全タスク数", &stats.total_count.to_string()]);

    // ステータス別統計
    if !stats.status_stats.is_empty() {
        table.add_row(vec!["", ""]);
        table.add_row(vec!["ステータス別", ""]);
        for (status, count) in &stats.status_stats {
            table.add_row(vec![&format!("  {}", status), &count.to_string()]);
        }
    }

    // 優先度別統計
    if !stats.priority_stats.is_empty() {
        table.add_row(vec!["", ""]);
        table.add_row(vec!["優先度別", ""]);
        for (priority, count) in &stats.priority_stats {
            table.add_row(vec![&format!("  {}", priority), &count.to_string()]);
        }
    }

    // 期限関連統計
    if !stats.due_date_stats.is_empty() {
        table.add_row(vec!["", ""]);
        table.add_row(vec!["期限関連", ""]);
        for (status, count) in &stats.due_date_stats {
            table.add_row(vec![&format!("  {}", status), &count.to_string()]);
        }
    }

    // タグ別統計
    if !stats.tag_stats.is_empty() {
        table.add_row(vec!["", ""]);
        table.add_row(vec!["タグ別", ""]);
        for (tag, count) in &stats.tag_stats {
            table.add_row(vec![&format!("  {}", tag), &count.to_string()]);
        }
    }

    table
}
