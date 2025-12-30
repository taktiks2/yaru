use crate::{display::create_tag_detail_table, repository::Repository, tag::Tag};
use anyhow::Result;

/// 指定されたIDのタグ詳細を表示
pub fn show_tag(repo: &impl Repository<Tag>, id: u64) -> Result<()> {
    let tags = repo.load()?;

    let tag = tags
        .iter()
        .find(|tag| tag.id == id)
        .ok_or_else(|| anyhow::anyhow!("ID {} のタグが見つかりません", id))?;

    let table = create_tag_detail_table(tag);
    println!("{table}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::JsonRepository;
    use crate::tag::Tag;
    use tempfile::tempdir;

    #[test]
    fn test_show_tag_found() {
        let dir = tempdir().unwrap();
        let tag_repo = JsonRepository::<Tag>::new(dir.path().join("tags.json"));

        // タグを準備
        let tags = vec![
            Tag::new(1, "重要", "重要なタスク"),
            Tag::new(2, "作業中", "現在作業中"),
        ];
        tag_repo.save(&tags).unwrap();

        // ID 2 のタグを表示（エラーが発生しないことを確認）
        let result = show_tag(&tag_repo, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_tag_not_found() {
        let dir = tempdir().unwrap();
        let tag_repo = JsonRepository::<Tag>::new(dir.path().join("tags.json"));

        // タグを準備
        let tags = vec![Tag::new(1, "重要", "重要なタスク")];
        tag_repo.save(&tags).unwrap();

        // 存在しないID 999 を検索
        let result = show_tag(&tag_repo, 999);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("見つかりません"));
    }

    #[test]
    fn test_show_tag_empty_repository() {
        let dir = tempdir().unwrap();
        let tag_repo = JsonRepository::<Tag>::new(dir.path().join("tags.json"));

        // 空のリポジトリを準備
        tag_repo.ensure_data_exists().unwrap();

        // 存在しないID を検索
        let result = show_tag(&tag_repo, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("見つかりません"));
    }
}
