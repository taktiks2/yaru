use crate::{
    display::create_tag_detail_table,
    repository::{Repository, tag::TagRepository},
};
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// タグ詳細表示のパラメータ
pub struct ShowTagParams {
    pub id: i32,
}

/// 指定されたIDのタグ詳細を表示
pub async fn show_tag(db: &DatabaseConnection, params: ShowTagParams) -> Result<()> {
    let tag_repo = TagRepository::new(db);
    let Some(tag) = tag_repo.find_by_id(params.id).await? else {
        anyhow::bail!("ID {} のタグが見つかりません", params.id);
    };

    let table = create_tag_detail_table(&tag);
    println!("{table}");

    Ok(())
}
