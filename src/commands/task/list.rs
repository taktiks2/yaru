use crate::{
    cli::{Filter, FilterKey},
    display::create_task_table,
    repository::Repository,
    task::{Status, Task},
};
use anyhow::Result;

/// 全てのタスクを一覧表示
pub fn list_tasks(repo: &impl Repository<Task>, filters: Option<Vec<Filter>>) -> Result<()> {
    let mut tasks = repo.load()?;

    // フィルタリングを適用
    if let Some(filters) = filters {
        for filter in filters {
            tasks = apply_filter(tasks, &filter)?;
        }
    }

    if tasks.is_empty() {
        println!("タスクはありません。");
        return Ok(());
    }

    let table = create_task_table(&tasks);
    println!("{table}");

    Ok(())
}

/// フィルタを適用してタスクリストを絞り込む
fn apply_filter(tasks: Vec<Task>, filter: &Filter) -> Result<Vec<Task>> {
    match filter.key {
        FilterKey::Status => {
            let status = Status::from_filter_value(&filter.value)
                .map_err(|_| anyhow::anyhow!("無効なステータス値です: {}", &filter.value))?;
            Ok(tasks
                .into_iter()
                .filter(|task| task.status == status)
                .collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::JsonRepository;
    use crate::task::{Priority, Status, Task};
    use tempfile::TempDir;

    fn setup_test_repo() -> (TempDir, JsonRepository<Task>) {
        let temp_dir = tempfile::tempdir().unwrap();
        let task_file = temp_dir.path().join("tasks.json");
        let repo = JsonRepository::new(&task_file);
        repo.ensure_data_exists().unwrap();
        (temp_dir, repo)
    }

    #[test]
    fn test_list_tasks_empty() {
        let (_temp_dir, repo) = setup_test_repo();

        // 空のリストを表示
        let result = list_tasks(&repo, None);
        assert!(result.is_ok());

        // タスクが存在しないことを確認
        let tasks = repo.load().unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_list_tasks_with_items() {
        let (_temp_dir, repo) = setup_test_repo();

        // タスクを事前に作成
        let tasks = vec![
            Task::new(
                1,
                "タスク1",
                "説明1",
                Status::Pending,
                Priority::High,
                vec![],
            ),
            Task::new(
                2,
                "タスク2",
                "説明2",
                Status::Completed,
                Priority::Medium,
                vec![],
            ),
        ];
        repo.save(&tasks).unwrap();

        // タスクを表示
        let result = list_tasks(&repo, None);
        assert!(result.is_ok());

        // 保存されたタスクが2件あることを確認
        let loaded_tasks = repo.load().unwrap();
        assert_eq!(loaded_tasks.len(), 2);
    }

    #[test]
    fn test_list_tasks_with_status_filter_pending() {
        let (_temp_dir, repo) = setup_test_repo();

        // 異なるステータスのタスクを作成
        let tasks = vec![
            Task::new(1, "保留中", "", Status::Pending, Priority::Medium, vec![]),
            Task::new(2, "完了", "", Status::Completed, Priority::Medium, vec![]),
            Task::new(
                3,
                "進行中",
                "",
                Status::InProgress,
                Priority::Medium,
                vec![],
            ),
        ];
        repo.save(&tasks).unwrap();

        // Pendingフィルタを適用
        let filters = vec![Filter {
            key: FilterKey::Status,
            value: "pending".to_string(),
        }];
        let result = list_tasks(&repo, Some(filters));
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_tasks_with_status_filter_completed() {
        let (_temp_dir, repo) = setup_test_repo();

        // 異なるステータスのタスクを作成
        let tasks = vec![
            Task::new(1, "保留中", "", Status::Pending, Priority::Medium, vec![]),
            Task::new(2, "完了", "", Status::Completed, Priority::Medium, vec![]),
        ];
        repo.save(&tasks).unwrap();

        // Completedフィルタを適用
        let filters = vec![Filter {
            key: FilterKey::Status,
            value: "completed".to_string(),
        }];
        let result = list_tasks(&repo, Some(filters));
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_tasks_with_invalid_status_filter() {
        let (_temp_dir, repo) = setup_test_repo();

        // タスクを作成
        let tasks = vec![Task::new(
            1,
            "タスク",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
        )];
        repo.save(&tasks).unwrap();

        // 無効なステータスフィルタを適用
        let filters = vec![Filter {
            key: FilterKey::Status,
            value: "invalid_status".to_string(),
        }];
        let result = list_tasks(&repo, Some(filters));

        // エラーが返されることを確認
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("無効なステータス値です")
        );
    }

    #[test]
    fn test_apply_filter_status_pending() {
        let tasks = vec![
            Task::new(1, "保留中", "", Status::Pending, Priority::Medium, vec![]),
            Task::new(2, "完了", "", Status::Completed, Priority::Medium, vec![]),
            Task::new(
                3,
                "進行中",
                "",
                Status::InProgress,
                Priority::Medium,
                vec![],
            ),
        ];

        let filter = Filter {
            key: FilterKey::Status,
            value: "pending".to_string(),
        };

        let filtered = apply_filter(tasks, &filter).unwrap();

        // Pendingのタスクのみが抽出されることを確認
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].status, Status::Pending);
        assert_eq!(filtered[0].title, "保留中");
    }

    #[test]
    fn test_apply_filter_status_completed() {
        let tasks = vec![
            Task::new(1, "保留中", "", Status::Pending, Priority::Medium, vec![]),
            Task::new(2, "完了1", "", Status::Completed, Priority::Medium, vec![]),
            Task::new(3, "完了2", "", Status::Completed, Priority::High, vec![]),
        ];

        let filter = Filter {
            key: FilterKey::Status,
            value: "completed".to_string(),
        };

        let filtered = apply_filter(tasks, &filter).unwrap();

        // Completedのタスクのみが抽出されることを確認
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|t| t.status == Status::Completed));
    }

    #[test]
    fn test_apply_filter_status_in_progress() {
        let tasks = vec![
            Task::new(1, "保留中", "", Status::Pending, Priority::Medium, vec![]),
            Task::new(
                2,
                "進行中",
                "",
                Status::InProgress,
                Priority::Medium,
                vec![],
            ),
        ];

        let filter = Filter {
            key: FilterKey::Status,
            value: "in_progress".to_string(),
        };

        let filtered = apply_filter(tasks, &filter).unwrap();

        // InProgressのタスクのみが抽出されることを確認
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].status, Status::InProgress);
    }

    #[test]
    fn test_apply_filter_invalid_status() {
        let tasks = vec![Task::new(
            1,
            "タスク",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
        )];

        let filter = Filter {
            key: FilterKey::Status,
            value: "invalid".to_string(),
        };

        let result = apply_filter(tasks, &filter);

        // エラーが返されることを確認
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("無効なステータス値です")
        );
    }

    #[test]
    fn test_apply_filter_no_matching_tasks() {
        let tasks = vec![Task::new(
            1,
            "保留中",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
        )];

        let filter = Filter {
            key: FilterKey::Status,
            value: "completed".to_string(),
        };

        let filtered = apply_filter(tasks, &filter).unwrap();

        // マッチするタスクがないことを確認
        assert_eq!(filtered.len(), 0);
    }
}
