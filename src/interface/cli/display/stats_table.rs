use crate::application::dto::stats_dto::StatsDTO;
use comfy_table::{Attribute, Cell, CellAlignment, Table, presets::UTF8_FULL};

/// プログレスバーを作成
///
/// # 引数
/// - `percentage`: 進捗率（0.0～100.0）
///
/// # 戻り値
/// ASCIIアートのプログレスバー（10文字）
fn create_progress_bar(percentage: f64) -> String {
    let filled_blocks = ((percentage / 10.0).round() as usize).min(10);
    let empty_blocks = 10 - filled_blocks;

    format!("{}{}", "█".repeat(filled_blocks), "░".repeat(empty_blocks))
}

/// タイトルを作成
///
/// # 引数
/// - `title`: タイトル文字列
///
/// # 戻り値
/// 装飾されたタイトル
fn create_title(title: &str) -> String {
    let line = "━".repeat(50);
    format!("{}\n{:^50}\n{}", line, title, line)
}

/// 期限関連のサマリーを作成
///
/// コンパクトな1行表示
fn create_due_date_summary(stats: &StatsDTO) -> String {
    let overdue = stats.due_date_stats.get("overdue").copied().unwrap_or(0);
    let due_today = stats.due_date_stats.get("due_today").copied().unwrap_or(0);
    let due_this_week = stats
        .due_date_stats
        .get("due_this_week")
        .copied()
        .unwrap_or(0);

    format!(
        "Overdue: {} tasks, Due today: {} tasks, Due this week: {} tasks",
        overdue, due_today, due_this_week
    )
}

/// priority_status_matrixにデータがあるかチェック
fn has_priority_status_data(stats: &StatsDTO) -> bool {
    !stats.priority_status_matrix.is_empty()
}

/// ステータス別の詳細テーブルを作成
///
/// 件数、パーセンテージ、プログレスバーを含むリッチな表示
fn create_status_detail_table(stats: &StatsDTO) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![
        Cell::new("Status").add_attribute(Attribute::Bold),
        Cell::new("Count")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Right),
        Cell::new("Percentage (%)")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Right),
        Cell::new("Progress").add_attribute(Attribute::Bold),
    ]);

    // 定義済みの順序でステータスを表示
    let status_order = ["pending", "in_progress", "completed"];
    let status_labels = ["Pending", "In Progress", "Completed"];

    for (i, status_key) in status_order.iter().enumerate() {
        if let Some(&count) = stats.status_stats.get(*status_key) {
            let percentage = if stats.total_count > 0 {
                (count as f64 / stats.total_count as f64) * 100.0
            } else {
                0.0
            };

            let progress_bar = create_progress_bar(percentage);

            table.add_row(vec![
                Cell::new(status_labels[i]),
                Cell::new(count.to_string()).set_alignment(CellAlignment::Right),
                Cell::new(format!("{:.1}%", percentage)).set_alignment(CellAlignment::Right),
                Cell::new(&progress_bar),
            ]);
        }
    }

    table
}

/// 優先度×ステータスのマトリックステーブルを作成
///
/// クロス集計により、各優先度のタスクがどのステータスにあるかを一覧表示
fn create_priority_status_matrix_table(stats: &StatsDTO) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    // ヘッダー行（ステータス）
    table.set_header(vec![
        Cell::new("").add_attribute(Attribute::Bold),
        Cell::new("Pending")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("In Progress")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Completed")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Total")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    // 優先度の順序と表示ラベル
    let priority_order = ["critical", "high", "medium", "low"];
    let priority_labels = ["Critical", "High", "Medium", "Low"];
    let status_order = ["pending", "in_progress", "completed"];

    let mut col_totals = vec![0, 0, 0]; // 各ステータスの合計

    // 各優先度の行を追加
    for (i, priority_key) in priority_order.iter().enumerate() {
        let mut row_cells = vec![Cell::new(priority_labels[i]).add_attribute(Attribute::Bold)];

        let mut row_total = 0;

        // 各ステータスのセルを追加
        for (j, status_key) in status_order.iter().enumerate() {
            let key = format!("{}:{}", priority_key, status_key);
            let count = stats.priority_status_matrix.get(&key).copied().unwrap_or(0);
            row_total += count;
            col_totals[j] += count;

            row_cells.push(Cell::new(count.to_string()).set_alignment(CellAlignment::Right));
        }

        // 行の合計
        row_cells.push(
            Cell::new(row_total.to_string())
                .set_alignment(CellAlignment::Right)
                .add_attribute(Attribute::Bold),
        );

        table.add_row(row_cells);
    }

    // 合計行を追加
    let mut total_row = vec![Cell::new("Total").add_attribute(Attribute::Bold)];
    for total in &col_totals {
        total_row.push(
            Cell::new(total.to_string())
                .set_alignment(CellAlignment::Right)
                .add_attribute(Attribute::Bold),
        );
    }
    total_row.push(
        Cell::new(stats.total_count.to_string())
            .set_alignment(CellAlignment::Right)
            .add_attribute(Attribute::Bold),
    );
    table.add_row(total_row);

    table
}

