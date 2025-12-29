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
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // タイトルとステータスを指定してタスクを追加
        let result = add_task(
            &task_repo,
            &tag_repo,
            Some("完了済みタスク".to_string()),
            Some("完了したタスクの説明".to_string()),
            Some(Status::Completed),
            None,
            None,
        );

        assert!(result.is_ok());

        // タスクが保存され、指定したステータスになっていることを確認
        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "完了済みタスク");
        assert_eq!(tasks[0].status, Status::Completed);
        assert_eq!(tasks[0].priority, Priority::Medium); // デフォルト
    }

    #[test]
    fn test_add_task_with_only_title() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // タイトルと説明のみ指定してタスクを追加
        let result = add_task(
            &task_repo,
            &tag_repo,
            Some("シンプルなタスク".to_string()),
            Some("説明".to_string()),
            None,
            None,
            None,
        );

        assert!(result.is_ok());

        // デフォルトステータスがPendingになることを確認
        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "シンプルなタスク");
        assert_eq!(tasks[0].status, Status::Pending);
        assert_eq!(tasks[0].priority, Priority::Medium);
    }

    #[test]
    fn test_add_task_with_custom_priority() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // 優先度を指定してタスクを追加
        let result = add_task(
            &task_repo,
            &tag_repo,
            Some("高優先度タスク".to_string()),
            Some("緊急のタスク".to_string()),
            None,
            Some(Priority::High),
            None,
        );

        assert!(result.is_ok());

        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].priority, Priority::High);
    }

    #[test]
    fn test_add_task_with_all_parameters() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // タグを作成
        let tags = vec![Tag::new(1, "重要", "重要なタスク")];
        tag_repo.save(&tags).unwrap();

        // すべてのパラメータを指定してタスクを追加
        let result = add_task(
            &task_repo,
            &tag_repo,
            Some("完全指定タスク".to_string()),
            Some("詳細な説明".to_string()),
            Some(Status::InProgress),
            Some(Priority::Low),
            Some(vec![1]),
        );

        assert!(result.is_ok());

        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "完全指定タスク");
        assert_eq!(tasks[0].description, "詳細な説明");
        assert_eq!(tasks[0].status, Status::InProgress);
        assert_eq!(tasks[0].priority, Priority::Low);
        assert_eq!(tasks[0].tags, vec![1]);
    }

    #[test]
    fn test_add_multiple_tasks() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // 複数のタスクを追加
        add_task(
            &task_repo,
            &tag_repo,
            Some("タスク1".to_string()),
            Some("説明1".to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        add_task(
            &task_repo,
            &tag_repo,
            Some("タスク2".to_string()),
            Some("説明2".to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        add_task(
            &task_repo,
            &tag_repo,
            Some("タスク3".to_string()),
            Some("説明3".to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        // 3件のタスクが保存されていることを確認
        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[1].id, 2);
        assert_eq!(tasks[2].id, 3);
    }

    #[test]
    fn test_add_task_id_auto_increment() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // タスクを1つ追加
        add_task(
            &task_repo,
            &tag_repo,
            Some("最初のタスク".to_string()),
            Some("説明".to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        // 次のタスクを追加
        add_task(
            &task_repo,
            &tag_repo,
            Some("2番目のタスク".to_string()),
            Some("説明".to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        // IDが自動的に増加することを確認
        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[1].id, 2);
    }

    #[test]
    fn test_add_task_with_multiple_tags() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // 複数のタグを作成
        let tags = vec![
            Tag::new(1, "重要", "重要なタスク"),
            Tag::new(2, "緊急", "緊急タスク"),
            Tag::new(3, "作業中", "作業中"),
        ];
        tag_repo.save(&tags).unwrap();

        // 複数のタグを指定してタスクを追加
        let result = add_task(
            &task_repo,
            &tag_repo,
            Some("複数タグタスク".to_string()),
            Some("説明".to_string()),
            None,
            None,
            Some(vec![1, 2, 3]),
        );

        assert!(result.is_ok());

        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].tags, vec![1, 2, 3]);
    }

    #[test]
    fn test_add_task_validates_all_tag_ids() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // タグを2つ作成
        let tags = vec![Tag::new(1, "タグ1", "説明1"), Tag::new(2, "タグ2", "説明2")];
        tag_repo.save(&tags).unwrap();

        // 存在するタグと存在しないタグを混在させる
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
    fn test_add_task_different_statuses() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // 異なるステータスでタスクを追加
        add_task(
            &task_repo,
            &tag_repo,
            Some("保留中".to_string()),
            Some("".to_string()),
            Some(Status::Pending),
            None,
            None,
        )
        .unwrap();

        add_task(
            &task_repo,
            &tag_repo,
            Some("進行中".to_string()),
            Some("".to_string()),
            Some(Status::InProgress),
            None,
            None,
        )
        .unwrap();

        add_task(
            &task_repo,
            &tag_repo,
            Some("完了".to_string()),
            Some("".to_string()),
            Some(Status::Completed),
            None,
            None,
        )
        .unwrap();

        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0].status, Status::Pending);
        assert_eq!(tasks[1].status, Status::InProgress);
        assert_eq!(tasks[2].status, Status::Completed);
    }

    #[test]
    fn test_add_task_different_priorities() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // 異なる優先度でタスクを追加
        add_task(
            &task_repo,
            &tag_repo,
            Some("低優先度".to_string()),
            Some("".to_string()),
            None,
            Some(Priority::Low),
            None,
        )
        .unwrap();

        add_task(
            &task_repo,
            &tag_repo,
            Some("中優先度".to_string()),
            Some("".to_string()),
            None,
            Some(Priority::Medium),
            None,
        )
        .unwrap();

        add_task(
            &task_repo,
            &tag_repo,
            Some("高優先度".to_string()),
            Some("".to_string()),
            None,
            Some(Priority::High),
            None,
        )
        .unwrap();

        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0].priority, Priority::Low);
        assert_eq!(tasks[1].priority, Priority::Medium);
        assert_eq!(tasks[2].priority, Priority::High);
    }

    #[test]
    fn test_add_task_preserves_task_data() {
        let (_temp_dir, task_repo, tag_repo) = setup_test_repos();

        // タグを作成
        let tags = vec![Tag::new(1, "テストタグ", "説明")];
        tag_repo.save(&tags).unwrap();

        // タスクを追加
        add_task(
            &task_repo,
            &tag_repo,
            Some("データ保持テスト".to_string()),
            Some("詳細な説明文".to_string()),
            Some(Status::InProgress),
            Some(Priority::High),
            Some(vec![1]),
        )
        .unwrap();

        // 保存されたタスクのデータが正確であることを確認
        let tasks = task_repo.load().unwrap();
        assert_eq!(tasks.len(), 1);

        let task = &tasks[0];
        assert_eq!(task.title, "データ保持テスト");
        assert_eq!(task.description, "詳細な説明文");
        assert_eq!(task.status, Status::InProgress);
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.tags, vec![1]);
        assert!(task.created_at.len() > 0);
        assert!(task.updated_at.len() > 0);
    }
}
