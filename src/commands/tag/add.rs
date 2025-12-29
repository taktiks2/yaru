use crate::display::create_single_tag_table;
use crate::{repository::Repository, tag::Tag};
use anyhow::{Context, Result};
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

    let table = create_single_tag_table(&new_tag);
    println!("{table}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::JsonRepository;
    use tempfile::TempDir;

    fn setup_test_repo() -> (TempDir, JsonRepository<Tag>) {
        let temp_dir = tempfile::tempdir().unwrap();
        let tag_file = temp_dir.path().join("tags.json");
        let repo = JsonRepository::new(&tag_file);
        repo.ensure_data_exists().unwrap();
        (temp_dir, repo)
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
}
