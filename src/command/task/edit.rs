use crate::{
    display::create_task_detail_table,
    domain::task::{Priority, Status, Task},
    repository::{Repository, tag::TagRepository, task::TaskRepository},
};
use anyhow::{Context, Result};
use chrono::NaiveDate;
use clap::ValueEnum;
use inquire::{DateSelect, Editor, MultiSelect, Select, Text, validator};
use sea_orm::DatabaseConnection;

/// タスクを編集
#[allow(clippy::too_many_arguments)]
pub async fn edit_task(
    db: &DatabaseConnection,
    id: i32,
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
    tag_ids: Option<Vec<i32>>,
    due_date: Option<NaiveDate>,
    clear_due_date: bool,
) -> Result<()> {
    // 1. 既存タスクを取得
    let task_repo = TaskRepository::new(db);
    let existing_task = task_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("ID {} のタスクが見つかりません", id))?;

    // 2. 利用可能なタグを取得（対話モード用）
    let tag_repo = TagRepository::new(db);
    let available_tags = tag_repo.find_all().await?;

    // 3. 引数モードか対話モードか判定
    let is_interactive = title.is_none()
        && description.is_none()
        && status.is_none()
        && priority.is_none()
        && tag_ids.is_none()
        && due_date.is_none()
        && !clear_due_date;

    let (new_title, new_description, new_status, new_priority, new_tags, new_due_date) =
        if is_interactive {
            // 対話モード: 現在値をデフォルトとして表示
            let t = Text::new("タスクのタイトル:")
                .with_initial_value(&existing_task.title)
                .with_validator(validator::MinLengthValidator::new(1))
                .prompt()
                .context("タスクのタイトルの入力に失敗しました")?;

            let d = Editor::new("タスクの説明:")
                .with_predefined_text(&existing_task.description)
                .prompt()
                .unwrap_or_else(|_| existing_task.description.clone());

            let s = Select::new("ステータス:", Status::value_variants().to_vec())
                .with_starting_cursor(
                    Status::value_variants()
                        .iter()
                        .position(|v| *v == existing_task.status)
                        .unwrap_or(0),
                )
                .with_vim_mode(true)
                .prompt()
                .unwrap_or(existing_task.status);

            let p = Select::new("優先度:", Priority::value_variants().to_vec())
                .with_starting_cursor(
                    Priority::value_variants()
                        .iter()
                        .position(|v| *v == existing_task.priority)
                        .unwrap_or(0),
                )
                .with_vim_mode(true)
                .prompt()
                .unwrap_or(existing_task.priority);

            // タグ選択（現在のタグを初期選択）
            let tags = if available_tags.is_empty() {
                Vec::new()
            } else {
                let current_tag_ids: Vec<i32> = existing_task.tags.iter().map(|t| t.id).collect();
                let default_indices: Vec<usize> = current_tag_ids
                    .iter()
                    .filter_map(|&id| available_tags.iter().position(|t| t.id == id))
                    .collect();

                MultiSelect::new(
                    "タスクに紐づけるタグを選択してください（スペースで選択、Enterで確定）:",
                    available_tags.clone(),
                )
                .with_default(&default_indices)
                .with_vim_mode(true)
                .prompt()
                .unwrap_or_else(|_| existing_task.tags.clone())
            };

            // 期限の入力
            let due = if let Some(current_due) = existing_task.due_date {
                // 既存の期限がある場合：3択で明示的に選択
                let options = vec!["維持", "変更", "クリア"];
                let choice = Select::new("期限の設定:", options)
                    .with_starting_cursor(0)
                    .with_vim_mode(true)
                    .prompt()
                    .unwrap_or("維持");

                match choice {
                    "維持" => Some(current_due),
                    "クリア" => None,
                    "変更" => DateSelect::new("新しい期限を選択してください:")
                        .with_starting_date(current_due)
                        .prompt()
                        .ok(),
                    _ => Some(current_due),
                }
            } else {
                // 期限がない場合：直接入力（Escでスキップ）
                DateSelect::new("期限を設定しますか？（Escでスキップ）:")
                    .prompt()
                    .ok()
            };

            (t, d, s, p, tags, due)
        } else {
            // 引数モード: 指定された引数のみ更新、未指定は現在値を維持
            let new_tags = if let Some(ids) = tag_ids {
                // タグIDが指定された場合、存在確認して取得
                ids.iter()
                    .map(|&id| {
                        available_tags
                            .iter()
                            .find(|t| t.id == id)
                            .cloned()
                            .ok_or_else(|| anyhow::anyhow!("存在しないタグID: {}", id))
                    })
                    .collect::<Result<Vec<_>, _>>()?
            } else {
                existing_task.tags.clone()
            };

            let new_due_date = if clear_due_date {
                None
            } else {
                due_date.or(existing_task.due_date)
            };

            (
                title.unwrap_or(existing_task.title),
                description.unwrap_or(existing_task.description),
                status.unwrap_or(existing_task.status),
                priority.unwrap_or(existing_task.priority),
                new_tags,
                new_due_date,
            )
        };

    // 4. 更新されたタスクを作成
    let updated_task = Task {
        id: existing_task.id,
        title: new_title,
        description: new_description,
        status: new_status,
        priority: new_priority,
        tags: new_tags,
        due_date: new_due_date,
        // これらのフィールドはRepositoryで適切に処理される
        created_at: existing_task.created_at,
        updated_at: existing_task.updated_at,
        completed_at: existing_task.completed_at,
    };

    // 5. リポジトリで更新
    let result = task_repo.update(&updated_task).await?;

    println!("タスクを更新しました。");
    let table = create_task_detail_table(&result);
    println!("{table}");

    Ok(())
}