/// トップタグのテーブルを作成
///
/// タグを件数で降順ソートし、上位N件のみを表示
///
/// # 引数
/// - `stats`: 統計情報DTO
/// - `limit`: 表示する最大件数
fn create_top_tags_table(stats: &StatsDTO, limit: usize) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![
        Cell::new("Tag Name").add_attribute(Attribute::Bold),
        Cell::new("Count")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Right),
    ]);

    // タグを件数で降順ソート
    let mut tags: Vec<_> = stats.tag_stats.iter().collect();
    tags.sort_by(|a, b| b.1.cmp(a.1));

    // 上位N件のみ表示
    for (tag_name, count) in tags.iter().take(limit) {
        table.add_row(vec![
            Cell::new(*tag_name),
            Cell::new(count.to_string()).set_alignment(CellAlignment::Right),
        ]);
    }

    table
}

/// 統計情報のリッチ表示を作成
///
/// サマリー、ステータス別詳細、優先度×ステータスマトリックス、
/// 期限関連、トップタグを含む包括的な統計情報を表示します。
///
/// # 引数
/// - `stats`: 統計情報DTO
///
/// # 戻り値
/// フォーマットされた文字列（複数のテーブルを含む）
pub fn create_rich_stats_display(stats: &StatsDTO) -> String {
    let mut output = String::new();

    // タイトル
    output.push_str(&create_title("Task Statistics Summary"));
    output.push('\n');
    output.push('\n');

    // サマリーセクション
    output.push_str(&format!("Total tasks: {}\n", stats.total_count));
    output.push('\n');

    // ステータス別詳細テーブル（パーセンテージとプログレスバー付き）
    if !stats.status_stats.is_empty() {
        output.push_str("[By Status]\n");
        output.push_str(&create_status_detail_table(stats).to_string());
        output.push('\n');
        output.push('\n');
    }

    // 優先度×ステータス マトリックステーブル
    if has_priority_status_data(stats) {
        output.push_str("[Priority × Status Matrix]\n");
        output.push_str(&create_priority_status_matrix_table(stats).to_string());
        output.push('\n');
        output.push('\n');
    }

    // 期限関連（コンパクト表示）
    if !stats.due_date_stats.is_empty() {
        output.push_str("[Due Dates]\n");
        output.push_str(&create_due_date_summary(stats));
        output.push('\n');
        output.push('\n');
    }

    // トップタグ
    if !stats.tag_stats.is_empty() {
        output.push_str("[Top Tags (Top 5)]\n");
        output.push_str(&create_top_tags_table(stats, 5).to_string());
        output.push('\n');
    }

    output
}

