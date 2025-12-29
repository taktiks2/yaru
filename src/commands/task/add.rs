use crate::{
    display::create_single_task_table,
    repository::Repository,
    tag::Tag,
    task::{Priority, Status, Task},
};
use anyhow::{Context, Result};
use inquire::{Editor, Text, validator};

/// 新しいタスクを追加
pub fn add_task(
    repo: &impl Repository<Task>,
    tag_repo: &impl Repository<Tag>,
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
    tags: Option<Vec<u64>>,
) -> Result<()> {
    let title = match title {
        Some(t) => t,
        None => Text::new("タスクのタイトルを入力してください")
            .with_validator(validator::MinLengthValidator::new(1))
            .prompt()
            .context("タスクのタイトルの入力に失敗しました")?,
    };

    let description = match description {
        Some(d) => d,
        None => Editor::new("タスクの説明を入力してください")
            .prompt()
            .context("タスクの説明の入力に失敗しました")?,
    };

    let status = status.unwrap_or(Status::Pending);
    let priority = priority.unwrap_or(Priority::Medium);
    let tags = tags.unwrap_or_default();

    // タグIDの存在確認
    if !tags.is_empty() {
        let existing_tags = tag_repo.load()?;
        let existing_tag_ids: Vec<u64> = existing_tags.iter().map(|t| t.id).collect();

        for tag_id in &tags {
            if !existing_tag_ids.contains(tag_id) {
                anyhow::bail!("存在しないタグID: {}", tag_id);
            }
        }
    }

    let mut tasks = repo.load()?;
    let new_id = repo.find_next_id(&tasks);
    let new_task = Task::new(new_id, &title, &description, status, priority, tags);

    tasks.push(new_task.clone());
    repo.save(&tasks)?;

    println!("タスクを登録しました。");

    let table = create_single_task_table(&new_task);
    println!("{table}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::JsonRepository;
    use crate::tag::Tag;
    use tempfile::TempDir;

    fn setup_test_repos() -> (TempDir, JsonRepository<Task>, JsonRepository<Tag>) {
        let temp_dir = tempfile::tempdir().unwrap();
        let task_file = temp_dir.path().join("tasks.json");
        let tag_file = temp_dir.path().join("tags.json");

        let task_repo = JsonRepository::new(&task_file);
        let tag_repo = JsonRepository::new(&tag_file);

        task_repo.ensure_data_exists().unwrap();
        tag_repo.ensure_data_exists().unwrap();

        (temp_dir, task_repo, tag_repo)
    }

    #[test]
    fn test_add_task_with_valid_tag_ids() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // タグを事前に作成
        let tags = vec![
            Tag::new(1, "重要", "重要なタスク"),
            Tag::new(2, "作業中", "現在作業中"),
        ];
        tag_repo.save(&tags).unwrap();

        // 存在するタグIDを指定してタスクを追加
        let result = add_task(
            &task_repo,
            &tag_repo,
            Some("テストタスク".to_string()),
            Some("説明".to_string()),
            None,
            None,
            Some(vec![1, 2]),
        );

        assert!(result.is_ok());

        // タスクが保存されたことを確認
        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].tags, vec![1, 2]);
    }

    #[test]
    fn test_add_task_with_invalid_tag_id() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // タグを1つだけ作成
        let tags = vec![Tag::new(1, "重要", "重要なタスク")];
        tag_repo.save(&tags).unwrap();

        // 存在しないタグID (999) を指定してタスクを追加
        let result = add_task(
            &task_repo,
            &tag_repo,
            Some("テストタスク".to_string()),
            Some("説明".to_string()),
            None,
            None,
            Some(vec![1, 999]),
        );

        // エラーが返されることを確認
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("存在しないタグID"));

        // タスクが保存されていないことを確認
        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_add_task_without_tags() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // タグなしでタスクを追加
        let result = add_task(
            &task_repo,
            &tag_repo,
            Some("テストタスク".to_string()),
            Some("説明".to_string()),
            None,
            None,
            None,
        );

        assert!(result.is_ok());

        // タスクが保存されたことを確認
        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 1);
        assert!(tasks[0].tags.is_empty());
    }

    #[test]
    fn test_add_task_with_title_and_status() {
        // タイトルとステータスを指定してタスクを追加する場合のテスト
        // Note: このテストは実装後に適切な形に修正する必要がある
    }

    #[test]
    fn test_add_task_with_only_title() {
        // タイトルのみを指定してタスクを追加する場合のテスト
        // デフォルトステータスがPendingになることを確認
    }

    #[test]
    fn test_add_task_without_title() {
        // タイトルなしで追加する場合のテスト
        // 対話的入力が必要になるため、統合テストで実装する
    }
}
