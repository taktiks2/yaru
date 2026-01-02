use crate::{
    display::format::{
        format_date, format_local_time, format_optional_datetime, format_optional_text,
        format_tags, truncate_text,
    },
    domain::tag::Tag,
    domain::task::Task,
};
use comfy_table::{Table, presets::UTF8_FULL};

/// タスクのテーブルを作成
pub fn create_task_table(tasks: &[Task]) -> Table {
    let headers = vec![
        "ID",
        "タイトル",
        "説明",
        "ステータス",
        "優先度",
        "タグ",
        "期限",
        "完了日時",
        "作成日",
        "更新日",
    ];

    let rows: Vec<Vec<String>> = tasks.iter().map(create_task_row).collect();

    build_table_with_preset(headers, rows)
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
/// - `task`: タスク
///
/// # 戻り値
/// タスクの1行分のデータ（文字列のベクタ）
fn create_task_row(task: &Task) -> Vec<String> {
    let tags_str = format_tags(&task.tags, ",");
    let description = truncate_text(&format_optional_text(&task.description), 20);
    let due_date_str = format_date(&task.due_date);
    let completed_at_str = format_optional_datetime(task.completed_at);

    vec![
        task.id.to_string(),
        truncate_text(&task.title, 20),
        description,
        task.status.to_string(),
        task.priority.to_string(),
        tags_str,
        due_date_str,
        completed_at_str,
        format_local_time(&task.created_at),
        format_local_time(&task.updated_at),
    ]
}

/// タグの1行分のデータを作成
///
/// # 引数
/// - `tag`: タグ
///
/// # 戻り値
/// タグの1行分のデータ（文字列のベクタ）
fn create_tag_row(tag: &Tag) -> Vec<String> {
    let description = truncate_text(&format_optional_text(&tag.description), 20);

    vec![
        tag.id.to_string(),
        truncate_text(&tag.name, 20),
        description,
        format_local_time(&tag.created_at),
        format_local_time(&tag.updated_at),
    ]
}

/// タグのテーブルを作成
///
/// # 引数
/// - `tags`: 表示するタグのスライス
///
/// # 戻り値
/// UTF8_FULLプリセットが適用されたタグのテーブル
pub fn create_tag_table(tags: &[Tag]) -> Table {
    let headers = vec!["ID", "名前", "説明", "作成日", "更新日"];

    let rows: Vec<Vec<String>> = tags.iter().map(create_tag_row).collect();

    build_table_with_preset(headers, rows)
}

/// タグの詳細をキー・バリュー形式で表示
///
/// # 引数
/// - `tag`: 表示するタグ
///
/// # 戻り値
/// 2列のキー・バリュー形式テーブル
pub fn create_tag_detail_table(tag: &Tag) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    let description = format_optional_text(&tag.description);

    table.add_row(vec!["ID", &tag.id.to_string()]);
    table.add_row(vec!["名前", &tag.name]);
    table.add_row(vec!["説明", &description]);
    table.add_row(vec!["作成日", &format_local_time(&tag.created_at)]);
    table.add_row(vec!["更新日", &format_local_time(&tag.updated_at)]);

    table
}

