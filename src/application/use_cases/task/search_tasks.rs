use crate::{
    application::dto::task_dto::{TagInfo, TaskDTO},
    domain::{
        tag::repository::TagRepository,
        task::{
            repository::TaskRepository,
            specification::{SearchField, TaskByKeyword},
        },
    },
};
use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

/// SearchTasksUseCase - タスク検索のユースケース
///
/// キーワードでタスクを検索してDTOに変換します。
/// タグ情報はTagRepositoryから一括取得し、N+1問題を回避します。
pub struct SearchTasksUseCase {
    task_repository: Arc<dyn TaskRepository>,
    tag_repository: Arc<dyn TagRepository>,
}

impl SearchTasksUseCase {
    /// 新しいSearchTasksUseCaseを作成
    pub fn new(
        task_repository: Arc<dyn TaskRepository>,
        tag_repository: Arc<dyn TagRepository>,
    ) -> Self {
        Self {
            task_repository,
            tag_repository,
        }
    }

    /// タスクを検索する
    ///
    /// # Arguments
    /// * `keywords` - 検索キーワード（空白区切りの文字列）
    /// * `field` - 検索対象フィールド
    ///
    /// # Returns
    /// * `Ok(Vec<TaskDTO>)` - 検索結果のタスクリスト
    /// * `Err` - エラーが発生した場合
    pub async fn execute(&self, keywords: &str, field: SearchField) -> Result<Vec<TaskDTO>> {
        // 1. キーワードを分割してSpecificationを作成
        let keyword_vec: Vec<String> = keywords.split_whitespace().map(|s| s.to_string()).collect();
        let spec = Box::new(TaskByKeyword::new(keyword_vec, field));

        // 2. Specificationに基づいてタスクを検索
        let tasks = self.task_repository.find_by_specification(spec).await?;

        // 3. 全タスクのタグIDを収集（重複排除）
        let all_tag_ids: HashSet<_> = tasks
            .iter()
            .flat_map(|task| task.tags().iter().copied())
            .collect();

        // 4. タグ情報を一括取得（N+1問題の回避）
        let tag_ids_vec: Vec<_> = all_tag_ids.into_iter().collect();
        let tags = self.tag_repository.find_by_ids(&tag_ids_vec).await?;

        // 5. TagId -> TagAggregateのマップを作成
        let tag_map: HashMap<_, _> = tags.iter().map(|tag| (tag.id().value(), tag)).collect();

        // 6. TaskDTOに変換（タグ詳細を含む）
        let task_dtos = tasks
            .into_iter()
            .map(|task| self.to_dto_with_tags(&task, &tag_map))
            .collect();

        Ok(task_dtos)
    }

    /// TaskAggregateをTaskDTOに変換（タグ詳細を含む）
    fn to_dto_with_tags(
        &self,
        task: &crate::domain::task::aggregate::TaskAggregate,
        tag_map: &HashMap<i32, &crate::domain::tag::aggregate::TagAggregate>,
    ) -> TaskDTO {
        // タグ情報を解決
        let tag_details = task
            .tags()
            .iter()
            .filter_map(|tag_id| {
                tag_map.get(&tag_id.value()).map(|tag| TagInfo {
                    id: tag.id().value(),
                    name: tag.name().value().to_string(),
                })
            })
            .collect();

        // TaskDTOに変換
        let mut dto = TaskDTO::from(task.clone());
        dto.tags = tag_details;
        dto
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::{
        aggregate::TaskAggregate,
        value_objects::{Priority, Status, TaskDescription, TaskTitle},
    };
    use crate::domain::tag::{
        aggregate::TagAggregate,
        value_objects::{TagDescription, TagName},
    };
    use crate::interface::persistence::in_memory::{InMemoryTagRepository, InMemoryTaskRepository};

    #[tokio::test]
    async fn test_search_tasks_empty_repository() {
        // Arrange: 空のリポジトリ
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());
        let use_case = SearchTasksUseCase::new(task_repo, tag_repo);

        // Act: 検索実行
        let result = use_case.execute("買い物", SearchField::All).await;

        // Assert: 空の結果が返る
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_search_tasks_single_keyword_match() {
        // Arrange: タスクを1件登録
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let title = TaskTitle::new("買い物リスト").unwrap();
        let description = TaskDescription::new("牛乳を買う").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        task_repo.save(task).await.unwrap();

        let use_case = SearchTasksUseCase::new(task_repo, tag_repo);

        // Act: 「買い物」で検索
        let result = use_case.execute("買い物", SearchField::All).await;

        // Assert: 1件マッチ
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "買い物リスト");
    }

