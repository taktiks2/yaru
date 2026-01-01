use crate::{
    display::create_task_detail_table,
    domain::task::{Priority, Status, Task},
    repository::{Repository, tag::TagRepository, task::TaskRepository},
};
use anyhow::{Context, Result};
use clap::ValueEnum;
use inquire::{Editor, MultiSelect, Select, Text, validator};
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
    // タグリポジトリから全タグを取得（引数モードとインタラクティブモード両方で使用）
    let tag_repo = TagRepository::new(db);
    let available_tags = tag_repo.find_all().await?;

    let (title, description, status, priority, tags) = match title {
        Some(title) => {
            anyhow::ensure!(!title.is_empty(), "タイトルは空にできません");

            let tags = match tag_ids {
                Some(ref ids) => ids
                    .iter()
                    .map(|id| {
                        available_tags
                            .iter()
                            .find(|t| t.id == *id)
                            .cloned()
                            .ok_or_else(|| anyhow::anyhow!("存在しないタグID: {}", id))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                None => vec![],
            };

            (
                title,
                description.unwrap_or_default(),
                status.unwrap_or(Status::Pending),
                priority.unwrap_or(Priority::Medium),
                tags,
            )
        }
        None => {
            let t = Text::new("タスクのタイトルを入力してください")
                .with_validator(validator::MinLengthValidator::new(1))
                .prompt()
                .context("タスクのタイトルの入力に失敗しました")?;
            let d = Editor::new("タスクの説明を入力してください")
                .prompt()
                .unwrap_or_default();
            let s = Select::new(
                "ステータスを選択してください",
                Status::value_variants().to_vec(),
            )
            .with_vim_mode(true)
            .prompt()
            .unwrap_or(Status::Pending);
            let p = Select::new(
                "優先度を選択してください",
                Priority::value_variants().to_vec(),
            )
            .with_vim_mode(true)
            .prompt()
            .unwrap_or(Priority::Medium);

            // タグ選択
            let tags = if available_tags.is_empty() {
                // タグが0件の場合は空のVecを返す
                Vec::new()
            } else {
                // MultiSelectでタグを選択
                MultiSelect::new(
                    "タスクに紐づけるタグを選択してください（スペースで選択、Enterで確定）",
                    available_tags.clone(),
                )
                .with_vim_mode(true)
                .prompt()
                .unwrap_or_default()
            };

            (t, d, s, p, tags)
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::{tag::TagRepository, task::TaskRepository};
    use migration::MigratorTrait;
    use sea_orm::Database;

    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();
        db
    }

    // テスト1: 引数モードで正しいタグIDを指定した場合
    #[tokio::test]
    async fn test_add_task_with_valid_tag_ids() {
        let db = setup_test_db().await;

        // タグを作成
        let tag_repo = TagRepository::new(&db);
        let tag1 = tag_repo
            .create(&crate::domain::tag::Tag::new(0, "重要", ""))
            .await
            .unwrap();
        let tag2 = tag_repo
            .create(&crate::domain::tag::Tag::new(0, "緊急", ""))
            .await
            .unwrap();

        // タスクを作成
        let result = add_task(
            &db,
            Some("テストタスク".to_string()),
            Some("テスト説明".to_string()),
            None,
            None,
            Some(vec![tag1.id, tag2.id]),
        )
        .await;

        assert!(result.is_ok());

        // タスクが正しく作成されたか確認
        let task_repo = TaskRepository::new(&db);
        let tasks = task_repo.find_all().await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].tags.len(), 2);
    }

    // テスト2: 引数モードで存在しないタグIDを指定した場合
    #[tokio::test]
    async fn test_add_task_with_invalid_tag_ids() {
        let db = setup_test_db().await;

        // 存在しないタグIDでタスクを作成
        let result = add_task(
            &db,
            Some("テストタスク".to_string()),
            Some("テスト説明".to_string()),
            None,
            None,
            Some(vec![999]),
        )
        .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("存在しないタグID"));
    }

    // テスト3: タグなしでタスクを作成した場合
    #[tokio::test]
    async fn test_add_task_without_tags() {
        let db = setup_test_db().await;

        let result = add_task(
            &db,
            Some("テストタスク".to_string()),
            Some("テスト説明".to_string()),
            None,
            None,
            None,
        )
        .await;

        assert!(result.is_ok());

        let task_repo = TaskRepository::new(&db);
        let tasks = task_repo.find_all().await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].tags.len(), 0);
    }
}
