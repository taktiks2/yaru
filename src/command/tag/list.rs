use crate::{
    display::create_tag_table,
    repository::{tag::TagRepository, Repository},
};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// タグ一覧を表示
pub async fn list_tags(db: &DatabaseConnection) -> Result<()> {
    let tag_repo = TagRepository::new(db);
    let tags = tag_repo.find_all().await?;

    if tags.is_empty() {
        println!("登録されているタグはありません");
        return Ok(());
    }

    let table = create_tag_table(&tags);
    println!("{table}");
    Ok(())
}
