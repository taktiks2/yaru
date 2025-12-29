use crate::{
    json::{load_json, save_json},
    tag::Tag,
};
use anyhow::Result;
use std::path::PathBuf;

/// タグリポジトリのトレイト
/// データの永続化方法を抽象化し、異なる実装（JSON、SQLiteなど）を切り替え可能にする
pub trait TagRepository {
    /// タグリストを読み込む
    fn load_tags(&self) -> Result<Vec<Tag>>;

    /// タグリストを保存する
    fn save_tags(&self, tags: &[Tag]) -> Result<()>;

    /// 次のIDを取得する
    fn find_next_id(&self, tags: &[Tag]) -> u64 {
        tags.iter().map(|tag| tag.id).max().unwrap_or(0) + 1
    }

    /// データファイルが存在することを確認（必要に応じて初期化）
    fn ensure_data_exists(&self) -> Result<()>;
}

/// JSON形式でタグを保存するリポジトリ実装
pub struct JsonTagRepository {
    file_path: PathBuf,
}

impl JsonTagRepository {
    /// 新しいJsonTagRepositoryインスタンスを作成
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
}

impl TagRepository for JsonTagRepository {
    fn load_tags(&self) -> Result<Vec<Tag>> {
        load_json(&self.file_path)
    }

    fn save_tags(&self, tags: &[Tag]) -> Result<()> {
        save_json(&self.file_path, tags)
    }

    fn ensure_data_exists(&self) -> Result<()> {
        if !self.file_path.exists() {
            save_json(&self.file_path, &Vec::<Tag>::new())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_ensure_data_exists_creates_file() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("tags.json");

        // ファイルが存在しないことを確認
        assert!(!test_file.exists());

        // リポジトリを作成してファイルを初期化
        let repo = JsonTagRepository::new(&test_file);
        repo.ensure_data_exists().unwrap();

        // ファイルが作成されたことを確認
        assert!(test_file.exists());

        // 空のJSONリストが保存されていることを確認
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "[]");
    }

    #[test]
    fn test_load_tags_empty_list() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("tags.json");

        // 空のJSONファイルを作成
        fs::write(&test_file, "[]").unwrap();

        // リポジトリを使って読み込み
        let repo = JsonTagRepository::new(&test_file);
        let tags = repo.load_tags().unwrap();

        // 空のリストが返されることを確認
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn test_save_and_load_tags() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("tags.json");

        let tags = vec![
            Tag::new(1, "重要", "重要なタスク用のタグ"),
            Tag::new(2, "作業中", "現在作業中のタスク用のタグ"),
        ];

        // リポジトリを使って保存
        let repo = JsonTagRepository::new(&test_file);
        repo.save_tags(&tags).unwrap();

        // ファイルが作成されたことを確認
        assert!(test_file.exists());

        // 保存したTagを読み込み
        let loaded_tags = repo.load_tags().unwrap();

        // 正しく保存・読み込みできたことを確認
        assert_eq!(loaded_tags.len(), 2);
        assert_eq!(loaded_tags[0].id, 1);
        assert_eq!(loaded_tags[0].name, "重要");
        assert_eq!(loaded_tags[0].description, "重要なタスク用のタグ");
        assert_eq!(loaded_tags[1].id, 2);
        assert_eq!(loaded_tags[1].name, "作業中");
    }

    #[test]
    fn test_find_next_id_empty_list() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("tags.json");
        let repo = JsonTagRepository::new(&test_file);

        let tags: Vec<Tag> = vec![];
        let next_id = repo.find_next_id(&tags);
        assert_eq!(next_id, 1);
    }

    #[test]
    fn test_find_next_id_with_existing_tags() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("tags.json");
        let repo = JsonTagRepository::new(&test_file);

        let tags = vec![
            Tag::new(1, "タグ1", "説明1"),
            Tag::new(3, "タグ3", "説明3"),
            Tag::new(2, "タグ2", "説明2"),
        ];
        let next_id = repo.find_next_id(&tags);
        assert_eq!(next_id, 4);
    }

    #[test]
    fn test_find_next_id_with_single_tag() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("tags.json");
        let repo = JsonTagRepository::new(&test_file);

        let tags = vec![Tag::new(5, "タグ", "説明")];
        let next_id = repo.find_next_id(&tags);
        assert_eq!(next_id, 6);
    }

    #[test]
    fn test_repository_trait_with_custom_path() {
        let test_dir = setup_test_dir();
        let test_file = test_dir.path().join("custom_tags.json");

        // カスタムパスでリポジトリを作成
        let repo = JsonTagRepository::new(&test_file);

        // データが存在することを確認
        repo.ensure_data_exists().unwrap();
        assert!(test_file.exists());

        // Tagを保存
        let tags = vec![Tag::new(1, "カスタムタグ", "カスタムパステスト用タグ")];
        repo.save_tags(&tags).unwrap();

        // Tagを読み込み
        let loaded_tags = repo.load_tags().unwrap();
        assert_eq!(loaded_tags.len(), 1);
        assert_eq!(loaded_tags[0].name, "カスタムタグ");
    }
}
