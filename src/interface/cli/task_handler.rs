use crate::{
    application::{
        dto::task_dto::{CreateTaskDTO, UpdateTaskDTO},
        use_cases::task::{
            add_task::AddTaskUseCase, delete_task::DeleteTaskUseCase, edit_task::EditTaskUseCase,
            list_tasks::ListTasksUseCase, search_tasks::SearchTasksUseCase,
            show_stats::ShowStatsUseCase, show_task::ShowTaskUseCase,
        },
    },
    domain::{
        tag::{repository::TagRepository, value_objects::TagId},
        task::{
            repository::TaskRepository,
            value_objects::{Priority, Status},
        },
    },
    interface::{
        cli::args::{Filter, SearchFieldArg, TaskCommands},
        presentation::Presenter,
    },
};
use anyhow::{Context, Result};
use chrono::NaiveDate;
use inquire::{DateSelect, Editor, MultiSelect, Select, Text, validator};
use std::{collections::HashSet, sync::Arc};
use strum::IntoEnumIterator;

/// タスク追加のパラメータ
struct AddTaskParams {
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
    tags: Option<Vec<i32>>,
    due_date: Option<NaiveDate>,
}

/// タスク編集のパラメータ
struct EditTaskParams {
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
    tags: Option<Vec<i32>>,
    due_date: Option<NaiveDate>,
    clear_due_date: bool,
}

/// タスク検索のパラメータ
struct SearchParams {
    keywords: Option<String>,
    field: SearchFieldArg,
}

/// タグ選択用のラッパー型
///
/// `inquire::MultiSelect`で使用するために、タグIDと表示文字列をペアで保持します。
/// 文字列パースに依存せず、型安全にタグIDを取得できます。
#[derive(Debug, Clone)]
struct TagOption {
    id: i32,
    display: String,
}

impl std::fmt::Display for TagOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

/// タグIDの存在を一括検証
///
/// # Arguments
/// * `tag_repo` - TagRepositoryの参照
/// * `tag_ids` - 検証するタグIDのスライス
///
/// # Returns
/// * `Ok(())` - すべてのタグが存在する場合
/// * `Err` - 存在しないタグがある場合（最初に見つかった不正なIDを報告）
///
/// # Note
/// - find_by_idsを使用してN+1問題を回避します
/// - **UX向上のための事前検証**: アプリケーション層でも再度検証されるため、
///   ここでのエラーはユーザーへの早期フィードバックが目的です
/// - 不要な処理をスキップし、CLI固有のエラーメッセージを提供できます
async fn validate_tag_ids(tag_repo: &Arc<dyn TagRepository>, tag_ids: &[i32]) -> Result<()> {
    if tag_ids.is_empty() {
        return Ok(());
    }

    // i32からTagIdへ変換
    let tag_id_vos = tag_ids
        .iter()
        .map(|id| TagId::new(*id))
        .collect::<Result<Vec<_>>>()?;

    // 一括でタグを取得
    let found_tags = tag_repo.find_by_ids(&tag_id_vos).await?;

    // 見つかったタグのIDセットを作成
    let found_ids: HashSet<i32> = found_tags.iter().map(|tag| tag.id().value()).collect();

    // 存在しないIDを検出
    for id in tag_ids {
        if !found_ids.contains(id) {
            anyhow::bail!("Tag ID does not exist: {}", id);
        }
    }

    Ok(())
}

/// タスクコマンドを処理
pub async fn handle_task_command(
    command: TaskCommands,
    task_repo: Arc<dyn TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
) -> Result<()> {
    match command {
        TaskCommands::List { filter } => handle_list(task_repo, tag_repo, presenter, filter).await,
        TaskCommands::Show { id } => handle_show(task_repo, tag_repo, presenter, id).await,
        TaskCommands::Add {
            title,
            description,
            status,
            priority,
            tags,
            due_date,
        } => {
            let params = AddTaskParams {
                title,
                description,
                status,
                priority,
                tags,
                due_date,
            };
            handle_add(task_repo, tag_repo, presenter, params).await
        }
        TaskCommands::Delete { id } => handle_delete(task_repo, presenter, id).await,
        TaskCommands::Edit {
            id,
            title,
            description,
            status,
            priority,
            tags,
            due_date,
            clear_due_date,
        } => {
            let params = EditTaskParams {
                title,
                description,
                status,
                priority,
                tags,
                due_date,
                clear_due_date,
            };
            handle_edit(task_repo, tag_repo, presenter, id, params).await
        }
        TaskCommands::Stats => handle_stats(task_repo, tag_repo, presenter).await,
        TaskCommands::Search { keywords, field } => {
            let params = SearchParams { keywords, field };
            handle_search(task_repo, tag_repo, presenter, params).await
        }
    }
}

