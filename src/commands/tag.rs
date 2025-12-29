use crate::display::format_local_time;
use crate::task::Task;
use crate::{repository::Repository, tag::Tag};
use anyhow::{Context, Result};
use comfy_table::{Table, presets::UTF8_FULL};
use inquire::{Editor, Text, validator};

/// 新しいタグを追加
pub fn add_tag(
    repo: &impl Repository<Tag>,
    name: Option<String>,
    description: Option<String>,
) -> Result<()> {
    let name = match name {
        Some(n) => n,
        None => Text::new("タグの名前を入力してください")
            .with_validator(validator::MinLengthValidator::new(1))
            .prompt()
            .context("タグの名前の入力に失敗しました")?,
    };

    let description = match description {
        Some(d) => d,
        None => Editor::new("タグの説明を入力してください")
            .prompt()
            .context("タグの説明の入力に失敗しました")?,
    };

    let mut tags = repo.load()?;
    let new_id = repo.find_next_id(&tags);
    let new_tag = Tag::new(new_id, &name, &description);

    tags.push(new_tag.clone());
    repo.save(&tags)?;

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "名前", "説明", "作成日", "更新日"]);
    table.add_row(vec![
        new_tag.id.to_string(),
        new_tag.name.to_string(),
        new_tag.description.to_string(),
        format_local_time(&new_tag.created_at),
        format_local_time(&new_tag.updated_at),
    ]);

    println!("{table}");

    Ok(())
}

/// タグ一覧を表示
pub fn list_tags(repo: &impl Repository<Tag>) -> Result<()> {
    let tags = repo.load()?;

    if tags.is_empty() {
        println!("登録されているタグはありません");
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "名前", "説明", "作成日", "更新日"]);

    for tag in tags {
        table.add_row(vec![
            tag.id.to_string(),
            tag.name.to_string(),
            tag.description.to_string(),
            format_local_time(&tag.created_at),
            format_local_time(&tag.updated_at),
        ]);
    }

    println!("{table}");
    Ok(())
}

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
    use crate::task::{Priority, Status, Task};
    use tempfile::TempDir;

    fn setup_test_repo() -> (TempDir, JsonRepository<Tag>) {
        let temp_dir = tempfile::tempdir().unwrap();
        let tag_file = temp_dir.path().join("tags.json");
        let repo = JsonRepository::new(&tag_file);
        repo.ensure_data_exists().unwrap();
        (temp_dir, repo)
    }

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
    fn test_add_tag_with_name_and_description() {
        let (_temp_dir, repo) = setup_test_repo();

        // タグを追加
        add_tag(
            &repo,
            Some("重要".to_string()),
            Some("重要なタスク用".to_string()),
        )
        .unwrap();

        // 追加されたタグを確認
        let tags = repo.load().unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "重要");
        assert_eq!(tags[0].description, "重要なタスク用");
        assert_eq!(tags[0].id, 1);
    }

    #[test]
    fn test_add_multiple_tags() {
        let (_temp_dir, repo) = setup_test_repo();

        // 複数のタグを追加
        add_tag(
            &repo,
            Some("重要".to_string()),
            Some("重要なタスク用".to_string()),
        )
        .unwrap();
        add_tag(
            &repo,
            Some("作業中".to_string()),
            Some("現在作業中".to_string()),
        )
        .unwrap();

        // タグを確認
        let tags = repo.load().unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].id, 1);
        assert_eq!(tags[1].id, 2);
    }

    #[test]
    fn test_list_tags_empty() {
        let (_temp_dir, repo) = setup_test_repo();

        // 空の状態でリストを表示
        let result = list_tags(&repo);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_tags_with_data() {
        let (_temp_dir, repo) = setup_test_repo();

        // タグを追加
        add_tag(
            &repo,
            Some("重要".to_string()),
            Some("重要なタスク用".to_string()),
        )
        .unwrap();

        // リストを表示
        let result = list_tags(&repo);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_tag() {
        let (_temp_dir, tag_repo, task_repo) = setup_test_repos();

        // タグを追加
        add_tag(
            &tag_repo,
            Some("重要".to_string()),
            Some("重要なタスク用".to_string()),
        )
        .unwrap();
        add_tag(
            &tag_repo,
            Some("作業中".to_string()),
            Some("現在作業中".to_string()),
        )
        .unwrap();

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
