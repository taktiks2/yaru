use crate::{repository::Repository, task::Task};
use anyhow::Result;

/// 指定されたIDのタスクを削除
pub fn delete_task(repo: &impl Repository<Task>, id: u64) -> Result<()> {
    let mut tasks = repo.load()?;
    let initial_count = tasks.len();
    tasks.retain(|task| task.id != id);

    if initial_count == tasks.len() {
        println!("ID {} のタスクが見つかりませんでした。", id);
        return Ok(());
    }

    repo.save(&tasks)?;
    println!("タスクを削除しました。");
    Ok(())
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
    fn test_delete_task_existing_id() {
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

        // ID=1のタスクを削除
        let result = delete_task(&repo, 1);
        assert!(result.is_ok());

        // タスクが1件削除されていることを確認
        let remaining_tasks = repo.load().unwrap();
        assert_eq!(remaining_tasks.len(), 1);
        assert_eq!(remaining_tasks[0].id, 2);
        assert_eq!(remaining_tasks[0].title, "タスク2");
    }

    #[test]
    fn test_delete_task_non_existing_id() {
        let (_temp_dir, repo) = setup_test_repo();

        // タスクを事前に作成
        let tasks = vec![Task::new(
            1,
            "タスク1",
            "説明1",
            Status::Pending,
            Priority::High,
            vec![],
        )];
        repo.save(&tasks).unwrap();

        // 存在しないID=999を削除しようとする
        let result = delete_task(&repo, 999);
        assert!(result.is_ok());

        // タスクは削除されずに残っていることを確認
        let remaining_tasks = repo.load().unwrap();
        assert_eq!(remaining_tasks.len(), 1);
        assert_eq!(remaining_tasks[0].id, 1);
    }

    #[test]
    fn test_delete_task_empty_list() {
        let (_temp_dir, repo) = setup_test_repo();

        // 空のリストからタスクを削除しようとする
        let result = delete_task(&repo, 1);
        assert!(result.is_ok());

        // タスクが存在しないことを確認
        let tasks = repo.load().unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_delete_task_from_multiple_tasks() {
        let (_temp_dir, repo) = setup_test_repo();

        // 複数のタスクを作成
        let tasks = vec![
            Task::new(1, "タスク1", "", Status::Pending, Priority::Low, vec![]),
            Task::new(
                2,
                "タスク2",
                "",
                Status::InProgress,
                Priority::Medium,
                vec![],
            ),
            Task::new(3, "タスク3", "", Status::Completed, Priority::High, vec![]),
        ];
        repo.save(&tasks).unwrap();

        // 中間のタスク（ID=2）を削除
        let result = delete_task(&repo, 2);
        assert!(result.is_ok());

        // タスクが2件残っていることを確認
        let remaining_tasks = repo.load().unwrap();
        assert_eq!(remaining_tasks.len(), 2);

        // ID=1とID=3が残っていることを確認
        let ids: Vec<u64> = remaining_tasks.iter().map(|t| t.id).collect();
        assert!(ids.contains(&1));
        assert!(ids.contains(&3));
        assert!(!ids.contains(&2));
    }

    #[test]
    fn test_delete_task_last_remaining() {
        let (_temp_dir, repo) = setup_test_repo();

        // タスクを1件だけ作成
        let tasks = vec![Task::new(
            1,
            "最後のタスク",
            "",
            Status::Pending,
            Priority::Medium,
            vec![],
        )];
        repo.save(&tasks).unwrap();

        // 唯一のタスクを削除
        let result = delete_task(&repo, 1);
        assert!(result.is_ok());

        // タスクが空になったことを確認
        let remaining_tasks = repo.load().unwrap();
        assert_eq!(remaining_tasks.len(), 0);
    }

    #[test]
    fn test_delete_task_with_tags() {
        let (_temp_dir, repo) = setup_test_repo();

        // タグ付きタスクを作成
        let tasks = vec![
            Task::new(
                1,
                "タスク1",
                "",
                Status::Pending,
                Priority::Medium,
                vec![1, 2],
            ),
            Task::new(2, "タスク2", "", Status::Pending, Priority::Medium, vec![3]),
        ];
        repo.save(&tasks).unwrap();

        // タグ付きタスクを削除
        let result = delete_task(&repo, 1);
        assert!(result.is_ok());

        // タスクが削除されていることを確認
        let remaining_tasks = repo.load().unwrap();
        assert_eq!(remaining_tasks.len(), 1);
        assert_eq!(remaining_tasks[0].id, 2);
    }

    #[test]
    fn test_delete_task_preserves_other_tasks() {
        let (_temp_dir, repo) = setup_test_repo();

        // 様々な状態のタスクを作成
        let tasks = vec![
            Task::new(1, "削除対象", "", Status::Pending, Priority::Low, vec![]),
            Task::new(
                2,
                "保持対象",
                "",
                Status::Completed,
                Priority::High,
                vec![1, 2],
            ),
        ];
        repo.save(&tasks).unwrap();

        // ID=1を削除
        let result = delete_task(&repo, 1);
        assert!(result.is_ok());

        // 残ったタスクの内容が変更されていないことを確認
        let remaining_tasks = repo.load().unwrap();
        assert_eq!(remaining_tasks.len(), 1);
        assert_eq!(remaining_tasks[0].id, 2);
        assert_eq!(remaining_tasks[0].title, "保持対象");
        assert_eq!(remaining_tasks[0].status, Status::Completed);
        assert_eq!(remaining_tasks[0].priority, Priority::High);
        assert_eq!(remaining_tasks[0].tags, vec![1, 2]);
    }
}
