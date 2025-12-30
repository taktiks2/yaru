use crate::{display::create_task_detail_table, repository::Repository, tag::Tag, task::Task};
use anyhow::Result;

/// 指定されたIDのタスク詳細を表示
pub fn show_task(
    task_repo: &impl Repository<Task>,
    tag_repo: &impl Repository<Tag>,
    id: u64,
) -> Result<()> {
    let tasks = task_repo.load()?;
    let tags = tag_repo.load()?;

    let task = tasks
        .iter()
        .find(|task| task.id == id)
        .ok_or_else(|| anyhow::anyhow!("ID {} のタスクが見つかりません", id))?;

    let table = create_task_detail_table(task, &tags);
    println!("{table}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::JsonRepository;
    use crate::tag::Tag;
    use crate::task::{Priority, Status, Task};
    use tempfile::tempdir;

    #[test]
    fn test_show_task_found() {
        let dir = tempdir().unwrap();
        let task_repo = JsonRepository::<Task>::new(dir.path().join("tasks.json"));
        let tag_repo = JsonRepository::<Tag>::new(dir.path().join("tags.json"));

        // タスクとタグを準備
        let tasks = vec![
            Task::new(
                1,
                "タスク1",
                "説明1",
                Status::Pending,
                Priority::Medium,
                vec![],
            ),
            Task::new(
                2,
                "タスク2",
                "説明2",
                Status::InProgress,
                Priority::High,
                vec![1],
            ),
        ];
        let tags = vec![Tag::new(1, "重要", "重要なタスク")];

        task_repo.save(&tasks).unwrap();
        tag_repo.save(&tags).unwrap();

        // ID 2 のタスクを表示（エラーが発生しないことを確認）
        let result = show_task(&task_repo, &tag_repo, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_task_not_found() {
        let dir = tempdir().unwrap();
        let task_repo = JsonRepository::<Task>::new(dir.path().join("tasks.json"));
        let tag_repo = JsonRepository::<Tag>::new(dir.path().join("tags.json"));

        // タスクを準備
        let tasks = vec![Task::new(
            1,
            "タスク1",
            "説明1",
            Status::Pending,
            Priority::Medium,
            vec![],
        )];
        task_repo.save(&tasks).unwrap();
        tag_repo.ensure_data_exists().unwrap();

        // 存在しないID 999 を検索
        let result = show_task(&task_repo, &tag_repo, 999);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("見つかりません"));
    }

    #[test]
    fn test_show_task_with_tags() {
        let dir = tempdir().unwrap();
        let task_repo = JsonRepository::<Task>::new(dir.path().join("tasks.json"));
        let tag_repo = JsonRepository::<Tag>::new(dir.path().join("tags.json"));

        // タグ付きタスクを準備
        let tasks = vec![Task::new(
            1,
            "タスク1",
            "説明1",
            Status::Pending,
            Priority::Medium,
            vec![1, 2],
        )];
        let tags = vec![
            Tag::new(1, "重要", "重要なタスク"),
            Tag::new(2, "作業中", "現在作業中"),
        ];

        task_repo.save(&tasks).unwrap();
        tag_repo.save(&tags).unwrap();

        // タグ付きタスクを表示（エラーが発生しないことを確認）
        let result = show_task(&task_repo, &tag_repo, 1);
        assert!(result.is_ok());
    }
}
