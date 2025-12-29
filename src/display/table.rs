use crate::{
    display::format::{format_local_time, truncate_text},
    task::Task,
};
use comfy_table::Table;

/// タスクのテーブルを作成
pub fn create_task_table(tasks: &[Task]) -> Table {
    let mut table = Table::new();
    table.set_header(vec![
        "ID",
        "タイトル",
        "説明",
        "ステータス",
        "優先度",
        "タグ",
        "作成日",
        "更新日",
    ]);

    for task in tasks {
        let tags_str = if task.tags.is_empty() {
            "-".to_string()
        } else {
            task.tags
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        };

        table.add_row(vec![
            task.id.to_string(),
            truncate_text(&task.title, 20),
            truncate_text(&task.description, 20),
            task.status.to_string(),
            task.priority.to_string(),
            tags_str,
            format_local_time(&task.created_at),
            format_local_time(&task.updated_at),
        ]);
    }

    table
}

/// 単一のタスクをテーブルとして表示
pub fn create_single_task_table(task: &Task) -> Table {
    let mut table = Table::new();
    table.set_header(vec![
        "ID",
        "タイトル",
        "説明",
        "ステータス",
        "優先度",
        "タグ",
        "作成日",
        "更新日",
    ]);

    let tags_str = if task.tags.is_empty() {
        "-".to_string()
    } else {
        task.tags
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",")
    };

    table.add_row(vec![
        task.id.to_string(),
        truncate_text(&task.title, 20),
        truncate_text(&task.description, 20),
        task.status.to_string(),
        task.priority.to_string(),
        tags_str,
        format_local_time(&task.created_at),
        format_local_time(&task.updated_at),
    ]);

    table
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
}
