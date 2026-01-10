use crate::{
    application::dto::task_dto::TaskDTO,
    interface::cli::display::format::{
        format_date, format_local_time, format_optional_datetime, format_optional_text,
        format_tags, truncate_text,
    },
};
use comfy_table::{Table, presets::UTF8_FULL};

/// タスクのテーブルを作成
pub fn create_task_table(tasks: &[TaskDTO]) -> Table {
    let headers = vec![
        "ID",
        "Title",
        "Description",
        "Status",
        "Priority",
        "Tags",
        "Due Date",
        "Completed At",
        "Created At",
        "Updated At",
    ];

    let rows: Vec<Vec<String>> = tasks.iter().map(create_task_row).collect();

    build_table_with_preset(headers, rows)
}

/// タスクの詳細テーブルを作成
pub fn create_task_detail_table(task: &TaskDTO) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    table.add_row(vec!["ID", &task.id.to_string()]);
    table.add_row(vec!["Title", &task.title]);
    table.add_row(vec![
        "Description",
        &format_optional_text(&task.description),
    ]);
    table.add_row(vec!["Status", &task.status]);
    table.add_row(vec!["Priority", &task.priority]);
    table.add_row(vec!["Tags", &format_tags(&task.tags, ", ")]);
    table.add_row(vec!["Due Date", &format_date(&task.due_date)]);
    table.add_row(vec![
        "Completed At",
        &format_optional_datetime(&task.completed_at),
    ]);
    table.add_row(vec!["Created At", &format_local_time(&task.created_at)]);
    table.add_row(vec!["Updated At", &format_local_time(&task.updated_at)]);

    table
}

/// テーブルの基本構造を作成し、行データを追加
///
/// # 引数
/// - `headers`: テーブルのヘッダー
/// - `rows`: テーブルの行データ
///
/// # 戻り値
/// UTF8_FULLプリセットが適用されたテーブル
fn build_table_with_preset(headers: Vec<&str>, rows: Vec<Vec<String>>) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(headers);

    table.add_rows(rows);

    table
}

/// タスクの1行分のデータを作成
///
/// # 引数
/// - `task`: タスクDTO
///
/// # 戻り値
/// タスクの1行分のデータ（文字列のベクタ）
fn create_task_row(task: &TaskDTO) -> Vec<String> {
    let tags_str = format_tags(&task.tags, ",");
    let description = truncate_text(&format_optional_text(&task.description), 20);
    let due_date_str = format_date(&task.due_date);
    let completed_at_str = format_optional_datetime(&task.completed_at);

    vec![
        task.id.to_string(),
        truncate_text(&task.title, 20),
        description,
        task.status.clone(),
        task.priority.clone(),
        tags_str,
        due_date_str,
        completed_at_str,
        format_local_time(&task.created_at),
        format_local_time(&task.updated_at),
    ]
}
