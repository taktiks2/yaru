use anyhow::{Context, Result};
use chrono::NaiveDate;
use inquire::{DateSelect, Editor, MultiSelect, Select, Text, validator};
use std::sync::Arc;

use crate::application::dto::task_dto::{CreateTaskDTO, UpdateTaskDTO};
use crate::application::use_cases::task::{
    add_task::AddTaskUseCase, delete_task::DeleteTaskUseCase, edit_task::EditTaskUseCase,
    list_tasks::ListTasksUseCase, show_stats::ShowStatsUseCase, show_task::ShowTaskUseCase,
};
use crate::domain::tag::repository::TagRepository;
use crate::domain::task::value_objects::{Priority, Status};
use crate::interface::cli::args::{Filter, TaskCommands};
use crate::interface::cli::display::{
    create_stats_table, create_task_detail_table, create_task_table,
};

/// タスクコマンドを処理
pub async fn handle_task_command(
    command: TaskCommands,
    task_repo: Arc<dyn crate::domain::task::repository::TaskRepository>,
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
            handle_add(
                task_repo,
                tag_repo,
                title,
                description,
                status,
                priority,
                tags,
                due_date,
            )
            .await
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
            handle_edit(
                task_repo,
                tag_repo,
                id,
                title,
                description,
                status,
                priority,
                tags,
                due_date,
                clear_due_date,
            )
            .await
        }
        TaskCommands::Stats => handle_stats(task_repo).await,
    }
}

/// タスク一覧を表示
async fn handle_list(
    task_repo: Arc<dyn crate::domain::task::repository::TaskRepository>,
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
async fn handle_show(
    task_repo: Arc<dyn crate::domain::task::repository::TaskRepository>,
    id: i32,
) -> Result<()> {
    let use_case = ShowTaskUseCase::new(task_repo);
    let task = use_case.execute(id).await?;

    let table = create_task_detail_table(&task);
    println!("{}", table);

    Ok(())
}

/// 新しいタスクを追加
#[allow(clippy::too_many_arguments)]
async fn handle_add(
    task_repo: Arc<dyn crate::domain::task::repository::TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
    tag_ids: Option<Vec<i32>>,
    due_date: Option<NaiveDate>,
) -> Result<()> {
    // 引数モードか対話モードか判定
    let is_interactive = title.is_none();

    // タグIDの検証（指定されている場合）
    if let Some(ref ids) = tag_ids {
        for id in ids {
            let tag = tag_repo
                .find_by_id(&crate::domain::tag::value_objects::TagId::new(*id)?)
                .await?;
            if tag.is_none() {
                anyhow::bail!("存在しないタグID: {}", id);
            }
        }
    }

    let (final_title, final_description, final_status, final_priority, final_tags, final_due_date) =
        if is_interactive {
            // 対話モード
            let t = Text::new("タスクのタイトルを入力してください")
                .with_validator(validator::MinLengthValidator::new(1))
                .prompt()
                .context("タスクのタイトルの入力に失敗しました")?;

            let d = description.unwrap_or_else(|| {
                Editor::new("タスクの説明を入力してください")
                    .prompt()
                    .unwrap_or_default()
            });

            let s = status.unwrap_or_else(|| {
                Select::new(
                    "ステータスを選択してください",
                    vec![Status::Pending, Status::InProgress, Status::Completed],
                )
                .with_vim_mode(true)
                .prompt()
                .unwrap_or(Status::Pending)
            });

            let p = priority.unwrap_or_else(|| {
                Select::new(
                    "優先度を選択してください",
                    vec![
                        Priority::Low,
                        Priority::Medium,
                        Priority::High,
                        Priority::Critical,
                    ],
                )
                .with_vim_mode(true)
                .prompt()
                .unwrap_or(Priority::Medium)
            });

            // タグ選択（対話モード）
            let tags = if tag_ids.is_some() {
                tag_ids.unwrap_or_default()
            } else {
                // 利用可能なタグを取得
                let available_tags = tag_repo.find_all().await?;
                if !available_tags.is_empty() {
                    let tag_options: Vec<String> = available_tags
                        .iter()
                        .map(|t| format!("[{}] {}", t.id().value(), t.name().value()))
                        .collect();

                    let selected = MultiSelect::new(
                        "タグを選択してください（スペースで選択、Enterで確定）",
                        tag_options,
                    )
                    .with_vim_mode(true)
                    .prompt()
                    .unwrap_or_default();

                    selected
                        .iter()
                        .filter_map(|s| {
                            s.split(']')
                                .next()?
                                .trim_start_matches('[')
                                .parse::<i32>()
                                .ok()
                        })
                        .collect()
                } else {
                    vec![]
                }
            };

            // 期限選択
            let dd = due_date.or_else(|| {
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
                title.unwrap(),
                description.unwrap_or_default(),
                status.unwrap_or(Status::Pending),
                priority.unwrap_or(Priority::Medium),
                tag_ids.unwrap_or_default(),
                due_date,
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
async fn handle_delete(
    task_repo: Arc<dyn crate::domain::task::repository::TaskRepository>,
    id: i32,
) -> Result<()> {
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
#[allow(clippy::too_many_arguments)]
async fn handle_edit(
    task_repo: Arc<dyn crate::domain::task::repository::TaskRepository>,
    tag_repo: Arc<dyn TagRepository>,
    id: i32,
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
    tags: Option<Vec<i32>>,
    due_date: Option<NaiveDate>,
    clear_due_date: bool,
) -> Result<()> {
    // タグIDの検証（指定されている場合）
    if let Some(ref ids) = tags {
        for tag_id in ids {
            let tag = tag_repo
                .find_by_id(&crate::domain::tag::value_objects::TagId::new(*tag_id)?)
                .await?;
            if tag.is_none() {
                anyhow::bail!("存在しないタグID: {}", tag_id);
            }
        }
    }

    // DTOを構築
    // due_dateの処理: clear_due_dateが指定されている場合は扱いを変える
    // UpdateTaskDTOではOption<NaiveDate>なので、Noneを渡すことで「変更しない」、
    // clear_due_dateの場合は別途処理が必要（現在のDTOでは対応していないため、due_dateをNoneのままにする）
    let dto = UpdateTaskDTO {
        title,
        description,
        status: status.map(|s| s.to_string()),
        priority: priority.map(|p| p.to_string()),
        tags,
        due_date: if clear_due_date {
            // 期限をクリアする場合は、use case側で処理する必要がある
            // TODO: DTOにclear_due_dateフィールドを追加するか、別の方法で処理
            None
        } else {
            due_date
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
async fn handle_stats(
    task_repo: Arc<dyn crate::domain::task::repository::TaskRepository>,
) -> Result<()> {
    let use_case = ShowStatsUseCase::new(task_repo);
    let stats = use_case.execute().await?;

    let table = create_stats_table(&stats);
    println!("{}", table);

    Ok(())
}
