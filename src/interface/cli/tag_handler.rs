use anyhow::{Context, Result};
use inquire::{Editor, Text, validator};
use std::sync::Arc;

use crate::application::dto::tag_dto::{CreateTagDTO, UpdateTagDTO};
use crate::application::use_cases::tag::{
    add_tag::AddTagUseCase, delete_tag::DeleteTagUseCase, edit_tag::EditTagUseCase,
    list_tags::ListTagsUseCase, show_tag::ShowTagUseCase,
};
use crate::domain::tag::repository::TagRepository;
use crate::interface::cli::args::TagCommands;
use crate::interface::cli::display::{create_tag_detail_table, create_tag_table};

/// タグコマンドを処理
pub async fn handle_tag_command(
    command: TagCommands,
    tag_repo: Arc<dyn TagRepository>,
) -> Result<()> {
    match command {
        TagCommands::List => handle_list(tag_repo).await,
        TagCommands::Show { id } => handle_show(tag_repo, id).await,
        TagCommands::Add { name, description } => handle_add(tag_repo, name, description).await,
        TagCommands::Delete { id } => handle_delete(tag_repo, id).await,
        TagCommands::Edit {
            id,
            name,
            description,
        } => handle_edit(tag_repo, id, name, description).await,
    }
}

/// タグ一覧を表示
async fn handle_list(tag_repo: Arc<dyn TagRepository>) -> Result<()> {
    let use_case = ListTagsUseCase::new(tag_repo);
    let tags = use_case.execute().await?;

    if tags.is_empty() {
        println!("タグがありません");
    } else {
        println!("タグ一覧 ({}件):", tags.len());
        let table = create_tag_table(&tags);
        println!("{}", table);
    }

    Ok(())
}

/// タグの詳細を表示
async fn handle_show(tag_repo: Arc<dyn TagRepository>, id: i32) -> Result<()> {
    let use_case = ShowTagUseCase::new(tag_repo);
    let tag = use_case.execute(id).await?;

    let table = create_tag_detail_table(&tag);
    println!("{}", table);

    Ok(())
}

/// 新しいタグを追加
async fn handle_add(
    tag_repo: Arc<dyn TagRepository>,
    name: Option<String>,
    description: Option<String>,
) -> Result<()> {
    // 引数モードか対話モードか判定
    let is_interactive = name.is_none();

    let (final_name, final_description) = if is_interactive {
        // 対話モード
        let n = Text::new("タグの名前を入力してください")
            .with_validator(validator::MinLengthValidator::new(1))
            .prompt()
            .context("タグの名前の入力に失敗しました")?;

        let d = description.unwrap_or_else(|| {
            Editor::new("タグの説明を入力してください")
                .prompt()
                .unwrap_or_default()
        });

        (n, d)
    } else {
        // 引数モード
        (name.unwrap(), description.unwrap_or_default())
    };

    // DTOを構築
    let dto = CreateTagDTO {
        name: final_name,
        description: if final_description.is_empty() {
            None
        } else {
            Some(final_description)
        },
    };

    // Use Caseを実行
    let use_case = AddTagUseCase::new(tag_repo);
    let created_tag = use_case.execute(dto).await?;

    println!(
        "タグを追加しました: [{}] {}",
        created_tag.id, created_tag.name
    );

    Ok(())
}

/// タグを削除
async fn handle_delete(tag_repo: Arc<dyn TagRepository>, id: i32) -> Result<()> {
    // 確認
    let confirm = inquire::Confirm::new(&format!("タグID {}を削除しますか？", id))
        .with_default(false)
        .prompt()
        .unwrap_or(false);

    if !confirm {
        println!("削除をキャンセルしました");
        return Ok(());
    }

    let use_case = DeleteTagUseCase::new(tag_repo);
    use_case.execute(id).await?;

    println!("タグID {}を削除しました", id);

    Ok(())
}

/// タグを編集
async fn handle_edit(
    tag_repo: Arc<dyn TagRepository>,
    id: i32,
    name: Option<String>,
    description: Option<String>,
) -> Result<()> {
    // DTOを構築
    let dto = UpdateTagDTO { name, description };

    // Use Caseを実行
    let use_case = EditTagUseCase::new(tag_repo);
    let updated_tag = use_case.execute(id, dto).await?;

    println!(
        "タグを更新しました: [{}] {}",
        updated_tag.id, updated_tag.name
    );

    Ok(())
}
