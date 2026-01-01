use crate::{
    display::create_tag_detail_table,
    repository::{tag::TagRepository, Repository},
};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// 指定されたIDのタグ詳細を表示
pub async fn show_tag(db: &DatabaseConnection, id: i32) -> Result<()> {
    let tag_repo = TagRepository::new(db);
    let Some(tag) = tag_repo.find_by_id(id).await? else {
        anyhow::bail!("ID {id} のタグが見つかりません");
    };

    let table = create_tag_detail_table(&tag);
    println!("{table}");

    Ok(())
}
