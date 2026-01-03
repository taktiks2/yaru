use crate::domain::task::{Priority, Status, Task};
use crate::repository::Repository;
use crate::repository::task::TaskRepository;
use anyhow::{Context, Result};
use chrono::Local;
use comfy_table::{Table, presets::UTF8_FULL};
use console::style;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

/// 期限の状況を表すEnum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DueDateStatus {
    Overdue,     // 期限切れ
    DueToday,    // 今日期限
    DueThisWeek, // 今週期限（7日以内）
    NoDueDate,   // 期限なし
}

/// 統計情報を格納する構造体
struct TaskStats {
    // HashMap化された統計
    status_stats: HashMap<Status, usize>,     // ステータス別統計
    priority_stats: HashMap<Priority, usize>, // 優先度別統計
    due_date_stats: HashMap<DueDateStatus, usize>, // 期限関連統計

    // 既存のHashMapベース統計
    tag_stats: HashMap<String, usize>,
    priority_status_matrix: HashMap<(Priority, Status), usize>,

    // 全体統計
    total_count: usize,
}

/// タスクの統計情報を表示
pub async fn show_stats(db: &DatabaseConnection) -> Result<()> {
    let task_repo = TaskRepository::new(db);

    let tasks = task_repo
        .find_all()
        .await
        .context("タスクの読み込みに失敗しました")?;

    let stats = calculate_stats(&tasks, Local::now().date_naive());

    // セクションごとに表示
    println!("\n{}", style("=== タスク統計 ===").bold().cyan());

    println!("\n{}", style("■ 全体サマリー").bold());
    println!("{}", create_summary_table(&stats));

    println!("\n{}", style("■ ステータス別").bold());
    println!("{}", create_status_table(&stats));

    println!("\n{}", style("■ 優先度別").bold());
    println!("{}", create_priority_table(&stats));

    println!("\n{}", style("■ 期限関連").bold());
    println!("{}", create_due_date_table(&stats));

    if !stats.tag_stats.is_empty() {
        println!("\n{}", style("■ タグ別統計").bold());
        println!("{}", create_tag_stats_table(&stats));
    }

    println!("\n{}", style("■ 優先度×ステータス クロス集計").bold());
    println!("{}", create_priority_status_matrix_table(&stats));

    Ok(())
}

/// タスクリストから統計情報を計算
fn calculate_stats(tasks: &[Task], today: chrono::NaiveDate) -> TaskStats {
    let total_count = tasks.len();

    let mut status_stats: HashMap<Status, usize> = HashMap::new();
    let mut priority_stats: HashMap<Priority, usize> = HashMap::new();
    let mut due_date_stats: HashMap<DueDateStatus, usize> = HashMap::new();
    let mut tag_stats: HashMap<String, usize> = HashMap::new();
    let mut priority_status_matrix: HashMap<(Priority, Status), usize> = HashMap::new();

    for task in tasks {
        // ステータス別カウント
        *status_stats.entry(task.status).or_default() += 1;

        // 優先度別カウント
        *priority_stats.entry(task.priority).or_default() += 1;

        // 期限関連カウント (完了済みタスクは除外)
        if task.status != Status::Completed {
            if let Some(due) = task.due_date {
                if due < today {
                    *due_date_stats.entry(DueDateStatus::Overdue).or_default() += 1;
                } else if due == today {
                    *due_date_stats.entry(DueDateStatus::DueToday).or_default() += 1;
                } else {
                    let week_later = today + chrono::Duration::days(7);
                    if due <= week_later {
                        *due_date_stats
                            .entry(DueDateStatus::DueThisWeek)
                            .or_default() += 1;
                    }
                }
            } else {
                *due_date_stats.entry(DueDateStatus::NoDueDate).or_default() += 1;
            }
        }

        // タグ別統計
        if task.tags.is_empty() {
            *tag_stats.entry("(タグなし)".to_string()).or_default() += 1;
        } else {
            for tag in &task.tags {
                *tag_stats.entry(tag.name.clone()).or_default() += 1;
            }
        }

        // 優先度×ステータス クロス集計
        *priority_status_matrix.entry((task.priority, task.status)).or_default() += 1;
    }

    TaskStats {
        status_stats,
        priority_stats,
        due_date_stats,
        tag_stats,
        priority_status_matrix,
        total_count,
    }
}

