use crate::task::Task;
use crate::{repository::Repository, tag::Tag};
use anyhow::Result;

/// 指定されたIDのタグを削除
pub fn delete_tag(
    repo: &impl Repository<Tag>,
    task_repo: &impl Repository<Task>,
    id: u64,
) -> Result<()> {
    let mut tags = repo.load()?;
    let original_len = tags.len();

    tags.retain(|tag| tag.id != id);

    if tags.len() == original_len {
        anyhow::bail!("ID {} のタグが見つかりません", id);
    }

    // 参照整合性チェック：このタグを参照しているタスクがないか確認
    let tasks = task_repo.load()?;
    let referenced_tasks: Vec<&Task> = tasks
        .iter()
        .filter(|task| task.tags.contains(&id))
        .collect();

    if !referenced_tasks.is_empty() {
        anyhow::bail!(
            "ID {} のタグは削除できません。このタグを参照しているタスクがあります (タスクID: {})",
            id,
            referenced_tasks
                .iter()
                .map(|t| t.id.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    repo.save(&tags)?;
    println!("ID {} のタグを削除しました", id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::JsonRepository;
    use crate::task::{Priority, Status};
    use tempfile::TempDir;

    fn setup_test_repos() -> (TempDir, JsonRepository<Tag>, JsonRepository<Task>) {
        let temp_dir = tempfile::tempdir().unwrap();
        let tag_file = temp_dir.path().join("tags.json");
        let task_file = temp_dir.path().join("tasks.json");

        let tag_repo = JsonRepository::new(&tag_file);
        let task_repo = JsonRepository::new(&task_file);

        tag_repo.ensure_data_exists().unwrap();
        task_repo.ensure_data_exists().unwrap();

        (temp_dir, tag_repo, task_repo)
    }

    #[test]
    fn test_delete_tag() {
        let (_temp_dir, tag_repo, task_repo) = setup_test_repos();

        // タグを追加
        let tags = vec![
            Tag::new(1, "重要", "重要なタスク用"),
            Tag::new(2, "作業中", "現在作業中"),
        ];
        tag_repo.save(&tags).unwrap();

        // タグを削除
        delete_tag(&tag_repo, &task_repo, 1).unwrap();

        // 削除されたことを確認
        let tags = tag_repo.load().unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].id, 2);
    }

    #[test]
    fn test_delete_nonexistent_tag() {
        let (_temp_dir, tag_repo, task_repo) = setup_test_repos();

        // 存在しないタグを削除しようとする
        let result = delete_tag(&tag_repo, &task_repo, 999);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_tag_referenced_by_task() {
        let (_temp_dir, tag_repo, task_repo) = setup_test_repos();

        // タグを作成
        let tags = vec![
            Tag::new(1, "重要", "重要なタスク"),
            Tag::new(2, "作業中", "現在作業中"),
        ];
        tag_repo.save(&tags).unwrap();

        // タグID=1を参照するタスクを作成
        let tasks = vec![Task::new(
            1,
            "テストタスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![1],
        )];
        task_repo.save(&tasks).unwrap();

        // タグID=1を削除しようとする
        let result = delete_tag(&tag_repo, &task_repo, 1);

        // エラーが返されることを確認
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("このタグを参照しているタスクがあります")
        );

        // タグが削除されていないことを確認
        let tags = tag_repo.load().unwrap();
        assert_eq!(tags.len(), 2);
    }

    #[test]
    fn test_delete_tag_not_referenced() {
        let (_temp_dir, tag_repo, task_repo) = setup_test_repos();

        // タグを作成
        let tags = vec![
            Tag::new(1, "重要", "重要なタスク"),
            Tag::new(2, "作業中", "現在作業中"),
        ];
        tag_repo.save(&tags).unwrap();

        // タグID=1を参照するタスクを作成
        let tasks = vec![Task::new(
            1,
            "テストタスク",
            "説明",
            Status::Pending,
            Priority::Medium,
            vec![1],
        )];
        task_repo.save(&tasks).unwrap();

        // タグID=2（参照されていない）を削除
        let result = delete_tag(&tag_repo, &task_repo, 2);

        // 成功することを確認
        assert!(result.is_ok());

        // タグID=2が削除されていることを確認
        let tags = tag_repo.load().unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].id, 1);
    }
}