/// タスク一覧を表示
async fn handle_list(
    task_repo: Arc<dyn TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
    _filter: Option<Vec<Filter>>,
) -> Result<()> {
    let use_case = ListTasksUseCase::new(task_repo, tag_repo);
    let tasks = use_case.execute().await?;

    // TODO: フィルタ処理を実装
    presenter.present_task_list(&tasks)?;

    Ok(())
}

/// タスクの詳細を表示
async fn handle_show(
    task_repo: Arc<dyn TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
    id: i32,
) -> Result<()> {
    let use_case = ShowTaskUseCase::new(task_repo, tag_repo);
    let task = use_case.execute(id).await?;

    presenter.present_task_detail(&task)?;

    Ok(())
}

/// 新しいタスクを追加
async fn handle_add(
    task_repo: Arc<dyn TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
    params: AddTaskParams,
) -> Result<()> {
    // 引数モードか対話モードか判定
    let is_interactive = params.title.is_none();

    // タグIDの検証（指定されている場合）
    if let Some(ref ids) = params.tags {
        validate_tag_ids(&tag_repo, ids).await?;
    }

    let (final_title, final_description, final_status, final_priority, final_tags, final_due_date) =
        if is_interactive {
            // 対話モード
            let t = Text::new("Enter task title")
                .with_validator(validator::MinLengthValidator::new(1))
                .prompt()
                .context("Failed to input task title")?;

            let d = params.description.unwrap_or_else(|| {
                Editor::new("Enter task description")
                    .prompt()
                    .unwrap_or_default()
            });

            let s = params.status.unwrap_or_else(|| {
                Select::new(
                    "Select status",
                    Status::iter().collect::<Vec<_>>(),
                )
                .with_vim_mode(true)
                .prompt()
                .unwrap_or(Status::Pending)
            });

            let p = params.priority.unwrap_or_else(|| {
                Select::new(
                    "Select priority",
                    Priority::iter().collect::<Vec<_>>(),
                )
                .with_vim_mode(true)
                .prompt()
                .unwrap_or(Priority::Medium)
            });

            // タグ選択（対話モード）
            let tags = if params.tags.is_some() {
                params.tags.unwrap_or_default()
            } else {
                // 利用可能なタグを取得
                let available_tags = tag_repo.find_all().await?;
                if !available_tags.is_empty() {
                    let tag_options: Vec<TagOption> = available_tags
                        .iter()
                        .map(|t| TagOption {
                            id: t.id().value(),
                            display: format!("[{}] {}", t.id().value(), t.name().value()),
                        })
                        .collect();

                    let selected = MultiSelect::new(
                        "Select tags (Space to select, Enter to confirm)",
                        tag_options,
                    )
                    .with_vim_mode(true)
                    .prompt()
                    .unwrap_or_default();

                    // 直接IDを取得（文字列パース不要）
                    selected.iter().map(|opt| opt.id).collect()
                } else {
                    vec![]
                }
            };

            // 期限選択
            let dd = params.due_date.or_else(|| {
                if inquire::Confirm::new("Set due date?")
                    .with_default(false)
                    .prompt()
                    .unwrap_or(false)
                {
                    DateSelect::new("Select due date").prompt().ok()
                } else {
                    None
                }
            });

            (t, d, s, p, tags, dd)
        } else {
            // 引数モード
            (
                // SAFETY: is_interactive=falseの場合、params.titleはSomeであることが保証されている
                params.title.unwrap(),
                params.description.unwrap_or_default(),
                params.status.unwrap_or(Status::Pending),
                params.priority.unwrap_or(Priority::Medium),
                params.tags.unwrap_or_default(),
                params.due_date,
            )
        };

    // DTOを構築
    let dto = CreateTaskDTO {
        title: final_title,
        description: Some(final_description),
        status: Some(final_status.to_string()),
        priority: Some(final_priority.to_string()),
        tags: final_tags,
        due_date: final_due_date,
    };

    // Use Caseを実行
    let use_case = AddTaskUseCase::new(task_repo, tag_repo);
    let created_task = use_case.execute(dto).await?;

    presenter.present_success(&format!(
        "Task added: [{}] {}",
        created_task.id, created_task.title
    ))?;

    Ok(())
}