/// タスクの詳細をキー・バリュー形式で表示
///
/// # 引数
/// - `task`: 表示するタスク
///
/// # 戻り値
/// 2列のキー・バリュー形式テーブル
/// Displayのfmtの表示内容をそのまま使用するため、clippyの警告を抑制
#[allow(clippy::unnecessary_to_owned)]
pub fn create_task_detail_table(task: &Task) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    let description = format_optional_text(&task.description);
    let tags_str = format_tags(&task.tags, ", ");
    let due_date_str = format_date(&task.due_date);
    let completed_at_str = format_optional_datetime(task.completed_at);

    table.add_row(vec!["ID", &task.id.to_string()]);
    table.add_row(vec!["タイトル", &task.title]);
    table.add_row(vec!["説明", &description]);
    table.add_row(vec!["ステータス", &task.status.to_string()]);
    table.add_row(vec!["優先度", &task.priority.to_string()]);
    table.add_row(vec!["タグ", &tags_str]);
    table.add_row(vec!["期限", &due_date_str]);
    table.add_row(vec!["完了日時", &completed_at_str]);
    table.add_row(vec!["作成日", &format_local_time(&task.created_at)]);
    table.add_row(vec!["更新日", &format_local_time(&task.updated_at)]);

    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::{Priority, Status};

    #[test]
    fn test_create_task_table_empty() {
        let tasks: Vec<Task> = vec![];
        let table = create_task_table(&tasks);

        // ヘッダーのみ存在することを確認
        let table_str = table.to_string();
        assert!(table_str.contains("ID"));
        assert!(table_str.contains("タイトル"));
        assert!(table_str.contains("ステータス"));
    }

    #[test]
    fn test_create_task_table_with_tasks() {
        let tasks = vec![
            Task::new(
                1,
                "テストタスク1",
                "",
                Status::Pending,
                Priority::Medium,
                vec![],
                None,
            ),
            Task::new(
                2,
                "テストタスク2",
                "",
                Status::Completed,
                Priority::Medium,
                vec![],
                None,
            ),
        ];
        let table = create_task_table(&tasks);

        let table_str = table.to_string();
        assert!(table_str.contains("1"));
        assert!(table_str.contains("テストタスク1"));
        assert!(table_str.contains("2"));
        assert!(table_str.contains("テストタスク2"));
    }

    #[test]
    fn test_create_task_table_with_different_statuses() {
        let tasks = vec![
            Task::new(
                1,
                "保留中タスク",
                "",
                Status::Pending,
                Priority::Medium,
                vec![],
                None,
            ),
            Task::new(
                2,
                "進行中タスク",
                "",
                Status::InProgress,
                Priority::Medium,
                vec![],
                None,
            ),
            Task::new(
                3,
                "完了タスク",
                "",
                Status::Completed,
                Priority::Medium,
                vec![],
                None,
            ),
        ];
        let table = create_task_table(&tasks);

        let table_str = table.to_string();
        assert!(table_str.contains("保留中"));
        assert!(table_str.contains("進行中"));
        assert!(table_str.contains("完了"));
    }

    #[test]
    fn test_create_task_table_includes_description() {
        // テーブルにdescription列が含まれていることを確認
        let tasks = vec![Task::new(
            1,
            "タスク1",
            "これは説明文です",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        )];
        let table = create_task_table(&tasks);

        let table_str = table.to_string();
        assert!(table_str.contains("説明"));
        assert!(table_str.contains("これは説明文です"));
    }

    #[test]
    fn test_create_task_table_truncates_long_description() {
        // 長い説明文が切り詰められることを確認
        let long_desc = "これは非常に長い説明文です。この説明文は30文字を超えているため切り詰められるはずです。さらに長くしています。";
        let tasks = vec![Task::new(
            1,
            "タスク",
            long_desc,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        )];
        let table = create_task_table(&tasks);

        let table_str = table.to_string();
        // 切り詰められた説明文が含まれている
        assert!(table_str.contains("..."));
        // 元の長い説明文がそのまま含まれていないことを確認
        assert!(!table_str.contains(long_desc));
    }

    #[test]
    fn test_task_table_uses_utf8_full_preset() {
        // タスクテーブルにもUTF8_FULLが適用されることを確認
        let tasks = vec![Task::new(
            1,
            "テストタスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        )];
        let table = create_task_table(&tasks);

        let table_str = table.to_string();
        // UTF8_FULLプリセットの特徴的な罫線文字が含まれることを確認
        assert!(table_str.contains("─") || table_str.contains("│"));
    }

    #[test]
    fn test_create_task_table_empty_description_shows_dash() {
        // 空の説明は"-"と表示されることを確認
        let tasks = vec![Task::new(
            1,
            "タスク",
            "",
            Status::Pending,
            Priority::Medium,
            vec![Tag::new(1, "タグ", "")], // タグありにして、空文字列の"-"と区別
            None,
        )];
        let _table = create_task_table(&tasks);

        // create_task_rowの動作を直接確認
        let row = create_task_row(&tasks[0]);
        // description列（インデックス2）が"-"であることを確認
        assert_eq!(row[2], "-");
    }

    #[test]
    fn test_create_task_table_empty_tags_shows_dash() {
        // 空のタグリストは"-"と表示されることを確認
        let tasks = vec![Task::new(
            1,
            "タスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        )];
        let table = create_task_table(&tasks);

        let table_str = table.to_string();
        // タグが空の場合"-"として表示される
        let rows: Vec<&str> = table_str.lines().collect();
        let data_row = rows.iter().find(|line| line.contains("タスク")).unwrap();
        assert!(data_row.contains("-"));
    }
}

#[cfg(test)]
mod tests_tag {
    use super::*;
    use crate::domain::tag::Tag;

    #[test]
    fn test_create_tag_table_empty() {
        let tags: Vec<Tag> = vec![];
        let table = create_tag_table(&tags);

        // ヘッダーのみ存在することを確認
        let table_str = table.to_string();
        assert!(table_str.contains("ID"));
        assert!(table_str.contains("名前"));
        assert!(table_str.contains("説明"));
    }

    #[test]
    fn test_create_tag_table_with_tags() {
        let tags = vec![
            Tag::new(1, "重要", "重要なタスク"),
            Tag::new(2, "作業中", "現在作業中"),
        ];
        let table = create_tag_table(&tags);

        let table_str = table.to_string();
        assert!(table_str.contains("1"));
        assert!(table_str.contains("重要"));
        assert!(table_str.contains("2"));
        assert!(table_str.contains("作業中"));
    }

    #[test]
    fn test_create_tag_table_includes_all_columns() {
        let tags = vec![Tag::new(1, "テスト", "テスト説明")];
        let table = create_tag_table(&tags);

        let table_str = table.to_string();
        // すべての列が含まれることを確認
        assert!(table_str.contains("ID"));
        assert!(table_str.contains("名前"));
        assert!(table_str.contains("説明"));
        assert!(table_str.contains("作成日"));
        assert!(table_str.contains("更新日"));
    }

    #[test]
    fn test_create_tag_table_truncates_long_description() {
        // 長い説明が切り詰められることを確認
        let long_desc =
            "これは非常に長い説明文です。タグの説明は切り詰められずに全文表示されるべきです。";
        let tags = vec![Tag::new(1, "タグ", long_desc)];
        let table = create_tag_table(&tags);

        let table_str = table.to_string();
        // 切り詰められた説明文が含まれている
        assert!(table_str.contains("..."));
        // 元の長い説明文がそのまま含まれていないことを確認
        assert!(!table_str.contains(long_desc));
    }

    #[test]
    fn test_tag_table_uses_utf8_full_preset() {
        // UTF8_FULLプリセットが適用されていることを確認（外観テスト）
        let tags = vec![Tag::new(1, "テスト", "説明")];
        let table = create_tag_table(&tags);

        let table_str = table.to_string();
        // UTF8_FULLプリセットの特徴的な文字が含まれることを確認
        // UTF8_FULLは罫線文字を使用する
        assert!(table_str.contains("─") || table_str.contains("│"));
    }

    #[test]
    fn test_create_tag_table_empty_description_shows_dash() {
        // 空の説明は"-"と表示されることを確認
        let tags = vec![Tag::new(1, "タグ", "")];
        let _table = create_tag_table(&tags);

        // create_tag_rowの動作を直接確認
        let row = create_tag_row(&tags[0]);
        // description列（インデックス2）が"-"であることを確認
        assert_eq!(row[2], "-");
    }
}

#[cfg(test)]
mod tests_task_with_tags {
    use super::*;
    use crate::domain::task::{Priority, Status};

    #[test]
    fn test_create_task_row_displays_tag_names() {
        let tag1 = Tag::new(1, "重要", "");
        let tag2 = Tag::new(2, "緊急", "");

        let task = Task::new(
            1,
            "タスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![tag1, tag2],
            None,
        );

        let row = create_task_row(&task);

        // タグ列にタグ名が含まれることを期待
        assert!(row[5].contains("重要"));
        assert!(row[5].contains("緊急"));
        // IDは含まれないことを確認
        assert!(!row[5].contains("1"));
    }

    #[test]
    fn test_create_task_detail_table_without_all_tags_parameter() {
        let tag = Tag::new(1, "テスト", "説明");
        let task = Task::new(
            1,
            "タスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![tag],
            None,
        );

        // all_tagsパラメータなしで呼び出せることを確認
        let table = create_task_detail_table(&task);
        let table_str = table.to_string();

        assert!(table_str.contains("テスト"));
    }
}