// テストのみを先に作成（TDD）
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // ヘルパー関数のテスト

    #[test]
    fn test_create_progress_bar_0_percent() {
        let bar = create_progress_bar(0.0);
        assert_eq!(bar, "░░░░░░░░░░");
    }

    #[test]
    fn test_create_progress_bar_50_percent() {
        let bar = create_progress_bar(50.0);
        assert_eq!(bar, "█████░░░░░");
    }

    #[test]
    fn test_create_progress_bar_100_percent() {
        let bar = create_progress_bar(100.0);
        assert_eq!(bar, "██████████");
    }

    #[test]
    fn test_create_progress_bar_rounding() {
        let bar = create_progress_bar(35.7);
        assert_eq!(bar, "████░░░░░░"); // 35.7 -> 4ブロック
    }

    #[test]
    fn test_create_due_date_summary() {
        let mut due_date_stats = HashMap::new();
        due_date_stats.insert("overdue".to_string(), 5);
        due_date_stats.insert("due_today".to_string(), 2);
        due_date_stats.insert("due_this_week".to_string(), 8);

        let stats = StatsDTO {
            total_count: 42,
            status_stats: HashMap::new(),
            priority_stats: HashMap::new(),
            due_date_stats,
            tag_stats: HashMap::new(),
            priority_status_matrix: HashMap::new(),
        };

        let summary = create_due_date_summary(&stats);
        assert_eq!(
            summary,
            "Overdue: 5 tasks, Due today: 2 tasks, Due this week: 8 tasks"
        );
    }

    #[test]
    fn test_create_due_date_summary_empty() {
        let stats = StatsDTO {
            total_count: 42,
            status_stats: HashMap::new(),
            priority_stats: HashMap::new(),
            due_date_stats: HashMap::new(),
            tag_stats: HashMap::new(),
            priority_status_matrix: HashMap::new(),
        };

        let summary = create_due_date_summary(&stats);
        assert_eq!(
            summary,
            "Overdue: 0 tasks, Due today: 0 tasks, Due this week: 0 tasks"
        );
    }

    #[test]
    fn test_has_priority_status_data_true() {
        let mut priority_status_matrix = HashMap::new();
        priority_status_matrix.insert("high:pending".to_string(), 5);

        let stats = StatsDTO {
            total_count: 5,
            status_stats: HashMap::new(),
            priority_stats: HashMap::new(),
            due_date_stats: HashMap::new(),
            tag_stats: HashMap::new(),
            priority_status_matrix,
        };

        assert!(has_priority_status_data(&stats));
    }

    #[test]
    fn test_has_priority_status_data_false() {
        let stats = StatsDTO {
            total_count: 0,
            status_stats: HashMap::new(),
            priority_stats: HashMap::new(),
            due_date_stats: HashMap::new(),
            tag_stats: HashMap::new(),
            priority_status_matrix: HashMap::new(),
        };

        assert!(!has_priority_status_data(&stats));
    }

    // テーブル作成関数のテスト

    #[test]
    fn test_create_status_detail_table() {
        let mut status_stats = HashMap::new();
        status_stats.insert("pending".to_string(), 15);
        status_stats.insert("in_progress".to_string(), 12);
        status_stats.insert("completed".to_string(), 15);

        let stats = StatsDTO {
            total_count: 42,
            status_stats,
            priority_stats: HashMap::new(),
            due_date_stats: HashMap::new(),
            tag_stats: HashMap::new(),
            priority_status_matrix: HashMap::new(),
        };

        let table = create_status_detail_table(&stats);
        let output = table.to_string();

        // ヘッダーが含まれることを確認
        assert!(output.contains("Status"));
        assert!(output.contains("Count"));
        assert!(output.contains("Percentage"));
        assert!(output.contains("Progress"));

        // データが含まれることを確認
        assert!(output.contains("15"));
        assert!(output.contains("12"));
        assert!(output.contains("35.7%"));
        assert!(output.contains("28.6%"));

        // プログレスバーが含まれることを確認
        assert!(output.contains("█"));
        assert!(output.contains("░"));
    }

    #[test]
    fn test_create_status_detail_table_empty() {
        let stats = StatsDTO {
            total_count: 0,
            status_stats: HashMap::new(),
            priority_stats: HashMap::new(),
            due_date_stats: HashMap::new(),
            tag_stats: HashMap::new(),
            priority_status_matrix: HashMap::new(),
        };

        let table = create_status_detail_table(&stats);
        let output = table.to_string();

        // ヘッダーは存在するはず
        assert!(output.contains("Status"));
    }

    #[test]
    fn test_create_priority_status_matrix_table() {
        let mut priority_status_matrix = HashMap::new();
        priority_status_matrix.insert("critical:pending".to_string(), 3);
        priority_status_matrix.insert("critical:in_progress".to_string(), 2);
        priority_status_matrix.insert("critical:completed".to_string(), 1);
        priority_status_matrix.insert("high:pending".to_string(), 5);
        priority_status_matrix.insert("high:in_progress".to_string(), 4);
        priority_status_matrix.insert("high:completed".to_string(), 3);

        let stats = StatsDTO {
            total_count: 18,
            status_stats: HashMap::new(),
            priority_stats: HashMap::new(),
            due_date_stats: HashMap::new(),
            tag_stats: HashMap::new(),
            priority_status_matrix,
        };

        let table = create_priority_status_matrix_table(&stats);
        let output = table.to_string();

        // ヘッダーが含まれることを確認
        assert!(output.contains("Pending"));
        assert!(output.contains("In Progress"));
        assert!(output.contains("Completed"));
        assert!(output.contains("Total"));

        // 優先度ラベルが含まれることを確認
        assert!(output.contains("Critical"));
        assert!(output.contains("High"));

        // データが含まれることを確認
        assert!(output.contains("3"));
        assert!(output.contains("2"));
        assert!(output.contains("1"));
        assert!(output.contains("5"));
        assert!(output.contains("4"));
    }

    #[test]
    fn test_create_priority_status_matrix_table_partial_data() {
        let mut priority_status_matrix = HashMap::new();
        priority_status_matrix.insert("high:pending".to_string(), 5);

        let stats = StatsDTO {
            total_count: 5,
            status_stats: HashMap::new(),
            priority_stats: HashMap::new(),
            due_date_stats: HashMap::new(),
            tag_stats: HashMap::new(),
            priority_status_matrix,
        };

        let table = create_priority_status_matrix_table(&stats);
        let output = table.to_string();

        // ヘッダーと合計行が含まれることを確認
        assert!(output.contains("Pending"));
        assert!(output.contains("Total"));
    }

    #[test]
    fn test_create_top_tags_table_with_limit() {
        let mut tag_stats = HashMap::new();
        tag_stats.insert("重要".to_string(), 15);
        tag_stats.insert("バグ".to_string(), 12);
        tag_stats.insert("機能追加".to_string(), 10);
        tag_stats.insert("ドキュメント".to_string(), 8);
        tag_stats.insert("リファクタリング".to_string(), 5);
        tag_stats.insert("その他".to_string(), 3);

        let stats = StatsDTO {
            total_count: 53,
            status_stats: HashMap::new(),
            priority_stats: HashMap::new(),
            due_date_stats: HashMap::new(),
            tag_stats,
            priority_status_matrix: HashMap::new(),
        };

        let table = create_top_tags_table(&stats, 3);
        let output = table.to_string();

        // 上位3件のみ含まれることを確認
        assert!(output.contains("重要"));
        assert!(output.contains("バグ"));
        assert!(output.contains("機能追加"));
        assert!(!output.contains("その他"));
    }

    #[test]
    fn test_create_top_tags_table_fewer_than_limit() {
        let mut tag_stats = HashMap::new();
        tag_stats.insert("重要".to_string(), 15);
        tag_stats.insert("バグ".to_string(), 12);

        let stats = StatsDTO {
            total_count: 27,
            status_stats: HashMap::new(),
            priority_stats: HashMap::new(),
            due_date_stats: HashMap::new(),
            tag_stats,
            priority_status_matrix: HashMap::new(),
        };

        let table = create_top_tags_table(&stats, 5);
        let output = table.to_string();

        // 2件のみ含まれることを確認
        assert!(output.contains("重要"));
        assert!(output.contains("バグ"));
    }

    // 統合テスト

    #[test]
    fn test_create_rich_stats_display_full_data() {
        let mut status_stats = HashMap::new();
        status_stats.insert("pending".to_string(), 15);
        status_stats.insert("in_progress".to_string(), 12);
        status_stats.insert("completed".to_string(), 15);

        let mut priority_status_matrix = HashMap::new();
        priority_status_matrix.insert("critical:pending".to_string(), 3);
        priority_status_matrix.insert("critical:in_progress".to_string(), 2);
        priority_status_matrix.insert("critical:completed".to_string(), 1);

        let mut due_date_stats = HashMap::new();
        due_date_stats.insert("overdue".to_string(), 5);
        due_date_stats.insert("due_today".to_string(), 2);
        due_date_stats.insert("due_this_week".to_string(), 8);

        let mut tag_stats = HashMap::new();
        tag_stats.insert("重要".to_string(), 15);
        tag_stats.insert("バグ".to_string(), 12);

        let stats = StatsDTO {
            total_count: 42,
            status_stats,
            priority_stats: HashMap::new(),
            due_date_stats,
            tag_stats,
            priority_status_matrix,
        };

        let display = create_rich_stats_display(&stats);

        // タイトルが含まれることを確認
        assert!(display.contains("Task Statistics Summary"));

        // 各セクションが含まれることを確認
        assert!(display.contains("Total tasks: 42"));
        assert!(display.contains("[By Status]"));
        assert!(display.contains("[Priority × Status Matrix]"));
        assert!(display.contains("[Due Dates]"));
        assert!(display.contains("[Top Tags (Top 5)]"));

        // プログレスバーが含まれることを確認
        assert!(display.contains("█"));
        assert!(display.contains("░"));
    }

    #[test]
    fn test_create_rich_stats_display_minimal_data() {
        let stats = StatsDTO {
            total_count: 0,
            status_stats: HashMap::new(),
            priority_stats: HashMap::new(),
            due_date_stats: HashMap::new(),
            tag_stats: HashMap::new(),
            priority_status_matrix: HashMap::new(),
        };

        let display = create_rich_stats_display(&stats);

        // 最小限のセクションが含まれることを確認
        assert!(display.contains("Task Statistics Summary"));
        assert!(display.contains("Total tasks: 0"));
    }
}
