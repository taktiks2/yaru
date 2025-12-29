use crate::{
    display::format::{format_local_time, truncate_text},
    tag::Tag,
    task::Task,
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
        "作成日",
        "更新日",
    ];

    let rows: Vec<Vec<String>> = tasks.iter().map(create_task_row).collect();

    build_table_with_preset(headers, rows)
}

/// 単一のタスクをテーブルとして表示
pub fn create_single_task_table(task: &Task) -> Table {
    let headers = vec![
        "ID",
        "タイトル",
        "説明",
        "ステータス",
        "優先度",
        "タグ",
        "作成日",
        "更新日",
    ];

    let rows = vec![create_task_row(task)];

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
    let tags_str = if task.tags.is_empty() {
        "-".to_string()
    } else {
        task.tags
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",")
    };

    let description = if task.description.is_empty() {
        "-".to_string()
    } else {
        truncate_text(&task.description, 20)
    };

    vec![
        task.id.to_string(),
        truncate_text(&task.title, 20),
        description,
        task.status.to_string(),
        task.priority.to_string(),
        tags_str,
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
    let description = if tag.description.is_empty() {
        "-".to_string()
    } else {
        truncate_text(&tag.description, 20)
    };

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

/// 単一のタグをテーブルとして表示
///
/// # 引数
/// - `tag`: 表示するタグ
///
/// # 戻り値
/// UTF8_FULLプリセットが適用されたタグのテーブル
pub fn create_single_tag_table(tag: &Tag) -> Table {
    let headers = vec!["ID", "名前", "説明", "作成日", "更新日"];

    let rows = vec![create_tag_row(tag)];

    build_table_with_preset(headers, rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{Priority, Status};

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
            ),
            Task::new(
                2,
                "テストタスク2",
                "",
                Status::Completed,
                Priority::Medium,
                vec![],
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
    fn test_create_single_task_table() {
        let task = Task::new(
            1,
            "新しいタスク",
            "",
            Status::InProgress,
            Priority::Medium,
            vec![],
        );
        let table = create_single_task_table(&task);

        let table_str = table.to_string();
        assert!(table_str.contains("1"));
        assert!(table_str.contains("新しいタスク"));
        assert!(table_str.contains("進行中"));
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
            ),
            Task::new(
                2,
                "進行中タスク",
                "",
                Status::InProgress,
                Priority::Medium,
                vec![],
            ),
            Task::new(
                3,
                "完了タスク",
                "",
                Status::Completed,
                Priority::Medium,
                vec![],
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
            vec![1], // タグありにして、空文字列の"-"と区別
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
    use crate::tag::Tag;

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
    fn test_create_single_tag_table() {
        let tag = Tag::new(1, "新しいタグ", "新規タグの説明");
        let table = create_single_tag_table(&tag);

        let table_str = table.to_string();
        assert!(table_str.contains("1"));
        assert!(table_str.contains("新しいタグ"));
        assert!(table_str.contains("新規タグの説明"));
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
