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
    // 引数モードか対話モードか判定
    let is_interactive = name.is_none() && description.is_none();

    let (name, description) = if is_interactive {
        // 対話モード
        let n = Text::new("タグの名前を入力してください")
            .with_validator(validator::MinLengthValidator::new(1))
            .prompt()
            .context("タグの名前の入力に失敗しました")?;
        let d = Editor::new("タグの説明を入力してください")
            .prompt()
            .unwrap_or_default();
        (n, d)
    } else {
        // 引数モード
        (name.unwrap_or_default(), description.unwrap_or_default())
    };

    // リポジトリを使用してタグを作成
    let new_tag = Tag::new(0, &name, &description);
    let tag_repo = TagRepository::new(db);
    let created_tag = tag_repo.create(&new_tag).await?;

    let table = create_tag_detail_table(&created_tag);
    println!("{table}");

    Ok(())
}
