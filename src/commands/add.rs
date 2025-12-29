use crate::{
    display::create_single_task_table,
    repository::TaskRepository,
    task::{Priority, Status, Task},
};
use anyhow::{Context, Result};
use inquire::{Editor, Text, validator};

/// 新しいタスクを追加
pub fn add_task(
    repo: &impl TaskRepository,
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
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

    let mut tasks = repo.load_tasks()?;
    let new_id = repo.find_next_id(&tasks);
    let new_task = Task::new(new_id, &title, &description, status, priority);

    tasks.push(new_task.clone());
    repo.save_tasks(&tasks)?;

    println!("タスクを登録しました。");

    let table = create_single_task_table(&new_task);
    println!("{table}");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add_task_with_title_and_status() {
        // タイトルとステータスを指定してタスクを追加する場合のテスト
        // Note: このテストは実装後に適切な形に修正する必要がある
    }

    #[test]
    fn test_add_task_with_only_title() {
        // タイトルのみを指定してタスクを追加する場合のテスト
        // デフォルトステータスがPendingになることを確認
    }

    #[test]
    fn test_add_task_without_title() {
        // タイトルなしで追加する場合のテスト
        // 対話的入力が必要になるため、統合テストで実装する
    }
}
