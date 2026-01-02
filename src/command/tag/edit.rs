use crate::{
    display::create_tag_detail_table,
    domain::tag::Tag,
    repository::{Repository, tag::TagRepository},
};
use anyhow::{Context, Result};
use inquire::{Editor, Text, validator};
use sea_orm::DatabaseConnection;

/// タグ編集のパラメータ
pub struct EditTagParams {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// タグを編集
pub async fn edit_tag(
    db: &DatabaseConnection,
    params: EditTagParams,
) -> Result<()> {
    // 1. 既存タグを取得
    let tag_repo = TagRepository::new(db);
    let existing_tag = tag_repo
        .find_by_id(params.id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("ID {} のタグが見つかりません", params.id))?;

    // 2. 引数モードか対話モードか判定
    let is_interactive = params.name.is_none() && params.description.is_none();

    let (new_name, new_description) = if is_interactive {
        // 対話モード
        let n = Text::new("タグの名前:")
            .with_initial_value(&existing_tag.name)
            .with_validator(validator::MinLengthValidator::new(1))
            .prompt()
            .context("タグの名前の入力に失敗しました")?;

        let d = Editor::new("タグの説明:")
            .with_predefined_text(&existing_tag.description)
            .prompt()
            .unwrap_or_else(|_| existing_tag.description.clone());

        (n, d)
    } else {
        // 引数モード
        (
            params.name.unwrap_or(existing_tag.name),
            params.description.unwrap_or(existing_tag.description),
        )
    };

    // 3. 更新されたタグを作成
    let updated_tag = Tag {
        id: existing_tag.id,
        name: new_name,
        description: new_description,
        created_at: existing_tag.created_at,
        updated_at: existing_tag.updated_at,
    };

    // 4. リポジトリで更新
    let result = tag_repo.update(&updated_tag).await?;

    println!("タグを更新しました。");
    let table = create_tag_detail_table(&result);
    println!("{table}");

    Ok(())
}