/// 全体サマリーテーブルを作成
fn create_summary_table(stats: &TaskStats) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["項目", "値"]);

    table.add_row(vec!["全タスク数", &stats.total_count.to_string()]);

    let completed_count = stats.status_stats.get(&Status::Completed).unwrap_or(&0);
    table.add_row(vec!["完了タスク数", &completed_count.to_string()]);
    table.add_row(vec![
        "完了率",
        &create_progress_bar(*completed_count, stats.total_count),
    ]);

    table
}

/// ステータス別テーブルを作成
fn create_status_table(stats: &TaskStats) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ステータス", "件数", "割合"]);

    let statuses = [
        (Status::Pending, "保留中"),
        (Status::InProgress, "進行中"),
        (Status::Completed, "完了"),
    ];

    for (status, label) in &statuses {
        let count = *stats.status_stats.get(status).unwrap_or(&0);
        table.add_row(vec![
            *label,
            &count.to_string(),
            &format!("{:.1}%", calculate_percentage(count, stats.total_count)),
        ]);
    }

    table
}

/// 優先度別テーブルを作成
fn create_priority_table(stats: &TaskStats) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["優先度", "件数", "割合"]);

    let priorities = [
        (Priority::Critical, "重大"),
        (Priority::High, "高"),
        (Priority::Medium, "中"),
        (Priority::Low, "低"),
    ];

    for (priority, label) in &priorities {
        let count = *stats.priority_stats.get(priority).unwrap_or(&0);
        table.add_row(vec![
            *label,
            &count.to_string(),
            &format!("{:.1}%", calculate_percentage(count, stats.total_count)),
        ]);
    }

    table
}

/// 期限関連テーブルを作成
fn create_due_date_table(stats: &TaskStats) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["期限状況", "件数"]);

    let due_statuses = [
        (DueDateStatus::Overdue, "期限切れ"),
        (DueDateStatus::DueToday, "今日期限"),
        (DueDateStatus::DueThisWeek, "今週期限(7日以内)"),
        (DueDateStatus::NoDueDate, "期限なし"),
    ];

    for (status, label) in &due_statuses {
        let count = *stats.due_date_stats.get(status).unwrap_or(&0);
        table.add_row(vec![*label, &count.to_string()]);
    }

    table
}

/// タグ別統計テーブルを作成
fn create_tag_stats_table(stats: &TaskStats) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["タグ名", "件数", "割合"]);

    let mut tag_vec: Vec<_> = stats.tag_stats.iter().collect();
    tag_vec.sort_by(|a, b| b.1.cmp(a.1)); // 件数の降順

    for (tag_name, count) in tag_vec {
        table.add_row(vec![
            tag_name,
            &count.to_string(),
            &format!("{:.1}%", calculate_percentage(*count, stats.total_count)),
        ]);
    }

    table
}

/// 優先度×ステータス クロス集計テーブルを作成
fn create_priority_status_matrix_table(stats: &TaskStats) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![
        "優先度 \\ ステータス",
        "保留中",
        "進行中",
        "完了",
        "合計",
    ]);

    let priorities = [
        Priority::Critical,
        Priority::High,
        Priority::Medium,
        Priority::Low,
    ];

    for &priority in &priorities {
        let pending = stats
            .priority_status_matrix
            .get(&(priority, Status::Pending))
            .unwrap_or(&0);
        let in_progress = stats
            .priority_status_matrix
            .get(&(priority, Status::InProgress))
            .unwrap_or(&0);
        let completed = stats
            .priority_status_matrix
            .get(&(priority, Status::Completed))
            .unwrap_or(&0);
        let total = pending + in_progress + completed;

        table.add_row(vec![
            &priority.to_string(),
            &pending.to_string(),
            &in_progress.to_string(),
            &completed.to_string(),
            &total.to_string(),
        ]);
    }

    table
}

/// パーセンテージを計算
///
/// # 引数
/// - `part`: 部分の数
/// - `total`: 全体の数
///
/// # 戻り値
/// パーセンテージ（0.0-100.0）
fn calculate_percentage(part: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (part as f64 / total as f64) * 100.0
    }
}

