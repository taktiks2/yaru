use crate::{
    display::create_task_detail_table,
    domain::task::{Priority, Status, Task},
    repository::{Repository, tag::TagRepository, task::TaskRepository},
};
use anyhow::{Context, Result};
use inquire::{Editor, Text, validator};
use sea_orm::DatabaseConnection;

/// 新しいタスクを追加
pub async fn add_task(
    db: &DatabaseConnection,
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
    tag_ids: Option<Vec<i32>>,
) -> Result<()> {
    let title = match title {
        Some(t) => t,
        None => Text::new("タスクのタイトルを入力してください")
            .with_validator(validator::MinLengthValidator::new(1))
            .prompt()
            .context("タスクのタイトルの入力に失敗しました")?,
    };

    let description = match description {
        Some(d) => d,
        None => Editor::new("タスクの説明を入力してください")
            .prompt()
            .context("タスクの説明の入力に失敗しました")?,
    };

    let status = status.unwrap_or(Status::Pending);
    let priority = priority.unwrap_or(Priority::Medium);
    let tag_ids = tag_ids.unwrap_or_default();

    // タグIDからTagオブジェクトへの変換
    let tags = if !tag_ids.is_empty() {
        let tag_repo = TagRepository::new(db);
        let existing_tags = tag_repo.find_all().await?;
        let existing_tag_ids: Vec<i32> = existing_tags.iter().map(|t| t.id).collect();

        // 存在確認
        for tag_id in &tag_ids {
            if !existing_tag_ids.contains(tag_id) {
                anyhow::bail!("存在しないタグID: {}", tag_id);
            }
        }

        // Vec<Tag>に変換
        tag_ids
            .iter()
            .filter_map(|id| existing_tags.iter().find(|t| t.id == *id).cloned())
            .collect()
    } else {
        vec![]
    };

    // リポジトリを使用してタスクを作成
    let new_task = Task::new(0, &title, &description, status, priority, tags);
    let task_repo = TaskRepository::new(db);
    let created_task = task_repo.create(&new_task).await?;

    println!("タスクを登録しました。");

    // all_tagsパラメータ削除
    let table = create_task_detail_table(&created_task);
    println!("{table}");

    Ok(())
}