    #[tokio::test]
    async fn test_search_tasks_multiple_keywords_and_condition() {
        // Arrange: タスクを2件登録
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let title1 = TaskTitle::new("レポート作成").unwrap();
        let description1 = TaskDescription::new("月次レポートを作成する").unwrap();
        let task1 = TaskAggregate::new(
            title1,
            description1,
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );
        task_repo.save(task1).await.unwrap();

        let title2 = TaskTitle::new("レポート提出").unwrap();
        let description2 = TaskDescription::new("レポートを提出する").unwrap();
        let task2 = TaskAggregate::new(
            title2,
            description2,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        task_repo.save(task2).await.unwrap();

        let use_case = SearchTasksUseCase::new(task_repo, tag_repo);

        // Act: 「レポート 作成」で検索（AND条件）
        let result = use_case.execute("レポート 作成", SearchField::All).await;

        // Assert: 「レポート作成」のみマッチ（「レポート提出」はマッチしない）
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "レポート作成");
    }

    #[tokio::test]
    async fn test_search_tasks_field_title_only() {
        // Arrange: タスクを登録
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let title = TaskTitle::new("買い物リスト").unwrap();
        let description = TaskDescription::new("牛乳を買う").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        task_repo.save(task).await.unwrap();

        let use_case = SearchTasksUseCase::new(task_repo, tag_repo);

        // Act: タイトルのみ検索で「買い物」を検索
        let result = use_case.execute("買い物", SearchField::Title).await;

        // Assert: 1件マッチ
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);

        // Act: タイトルのみ検索で「牛乳」を検索
        let result2 = use_case.execute("牛乳", SearchField::Title).await;

        // Assert: 「牛乳」はタイトルにないのでマッチしない
        assert!(result2.is_ok());
        let tasks2 = result2.unwrap();
        assert_eq!(tasks2.len(), 0);
    }

    #[tokio::test]
    async fn test_search_tasks_field_description_only() {
        // Arrange: タスクを登録
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let title = TaskTitle::new("買い物リスト").unwrap();
        let description = TaskDescription::new("牛乳を買う").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        task_repo.save(task).await.unwrap();

        let use_case = SearchTasksUseCase::new(task_repo, tag_repo);

        // Act: 説明のみ検索で「牛乳」を検索
        let result = use_case.execute("牛乳", SearchField::Description).await;

        // Assert: 1件マッチ
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);

        // Act: 説明のみ検索で「買い物」を検索
        let result2 = use_case.execute("買い物", SearchField::Description).await;

        // Assert: 「買い物」は説明にないのでマッチしない
        assert!(result2.is_ok());
        let tasks2 = result2.unwrap();
        assert_eq!(tasks2.len(), 0);
    }

    #[tokio::test]
    async fn test_search_tasks_no_match() {
        // Arrange: タスクを登録
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let title = TaskTitle::new("買い物リスト").unwrap();
        let description = TaskDescription::new("牛乳を買う").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        task_repo.save(task).await.unwrap();

        let use_case = SearchTasksUseCase::new(task_repo, tag_repo);

        // Act: マッチしないキーワードで検索
        let result = use_case.execute("会議", SearchField::All).await;

        // Assert: 0件
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_search_tasks_with_tags() {
        // Arrange: タグ付きタスクを登録
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        // タグを作成
        let tag_name = TagName::new("買い物").unwrap();
        let tag_description = TagDescription::new("").unwrap();
        let tag = TagAggregate::new(tag_name, tag_description);
        let saved_tag = tag_repo.save(tag).await.unwrap();
        let tag_id = saved_tag.id();

        // タグ付きタスクを作成
        let title = TaskTitle::new("買い物リスト").unwrap();
        let description = TaskDescription::new("牛乳を買う").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![*tag_id],
            None,
        );
        task_repo.save(task).await.unwrap();

        let use_case = SearchTasksUseCase::new(task_repo, tag_repo);

        // Act: 検索実行
        let result = use_case.execute("買い物", SearchField::All).await;

        // Assert: タグ情報も含めて返却される
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].tags.len(), 1);
        assert_eq!(tasks[0].tags[0].name, "買い物");
    }

    #[tokio::test]
    async fn test_search_tasks_case_insensitive() {
        // Arrange: タスクを登録
        let task_repo = Arc::new(InMemoryTaskRepository::new());
        let tag_repo = Arc::new(InMemoryTagRepository::new());

        let title = TaskTitle::new("Bug Report").unwrap();
        let description = TaskDescription::new("Fix critical bug").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );
        task_repo.save(task).await.unwrap();

        let use_case = SearchTasksUseCase::new(task_repo, tag_repo);

        // Act: 小文字で検索
        let result = use_case.execute("bug", SearchField::All).await;

        // Assert: 大文字小文字を無視してマッチ
        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Bug Report");
    }
}