/// プログレスバーを作成
///
/// # 引数
/// - `current`: 現在の進捗
/// - `total`: 全体の数
///
/// # 戻り値
/// プログレスバーの文字列
fn create_progress_bar(current: usize, total: usize) -> String {
    if total == 0 {
        return "[░░░░░░░░░░] 0.0%".to_string();
    }

    let percentage = (current as f64 / total as f64) * 100.0;
    let filled = ((current as f64 / total as f64) * 10.0).round() as usize;
    let empty = 10 - filled;

    format!(
        "[{}{}] {:.1}%",
        "█".repeat(filled),
        "░".repeat(empty),
        percentage
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    /// 期限切れかどうか判定（期限 < 今日 かつ 未完了）
    fn is_overdue(task: &Task, today: &NaiveDate) -> bool {
        if task.status == Status::Completed {
            return false;
        }

        task.due_date.map(|due| due < *today).unwrap_or(false)
    }

    /// 今日期限かどうか判定
    fn is_due_today(task: &Task, today: &NaiveDate) -> bool {
        task.due_date.map(|due| due == *today).unwrap_or(false)
    }

    /// 今週期限かどうか判定（今日 < 期限 <= 今日+7日）
    fn is_due_this_week(task: &Task, today: &NaiveDate) -> bool {
        task.due_date
            .map(|due| {
                let week_later = *today + chrono::Duration::days(7);
                due > *today && due <= week_later
            })
            .unwrap_or(false)
    }

    #[test]
    fn test_calculate_stats_empty_tasks() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let tasks: Vec<Task> = vec![];
        let stats = calculate_stats(&tasks, today);

        assert_eq!(stats.total_count, 0);
        assert_eq!(stats.status_stats.get(&Status::Pending).unwrap_or(&0), &0);
        assert_eq!(
            stats.status_stats.get(&Status::InProgress).unwrap_or(&0),
            &0
        );
        assert_eq!(stats.status_stats.get(&Status::Completed).unwrap_or(&0), &0);
    }

    #[test]
    fn test_calculate_stats_status_counts() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let tasks = vec![
            Task::new(
                1,
                "タスク1",
                "",
                Status::Pending,
                Priority::Medium,
                vec![],
                None,
            ),
            Task::new(
                2,
                "タスク2",
                "",
                Status::InProgress,
                Priority::Medium,
                vec![],
                None,
            ),
            Task::new(
                3,
                "タスク3",
                "",
                Status::Completed,
                Priority::Medium,
                vec![],
                None,
            ),
            Task::new(
                4,
                "タスク4",
                "",
                Status::Pending,
                Priority::Medium,
                vec![],
                None,
            ),
        ];

        let stats = calculate_stats(&tasks, today);

        assert_eq!(stats.total_count, 4);
        assert_eq!(*stats.status_stats.get(&Status::Pending).unwrap(), 2);
        assert_eq!(*stats.status_stats.get(&Status::InProgress).unwrap(), 1);
        assert_eq!(*stats.status_stats.get(&Status::Completed).unwrap(), 1);
    }

    #[test]
    fn test_calculate_stats_priority_counts() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let tasks = vec![
            Task::new(
                1,
                "タスク1",
                "",
                Status::Pending,
                Priority::Low,
                vec![],
                None,
            ),
            Task::new(
                2,
                "タスク2",
                "",
                Status::Pending,
                Priority::Medium,
                vec![],
                None,
            ),
            Task::new(
                3,
                "タスク3",
                "",
                Status::Pending,
                Priority::High,
                vec![],
                None,
            ),
            Task::new(
                4,
                "タスク4",
                "",
                Status::Pending,
                Priority::Critical,
                vec![],
                None,
            ),
            Task::new(
                5,
                "タスク5",
                "",
                Status::Pending,
                Priority::Medium,
                vec![],
                None,
            ),
        ];

        let stats = calculate_stats(&tasks, today);

        assert_eq!(*stats.priority_stats.get(&Priority::Low).unwrap(), 1);
        assert_eq!(*stats.priority_stats.get(&Priority::Medium).unwrap(), 2);
        assert_eq!(*stats.priority_stats.get(&Priority::High).unwrap(), 1);
        assert_eq!(*stats.priority_stats.get(&Priority::Critical).unwrap(), 1);
    }

    #[test]
    fn test_calculate_stats_due_date_counts() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let tasks = vec![
            // 期限切れ
            Task::new(1, "t1", "", Status::Pending, Priority::Medium, vec![], Some(today - chrono::Duration::days(1))),
            // 今日が期限
            Task::new(2, "t2", "", Status::Pending, Priority::Medium, vec![], Some(today)),
            // 今週が期限
            Task::new(3, "t3", "", Status::Pending, Priority::Medium, vec![], Some(today + chrono::Duration::days(3))),
            // 今週が期限 (7日後)
            Task::new(4, "t4", "", Status::Pending, Priority::Medium, vec![], Some(today + chrono::Duration::days(7))),
            // 期限なし
            Task::new(5, "t5", "", Status::Pending, Priority::Medium, vec![], None),
            // 期限切れだが完了済み
            Task::new(6, "t6", "", Status::Completed, Priority::Medium, vec![], Some(today - chrono::Duration::days(1))),
            // 期限がまだ先
            Task::new(7, "t7", "", Status::Pending, Priority::Medium, vec![], Some(today + chrono::Duration::days(8))),
        ];

        let stats = calculate_stats(&tasks, today);

        assert_eq!(*stats.due_date_stats.get(&DueDateStatus::Overdue).unwrap_or(&0), 1);
        assert_eq!(*stats.due_date_stats.get(&DueDateStatus::DueToday).unwrap_or(&0), 1);
        assert_eq!(*stats.due_date_stats.get(&DueDateStatus::DueThisWeek).unwrap_or(&0), 2);
        assert_eq!(*stats.due_date_stats.get(&DueDateStatus::NoDueDate).unwrap_or(&0), 1);
        // 完了済みと期限がまだ先のタスクはカウントされない
        assert_eq!(stats.due_date_stats.values().sum::<usize>(), 5);
    }

    #[test]
    fn test_is_overdue() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();

        // 過去の期限、未完了 -> 期限切れ
        let task = Task::new(
            1,
            "過去の期限",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()),
        );
        assert!(is_overdue(&task, &today));

        // 今日の期限 -> 期限切れではない
        let task = Task::new(
            2,
            "今日の期限",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(today),
        );
        assert!(!is_overdue(&task, &today));

        // 未来の期限 -> 期限切れではない
        let task = Task::new(
            3,
            "未来の期限",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()),
        );
        assert!(!is_overdue(&task, &today));

        // 過去の期限、完了済み -> 期限切れではない
        let task = Task::new(
            4,
            "完了済み",
            "",
            Status::Completed,
            Priority::Medium,
            vec![],
            Some(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()),
        );
        assert!(!is_overdue(&task, &today));

        // 期限なし -> 期限切れではない
        let task = Task::new(
            5,
            "期限なし",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        assert!(!is_overdue(&task, &today));
    }

    #[test]
    fn test_is_due_today() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();

        // 今日の期限
        let task = Task::new(
            1,
            "今日の期限",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(today),
        );
        assert!(is_due_today(&task, &today));

        // 過去の期限
        let task = Task::new(
            2,
            "過去の期限",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()),
        );
        assert!(!is_due_today(&task, &today));

        // 未来の期限
        let task = Task::new(
            3,
            "未来の期限",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()),
        );
        assert!(!is_due_today(&task, &today));

        // 期限なし
        let task = Task::new(
            4,
            "期限なし",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        assert!(!is_due_today(&task, &today));
    }

    #[test]
    fn test_is_due_this_week() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();

        // 今週の期限（3日後）
        let task = Task::new(
            1,
            "今週の期限",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(NaiveDate::from_ymd_opt(2026, 1, 18).unwrap()),
        );
        assert!(is_due_this_week(&task, &today));

        // 今週の期限（ちょうど7日後）
        let task = Task::new(
            2,
            "7日後",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(NaiveDate::from_ymd_opt(2026, 1, 22).unwrap()),
        );
        assert!(is_due_this_week(&task, &today));

        // 今日の期限 -> 今週には含まない
        let task = Task::new(
            3,
            "今日の期限",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(today),
        );
        assert!(!is_due_this_week(&task, &today));

        // 8日後 -> 今週ではない
        let task = Task::new(
            4,
            "8日後",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(NaiveDate::from_ymd_opt(2026, 1, 23).unwrap()),
        );
        assert!(!is_due_this_week(&task, &today));

        // 過去の期限 -> 今週ではない
        let task = Task::new(
            5,
            "過去の期限",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            Some(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()),
        );
        assert!(!is_due_this_week(&task, &today));

        // 期限なし -> 今週ではない
        let task = Task::new(
            6,
            "期限なし",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        assert!(!is_due_this_week(&task, &today));
    }

    // パーセンテージ計算のテスト
    #[test]
    fn test_calculate_percentage() {
        assert_eq!(calculate_percentage(5, 10), 50.0);
        assert_eq!(calculate_percentage(0, 10), 0.0);
        assert_eq!(calculate_percentage(10, 10), 100.0);
        assert_eq!(calculate_percentage(0, 0), 0.0);
    }

    // プログレスバーのテスト
    #[test]
    fn test_create_progress_bar_full() {
        let bar = create_progress_bar(10, 10);
        assert!(bar.contains("██████████"));
        assert!(bar.contains("100.0%"));
    }

    #[test]
    fn test_create_progress_bar_half() {
        let bar = create_progress_bar(5, 10);
        assert!(bar.contains("█████"));
        assert!(bar.contains("░░░░░"));
        assert!(bar.contains("50.0%"));
    }

    #[test]
    fn test_create_progress_bar_empty() {
        let bar = create_progress_bar(0, 10);
        assert!(bar.contains("░░░░░░░░░░"));
        assert!(bar.contains("0.0%"));
    }

    #[test]
    fn test_create_progress_bar_zero_total() {
        let bar = create_progress_bar(0, 0);
        assert!(bar.contains("0.0%"));
    }

    // タグ別統計のテスト
    #[test]
    fn test_calculate_tag_stats() {
        use crate::domain::tag::Tag;

        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let tag1 = Tag::new(1, "重要", "");
        let tag2 = Tag::new(2, "緊急", "");
        let tasks = vec![
            Task::new(
                1,
                "タスク1",
                "",
                Status::Pending,
                Priority::Medium,
                vec![tag1.clone(), tag2.clone()],
                None,
            ),
            Task::new(
                2,
                "タスク2",
                "",
                Status::Pending,
                Priority::Medium,
                vec![tag1.clone()],
                None,
            ),
        ];

        let stats = calculate_stats(&tasks, today);
        assert_eq!(stats.tag_stats.get("重要"), Some(&2));
        assert_eq!(stats.tag_stats.get("緊急"), Some(&1));
    }

    #[test]
    fn test_calculate_tag_stats_empty() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let tasks: Vec<Task> = vec![];
        let stats = calculate_stats(&tasks, today);
        assert!(stats.tag_stats.is_empty());
    }

    #[test]
    fn test_calculate_tag_stats_no_tags() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let tasks = vec![Task::new(
            1,
            "タスク1",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        )];

        let stats = calculate_stats(&tasks, today);
        assert_eq!(stats.tag_stats.get("(タグなし)"), Some(&1));
    }

    // クロス集計のテスト
    #[test]
    fn test_calculate_priority_status_matrix() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let tasks = vec![
            Task::new(
                1,
                "タスク1",
                "",
                Status::Pending,
                Priority::High,
                vec![],
                None,
            ),
            Task::new(
                2,
                "タスク2",
                "",
                Status::Pending,
                Priority::High,
                vec![],
                None,
            ),
            Task::new(
                3,
                "タスク3",
                "",
                Status::Completed,
                Priority::High,
                vec![],
                None,
            ),
            Task::new(
                4,
                "タスク4",
                "",
                Status::InProgress,
                Priority::Medium,
                vec![],
                None,
            ),
        ];

        let stats = calculate_stats(&tasks, today);
        assert_eq!(stats.priority_status_matrix.get(&(Priority::High, Status::Pending)), Some(&2));
        assert_eq!(stats.priority_status_matrix.get(&(Priority::High, Status::Completed)), Some(&1));
        assert_eq!(
            stats.priority_status_matrix.get(&(Priority::Medium, Status::InProgress)),
            Some(&1)
        );
    }
}
