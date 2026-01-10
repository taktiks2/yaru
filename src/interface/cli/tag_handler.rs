use crate::{
    application::{
        dto::tag_dto::{CreateTagDTO, UpdateTagDTO},
        use_cases::tag::{
            add_tag::AddTagUseCase, delete_tag::DeleteTagUseCase, edit_tag::EditTagUseCase,
            list_tags::ListTagsUseCase, show_tag::ShowTagUseCase,
        },
    },
    domain::tag::repository::TagRepository,
    interface::{cli::args::TagCommands, presentation::Presenter},
};
use anyhow::{Context, Result};
use inquire::{Editor, MultiSelect, Text, validator};
use std::sync::Arc;

/// タグ追加のパラメータ
struct AddTagParams {
    name: Option<String>,
    description: Option<String>,
}

/// タグ編集のパラメータ
struct EditTagParams {
    name: Option<String>,
    description: Option<String>,
}

/// タグコマンドを処理
pub async fn handle_tag_command(
    command: TagCommands,
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
) -> Result<()> {
    match command {
        TagCommands::List => handle_list(tag_repo, presenter).await,
        TagCommands::Show { id } => handle_show(tag_repo, presenter, id).await,
        TagCommands::Add { name, description } => {
            let params = AddTagParams { name, description };
            handle_add(tag_repo, presenter, params).await
        }
        TagCommands::Delete { id } => handle_delete(tag_repo, presenter, id).await,
        TagCommands::Edit {
            id,
            name,
            description,
        } => {
            let params = EditTagParams { name, description };
            handle_edit(tag_repo, presenter, id, params).await
        }
    }
}

/// タグ一覧を表示
async fn handle_list(
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
) -> Result<()> {
    let use_case = ListTagsUseCase::new(tag_repo);
    let tags = use_case.execute().await?;

    presenter.present_tag_list(&tags)?;

    Ok(())
}

/// タグの詳細を表示
async fn handle_show(
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
    id: i32,
) -> Result<()> {
    let use_case = ShowTagUseCase::new(tag_repo);
    let tag = use_case.execute(id).await?;

    presenter.present_tag_detail(&tag)?;

    Ok(())
}

/// 新しいタグを追加
async fn handle_add(
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
    params: AddTagParams,
) -> Result<()> {
    // 引数モードか対話モードか判定
    let is_interactive = params.name.is_none();

    let (final_name, final_description) = if is_interactive {
        // 対話モード
        let n = Text::new("タグの名前を入力してください")
            .with_validator(validator::MinLengthValidator::new(1))
            .prompt()
            .context("タグの名前の入力に失敗しました")?;

        let d = params.description.unwrap_or_else(|| {
            Editor::new("タグの説明を入力してください")
                .prompt()
                .unwrap_or_default()
        });

        (n, d)
    } else {
        // 引数モード
        // SAFETY: is_interactive=falseの場合、params.nameはSomeであることが保証されている
        (params.name.unwrap(), params.description.unwrap_or_default())
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

    presenter.present_success(&format!(
        "タグを追加しました: [{}] {}",
        created_tag.id, created_tag.name
    ))?;

    Ok(())
}

/// タグを削除
async fn handle_delete(
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
    id: i32,
) -> Result<()> {
    // 確認
    let confirm = presenter.confirm(&format!("タグID {}を削除しますか？", id), false)?;

    if !confirm {
        presenter.present_success("削除をキャンセルしました")?;
        return Ok(());
    }

    let use_case = DeleteTagUseCase::new(tag_repo);
    use_case.execute(id).await?;

    presenter.present_success(&format!("タグID {}を削除しました", id))?;

    Ok(())
}

async fn handle_edit(
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
    id: i32,
    params: EditTagParams,
) -> Result<()> {
    // 引数モードか対話モードか判定
    let is_interactive = params.name.is_none() && params.description.is_none();

    let (final_name, final_description) = if is_interactive {
        // 対話モード: 既存のタグ情報を取得
        let use_case = ShowTagUseCase::new(tag_repo.clone());
        let current_tag = use_case.execute(id).await?;

        presenter.present_tag_detail(&current_tag)?;
        println!(); // 空行を追加

        // 編集するフィールドを選択
        let field_options = vec!["名前", "説明"];

        let selected_fields = MultiSelect::new(
            "編集するフィールドを選択してください（スペースで選択、Enterで確定）",
            field_options,
        )
        .with_vim_mode(true)
        .prompt()
        .unwrap_or_default();

        // 選択されたフィールドのみ編集
        let new_name = if selected_fields.contains(&"名前") {
            Some(
                Text::new("名前:")
                    .with_default(&current_tag.name)
                    .with_validator(validator::MinLengthValidator::new(1))
                    .prompt()
                    .context("名前の入力に失敗しました")?,
            )
        } else {
            None
        };

        let new_description = if selected_fields.contains(&"説明") {
            Some(
                Editor::new("説明を入力してください")
                    .with_predefined_text(current_tag.description.as_deref().unwrap_or(""))
                    .prompt()
                    .unwrap_or_default(),
            )
        } else {
            None
        };

        (new_name, new_description)
    } else {
        // 引数モード
        (params.name, params.description)
    };

    // DTOを構築
    let dto = UpdateTagDTO {
        name: final_name,
        description: final_description,
    };

    // Use Caseを実行
    let use_case = EditTagUseCase::new(tag_repo);
    let updated_tag = use_case.execute(id, dto).await?;

    presenter.present_success(&format!(
        "タグを更新しました: [{}] {}",
        updated_tag.id, updated_tag.name
    ))?;

    Ok(())
}