/// タスクを削除
async fn handle_delete(
    task_repo: Arc<dyn TaskRepository>,
    presenter: Arc<dyn Presenter>,
    id: i32,
) -> Result<()> {
    // 確認
    let confirm = presenter.confirm(&format!("Delete task ID {}?", id), false)?;

    if !confirm {
        presenter.present_success("Deletion cancelled")?;
        return Ok(());
    }

    let use_case = DeleteTaskUseCase::new(task_repo);
    use_case.execute(id).await?;

    presenter.present_success(&format!("Task ID {id} deleted"))?;

    Ok(())
}

async fn handle_edit(
    task_repo: Arc<dyn TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
    id: i32,
    params: EditTaskParams,
) -> Result<()> {
    // タグIDの検証（指定されている場合）
    if let Some(ref ids) = params.tags {
        validate_tag_ids(&tag_repo, ids).await?;
    }

    // 引数モードか対話モードか判定
    let is_interactive = params.title.is_none()
        && params.description.is_none()
        && params.status.is_none()
        && params.priority.is_none()
        && params.tags.is_none()
        && params.due_date.is_none()
        && !params.clear_due_date;

    let (
        final_title,
        final_description,
        final_status,
        final_priority,
        final_tags,
        final_due_date,
        final_clear_due_date,
    ) = if is_interactive {
        // 対話モード: 既存のタスク情報を取得
        let use_case = ShowTaskUseCase::new(task_repo.clone(), tag_repo.clone());
        let current_task = use_case.execute(id).await?;

        presenter.present_task_detail(&current_task)?;
        println!(); // 空行を追加

        // 編集するフィールドを選択
        let field_options = vec!["Title", "Description", "Status", "Priority", "Tags", "Due Date"];

        let selected_fields = MultiSelect::new(
            "Select fields to edit (Space to select, Enter to confirm)",
            field_options,
        )
        .with_vim_mode(true)
        .prompt()
        .unwrap_or_default();

        // 選択されたフィールドのみ編集
        let new_title = if selected_fields.contains(&"Title") {
            Some(
                Text::new("Title:")
                    .with_default(&current_task.title)
                    .with_validator(validator::MinLengthValidator::new(1))
                    .prompt()
                    .context("Failed to input title")?,
            )
        } else {
            None
        };

        let new_description = if selected_fields.contains(&"Description") {
            Some(
                Editor::new("Enter description")
                    .with_predefined_text(current_task.description.as_deref().unwrap_or(""))
                    .prompt()
                    .unwrap_or_default(),
            )
        } else {
            None
        };

        let new_status = if selected_fields.contains(&"Status") {
            let current_status =
                Status::try_from(current_task.status.as_str()).unwrap_or(Status::Pending);
            Some(
                Select::new("Status:", Status::iter().collect::<Vec<_>>())
                    .with_starting_cursor(
                        Status::iter()
                            .position(|s| s == current_status)
                            .unwrap_or(0),
                    )
                    .with_vim_mode(true)
                    .prompt()
                    .unwrap_or(current_status),
            )
        } else {
            None
        };

        let new_priority = if selected_fields.contains(&"Priority") {
            let current_priority =
                Priority::try_from(current_task.priority.as_str()).unwrap_or(Priority::Medium);
            Some(
                Select::new("Priority:", Priority::iter().collect::<Vec<_>>())
                    .with_starting_cursor(
                        Priority::iter()
                            .position(|p| p == current_priority)
                            .unwrap_or(1),
                    )
                    .with_vim_mode(true)
                    .prompt()
                    .unwrap_or(current_priority),
            )
        } else {
            None
        };

        let new_tags = if selected_fields.contains(&"Tags") {
            let available_tags = tag_repo.find_all().await?;
            if !available_tags.is_empty() {
                let tag_options: Vec<TagOption> = available_tags
                    .iter()
                    .map(|t| TagOption {
                        id: t.id().value(),
                        display: format!("[{}] {}", t.id().value(), t.name().value()),
                    })
                    .collect();

                // 既存のタグIDを取得
                let current_tag_ids: Vec<i32> = current_task.tags.iter().map(|t| t.id).collect();

                // デフォルト選択のインデックスを計算
                let default_indices: Vec<usize> = tag_options
                    .iter()
                    .enumerate()
                    .filter(|(_, opt)| current_tag_ids.contains(&opt.id))
                    .map(|(idx, _)| idx)
                    .collect();

                let selected = MultiSelect::new(
                    "Select tags (Space to select, Enter to confirm)",
                    tag_options,
                )
                .with_default(&default_indices)
                .with_vim_mode(true)
                .prompt()
                .ok();

                // キャンセルされた場合はNoneを返し、既存のタグを保持
                selected.map(|tags| tags.iter().map(|opt| opt.id).collect())
            } else {
                Some(vec![])
            }
        } else {
            None
        };

        let (new_due_date, clear_due_date) = if selected_fields.contains(&"Due Date") {
            if current_task.due_date.is_some() {
                // 既存の期限がある場合、クリアするか新しい値を設定するか選択
                let options = vec!["Clear due date", "Set new due date"];
                let choice = Select::new("Due date:", options)
                    .with_starting_cursor(1) // デフォルトは「新しい期限を設定」
                    .with_vim_mode(true)
                    .prompt()
                    .unwrap_or("Set new due date");

                if choice == "Clear due date" {
                    (None, true)
                } else {
                    // SAFETY: このブロックに入るのはcurrent_task.due_date.is_some()の時のみ
                    let new_date = DateSelect::new("Select due date")
                        .with_default(current_task.due_date.unwrap())
                        .prompt()
                        .ok();
                    (new_date, false)
                }
            } else {
                // 既存の期限がない場合、新しく設定
                let new_date = DateSelect::new("Select due date").prompt().ok();
                (new_date, false)
            }
        } else {
            (None, false)
        };

        (
            new_title,
            new_description,
            new_status,
            new_priority,
            new_tags,
            new_due_date,
            clear_due_date,
        )
    } else {
        // 引数モード
        (
            params.title,
            params.description,
            params.status,
            params.priority,
            params.tags,
            params.due_date,
            params.clear_due_date,
        )
    };

    // DTOを構築
    let dto = UpdateTaskDTO {
        title: final_title,
        description: final_description,
        status: final_status.map(|s| s.to_string()),
        priority: final_priority.map(|p| p.to_string()),
        tags: final_tags,
        due_date: if final_clear_due_date {
            // 期限をクリアする場合は、use case側で処理する必要がある
            // TODO: DTOにclear_due_dateフィールドを追加するか、別の方法で処理
            None
        } else {
            final_due_date
        },
    };

    // Use Caseを実行
    let use_case = EditTaskUseCase::new(task_repo, tag_repo);
    let updated_task = use_case.execute(id, dto).await?;

    presenter.present_success(&format!(
        "Task updated: [{}] {}",
        updated_task.id, updated_task.title
    ))?;

    Ok(())
}

