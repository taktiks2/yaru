use crate::{
    application::{
        dto::task_dto::{CreateTaskDTO, UpdateTaskDTO},
        use_cases::task::{
            add_task::AddTaskUseCase, delete_task::DeleteTaskUseCase, edit_task::EditTaskUseCase,
            list_tasks::ListTasksUseCase, show_stats::ShowStatsUseCase, show_task::ShowTaskUseCase,
        },
    },
    domain::{
        tag::{repository::TagRepository, value_objects::TagId},
        task::{
            repository::TaskRepository,
            value_objects::{Priority, Status},
        },
    },
    interface::cli::{
        args::{Filter, TaskCommands},
        display::{create_stats_table, create_task_detail_table, create_task_table},
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
/// find_by_idsを使用してN+1問題を回避します
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
            anyhow::bail!("存在しないタグID: {}", id);
        }
    }

    Ok(())
}

/// タスクコマンドを処理
pub async fn handle_task_command(
    command: TaskCommands,
    task_repo: Arc<dyn TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
) -> Result<()> {
    match command {
        TaskCommands::List { filter } => handle_list(task_repo, filter).await,
        TaskCommands::Show { id } => handle_show(task_repo, id).await,
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
            handle_add(task_repo, tag_repo, params).await
        }
        TaskCommands::Delete { id } => handle_delete(task_repo, id).await,
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
            handle_edit(task_repo, tag_repo, id, params).await
        }
        TaskCommands::Stats => handle_stats(task_repo).await,
    }
}

/// タスク一覧を表示
async fn handle_list(
    task_repo: Arc<dyn TaskRepository>,
    _filter: Option<Vec<Filter>>,
) -> Result<()> {
    let use_case = ListTasksUseCase::new(task_repo);
    let tasks = use_case.execute().await?;

    // TODO: フィルタ処理を実装
    if tasks.is_empty() {
        println!("タスクがありません");
    } else {
        println!("タスク一覧 ({}件):", tasks.len());
        let table = create_task_table(&tasks);
        println!("{}", table);
    }

    Ok(())
}

/// タスクの詳細を表示
async fn handle_show(task_repo: Arc<dyn TaskRepository>, id: i32) -> Result<()> {
    let use_case = ShowTaskUseCase::new(task_repo);
    let task = use_case.execute(id).await?;

    let table = create_task_detail_table(&task);
    println!("{}", table);

    Ok(())
}

/// 新しいタスクを追加
async fn handle_add(
    task_repo: Arc<dyn TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
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
            let t = Text::new("タスクのタイトルを入力してください")
                .with_validator(validator::MinLengthValidator::new(1))
                .prompt()
                .context("タスクのタイトルの入力に失敗しました")?;

            let d = params.description.unwrap_or_else(|| {
                Editor::new("タスクの説明を入力してください")
                    .prompt()
                    .unwrap_or_default()
            });

            let s = params.status.unwrap_or_else(|| {
                Select::new(
                    "ステータスを選択してください",
                    Status::iter().collect::<Vec<_>>(),
                )
                .with_vim_mode(true)
                .prompt()
                .unwrap_or(Status::Pending)
            });

            let p = params.priority.unwrap_or_else(|| {
                Select::new(
                    "優先度を選択してください",
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
                        "タグを選択してください（スペースで選択、Enterで確定）",
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
                if inquire::Confirm::new("期限を設定しますか？")
                    .with_default(false)
                    .prompt()
                    .unwrap_or(false)
                {
                    DateSelect::new("期限を選択してください").prompt().ok()
                } else {
                    None
                }
            });

            (t, d, s, p, tags, dd)
        } else {
            // 引数モード
            (
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

    println!(
        "タスクを追加しました: [{}] {}",
        created_task.id, created_task.title
    );

    Ok(())
}

/// タスクを削除
async fn handle_delete(task_repo: Arc<dyn TaskRepository>, id: i32) -> Result<()> {
    // 確認
    let confirm = inquire::Confirm::new(&format!("タスクID {}を削除しますか？", id))
        .with_default(false)
        .prompt()
        .unwrap_or(false);

    if !confirm {
        println!("削除をキャンセルしました");
        return Ok(());
    }

    let use_case = DeleteTaskUseCase::new(task_repo);
    use_case.execute(id).await?;

    println!("タスクID {}を削除しました", id);

    Ok(())
}

/// タスクを編集
async fn handle_edit(
    task_repo: Arc<dyn TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
    id: i32,
    params: EditTaskParams,
) -> Result<()> {
    // タグIDの検証（指定されている場合）
    if let Some(ref ids) = params.tags {
        validate_tag_ids(&tag_repo, ids).await?;
    }

    // DTOを構築
    // due_dateの処理: clear_due_dateが指定されている場合は扱いを変える
    // UpdateTaskDTOではOption<NaiveDate>なので、Noneを渡すことで「変更しない」、
    // clear_due_dateの場合は別途処理が必要（現在のDTOでは対応していないため、due_dateをNoneのままにする）
    let dto = UpdateTaskDTO {
        title: params.title,
        description: params.description,
        status: params.status.map(|s| s.to_string()),
        priority: params.priority.map(|p| p.to_string()),
        tags: params.tags,
        due_date: if params.clear_due_date {
            // 期限をクリアする場合は、use case側で処理する必要がある
            // TODO: DTOにclear_due_dateフィールドを追加するか、別の方法で処理
            None
        } else {
            params.due_date
        },
    };

    // Use Caseを実行
    let use_case = EditTaskUseCase::new(task_repo, tag_repo);
    let updated_task = use_case.execute(id, dto).await?;

    println!(
        "タスクを更新しました: [{}] {}",
        updated_task.id, updated_task.title
    );

    Ok(())
}

/// タスクの統計情報を表示
async fn handle_stats(task_repo: Arc<dyn TaskRepository>) -> Result<()> {
    let use_case = ShowStatsUseCase::new(task_repo);
    let stats = use_case.execute().await?;

    let table = create_stats_table(&stats);
    println!("{}", table);

    Ok(())
}
