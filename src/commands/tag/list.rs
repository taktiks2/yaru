use crate::display::create_tag_table;
use crate::{repository::Repository, tag::Tag};
use anyhow::Result;

/// タグ一覧を表示
pub fn list_tags(repo: &impl Repository<Tag>) -> Result<()> {
    let tags = repo.load()?;

    if tags.is_empty() {
        println!("登録されているタグはありません");
        return Ok(());
    }

    let table = create_tag_table(&tags);
    println!("{table}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::JsonRepository;
    use crate::tag::Tag;
    use tempfile::TempDir;

    fn setup_test_repo() -> (TempDir, JsonRepository<Tag>) {
        let temp_dir = tempfile::tempdir().unwrap();
        let tag_file = temp_dir.path().join("tags.json");
        let repo = JsonRepository::new(&tag_file);
        repo.ensure_data_exists().unwrap();
        (temp_dir, repo)
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
        let tags = vec![Tag::new(1, "重要", "重要なタスク用")];
        repo.save(&tags).unwrap();

        // リストを表示
        let result = list_tags(&repo);
        assert!(result.is_ok());
    }
}