/// タスクの統計情報を表示
async fn handle_stats(
    task_repo: Arc<dyn TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
) -> Result<()> {
    let use_case = ShowStatsUseCase::new(task_repo, tag_repo);
    let stats = use_case.execute().await?;

    presenter.present_stats(&stats)?;

    Ok(())
}

/// タスクをキーワードで検索
async fn handle_search(
    task_repo: Arc<dyn TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
    presenter: Arc<dyn Presenter>,
    params: SearchParams,
) -> Result<()> {
    // 引数モードか対話モードか判定
    let is_interactive = params.keywords.is_none();

    let final_keywords = if is_interactive {
        // 対話モード: キーワードを入力
        inquire::Text::new("Search keyword:")
            .with_help_message("Multiple keywords can be specified separated by spaces (AND condition)")
            .with_validator(|input: &str| {
                if input.trim().is_empty() {
                    Ok(validator::Validation::Invalid(
                        "Please enter at least one character.".into(),
                    ))
                } else {
                    Ok(validator::Validation::Valid)
                }
            })
            .prompt()
            .context("Keyword input was cancelled")?
    } else {
        // 引数モード
        // SAFETY: is_interactive=falseの場合、params.keywordsはSomeであることが保証されている
        params.keywords.unwrap()
    };

    let search_field = params.field.into();
    let use_case = SearchTasksUseCase::new(task_repo, tag_repo);
    let tasks = use_case.execute(&final_keywords, search_field).await?;

    if tasks.is_empty() {
        println!(
            "No tasks found matching search keyword \"{}\"",
            final_keywords
        );
    } else {
        println!("Search results ({} items):", tasks.len());
        presenter.present_task_list(&tasks)?;
    }

    Ok(())
}
