use crate::{
    display::create_tag_detail_table,
    domain::tag::Tag,
    repository::{Repository, tag::TagRepository},
};
use anyhow::{Context, Result};
use inquire::{Editor, Text, validator};
use sea_orm::DatabaseConnection;

/// 新しいタグを追加
pub async fn add_tag(
    db: &DatabaseConnection,
    name: Option<String>,
    description: Option<String>,
) -> Result<()> {
    let (name, description) = match name {
        Some(name) => {
            anyhow::ensure!(!name.is_empty(), "名前は空にできません");
            (name, description.unwrap_or_default())
        }
        None => {
            let name = Text::new("タグの名前を入力してください")
                .with_validator(validator::MinLengthValidator::new(1))
                .prompt()
                .context("タグの名前の入力に失敗しました")?;
            let description = Editor::new("タグの説明を入力してください")
                .prompt()
                .unwrap_or_default();
            (name, description)
        }
    };

    // リポジトリを使用してタグを作成
    let new_tag = Tag::new(0, &name, &description);
    let tag_repo = TagRepository::new(db);
    let created_tag = tag_repo.create(&new_tag).await?;

    let table = create_tag_detail_table(&created_tag);
    println!("{table}");

    Ok(())
}
