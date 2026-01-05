#![allow(dead_code)]

use crate::domain::{
    tag::value_objects::TagId,
    task::{
        aggregate::TaskAggregate,
        value_objects::{Priority, Status, TaskId},
    },
};

/// TaskSpecification trait - タスクの検索条件を抽象化
///
/// Specification Patternにより、複雑なクエリロジックを抽象化します。
/// メモリフィルタとSQLクエリの両方に対応可能な設計です。
///
/// # 使用例
///
/// ```rust,ignore
/// // 単一条件でのフィルタリング
/// let spec = TaskByStatus::new(Status::Pending);
/// let pending_tasks: Vec<TaskAggregate> = all_tasks
///     .into_iter()
///     .filter(|task| spec.is_satisfied_by(task))
///     .collect();
///
/// // AND条件の組み合わせ - 「保留中」かつ「高優先度」のタスク
/// let spec = Box::new(TaskByStatus::new(Status::Pending))
///     .and(Box::new(TaskByPriority::new(Priority::High)));
/// let high_priority_pending: Vec<TaskAggregate> = all_tasks
///     .into_iter()
///     .filter(|task| spec.is_satisfied_by(task))
///     .collect();
///
/// // OR条件の組み合わせ - 「期限切れ」または「高優先度」のタスク
/// let spec = Box::new(TaskOverdue::new())
///     .or(Box::new(TaskByPriority::new(Priority::High)));
/// let urgent_tasks: Vec<TaskAggregate> = all_tasks
///     .into_iter()
///     .filter(|task| spec.is_satisfied_by(task))
///     .collect();
///
/// // 複雑な条件 - (「保留中」かつ「高優先度」) または「期限切れ」
/// let spec = Box::new(TaskByStatus::new(Status::Pending))
///     .and(Box::new(TaskByPriority::new(Priority::High)))
///     .or(Box::new(TaskOverdue::new()));
/// ```
///
/// # 応用例
///
/// Repository実装でSpecificationを受け取り、条件に応じたタスクを取得:
///
/// ```rust,ignore
/// impl TaskRepository for InMemoryTaskRepository {
///     async fn find_by_spec(&self, spec: &dyn TaskSpecification) -> Result<Vec<TaskAggregate>> {
///         let tasks = self.tasks.read().await;
///         Ok(tasks.values()
///             .filter(|task| spec.is_satisfied_by(task))
///             .cloned()
///             .collect())
///     }
/// }
/// ```
pub trait TaskSpecification: Send + Sync {
    /// タスクが条件を満たすかを判定（メモリフィルタ用）
    ///
    /// # Arguments
    /// * `task` - 判定対象のタスク
    ///
    /// # Returns
    /// * `true` - 条件を満たす
    /// * `false` - 条件を満たさない
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool;

    /// 他のSpecificationとAND結合
    fn and(self: Box<Self>, other: Box<dyn TaskSpecification>) -> Box<dyn TaskSpecification>
    where
        Self: 'static + Sized,
    {
        Box::new(AndSpecification {
            left: self,
            right: other,
        })
    }

    /// 他のSpecificationとOR結合
    fn or(self: Box<Self>, other: Box<dyn TaskSpecification>) -> Box<dyn TaskSpecification>
    where
        Self: 'static + Sized,
    {
        Box::new(OrSpecification {
            left: self,
            right: other,
        })
    }
}

/// ステータスでフィルタリング
///
/// # 使用シーン
/// - タスク一覧で「保留中」「進行中」「完了」などステータス別に表示
/// - 統計情報の計算時に、特定ステータスのタスク数をカウント
/// - 完了タスクのみをアーカイブする処理
///
/// # 例
/// ```rust,ignore
/// // 完了したタスクのみを取得
/// let spec = TaskByStatus::new(Status::Completed);
/// let completed_tasks = tasks.into_iter().filter(|t| spec.is_satisfied_by(t)).collect();
/// ```
#[derive(Debug, Clone)]
pub struct TaskByStatus {
    status: Status,
}

impl TaskByStatus {
    pub fn new(status: Status) -> Self {
        Self { status }
    }
}

impl TaskSpecification for TaskByStatus {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.status() == &self.status
    }
}

/// 優先度でフィルタリング
///
/// # 使用シーン
/// - 高優先度のタスクのみを表示して重要なタスクに集中
/// - 優先度別の統計レポートの作成
/// - 低優先度タスクの一括処理（例：一括削除、アーカイブ）
///
/// # 例
/// ```rust,ignore
/// // 高優先度のタスクのみを取得
/// let spec = TaskByPriority::new(Priority::High);
/// let high_priority_tasks = tasks.into_iter().filter(|t| spec.is_satisfied_by(t)).collect();
/// ```
#[derive(Debug, Clone)]
pub struct TaskByPriority {
    priority: Priority,
}

impl TaskByPriority {
    pub fn new(priority: Priority) -> Self {
        Self { priority }
    }
}

impl TaskSpecification for TaskByPriority {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.priority() == &self.priority
    }
}

/// タグでフィルタリング
///
/// # 使用シーン
/// - 特定のプロジェクトやカテゴリのタスクのみを表示
/// - タグ別のタスク数の集計
/// - 特定のタグが付いたタスクの一括操作
///
/// # 例
/// ```rust,ignore
/// // 「仕事」タグが付いたタスクのみを取得
/// let work_tag_id = TagId::new(1).unwrap();
/// let spec = TaskByTag::new(work_tag_id);
/// let work_tasks = tasks.into_iter().filter(|t| spec.is_satisfied_by(t)).collect();
/// ```
#[derive(Debug, Clone)]
pub struct TaskByTag {
    tag_id: TagId,
}

impl TaskByTag {
    pub fn new(tag_id: TagId) -> Self {
        Self { tag_id }
    }
}

impl TaskSpecification for TaskByTag {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.tags().contains(&self.tag_id)
    }
}

/// 期限切れタスクでフィルタリング
///
/// # 使用シーン
/// - 期限切れタスクの警告表示
/// - ダッシュボードで緊急対応が必要なタスクのハイライト
/// - 期限切れタスクの自動通知
///
/// # 例
/// ```rust,ignore
/// // 期限切れのタスクのみを取得
/// let spec = TaskOverdue::new();
/// let overdue_tasks = tasks.into_iter().filter(|t| spec.is_satisfied_by(t)).collect();
/// ```
#[derive(Debug, Clone)]
pub struct TaskOverdue;

impl TaskOverdue {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TaskOverdue {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskSpecification for TaskOverdue {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.is_overdue()
    }
}

/// 検索対象フィールド
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchField {
    /// タイトルのみ
    Title,
    /// 説明のみ
    Description,
    /// タイトルと説明の両方（デフォルト）
    All,
}

/// キーワードでフィルタリング
///
/// # 使用シーン
/// - タスクのタイトルや説明からキーワードで検索
/// - 複数キーワードのAND検索（すべてのキーワードを含む）
/// - 大文字小文字を区別しない検索
///
/// # 例
/// ```rust,ignore
/// // 「買い物」をタイトルと説明から検索
/// let spec = TaskByKeyword::new(vec!["買い物".to_string()], SearchField::All);
///
/// // 「レポート」と「作成」の両方を含むタスクを検索（AND条件）
/// let spec = TaskByKeyword::new(
///     vec!["レポート".to_string(), "作成".to_string()],
///     SearchField::All
/// );
/// ```
#[derive(Debug, Clone)]
pub struct TaskByKeyword {
    keywords: Vec<String>,
    field: SearchField,
}

impl TaskByKeyword {
    pub fn new(keywords: Vec<String>, field: SearchField) -> Self {
        // キーワードを小文字に正規化（大文字小文字を無視するため）
        // 空白のみのキーワードは除外
        let normalized_keywords: Vec<String> = keywords
            .into_iter()
            .map(|k| k.to_lowercase().trim().to_string())
            .filter(|k| !k.is_empty())
            .collect();

        Self {
            keywords: normalized_keywords,
            field,
        }
    }

    /// タスクのテキストを検索対象として取得
    fn get_searchable_text(&self, task: &TaskAggregate) -> String {
        let title = task.title().value();
        let description = task.description().value();

        match self.field {
            SearchField::Title => title.to_lowercase(),
            SearchField::Description => description.to_lowercase(),
            SearchField::All => format!("{} {}", title, description).to_lowercase(),
        }
    }
}

impl TaskSpecification for TaskByKeyword {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        // キーワードが空の場合は全てマッチ
        if self.keywords.is_empty() {
            return true;
        }

        let searchable_text = self.get_searchable_text(task);

        // すべてのキーワードが含まれているか（AND条件）
        self.keywords
            .iter()
            .all(|keyword| searchable_text.contains(keyword))
    }
}

/// IDでフィルタリング
///
/// # 使用シーン
/// - 特定のタスクの存在確認
/// - タスクIDを指定した検索処理
/// - 複数のタスクIDから一致するものを抽出
///
/// # 例
/// ```rust,ignore
/// // 特定IDのタスクのみを取得
/// let task_id = TaskId::new(42).unwrap();
/// let spec = TaskById::new(task_id);
/// let task = tasks.into_iter().find(|t| spec.is_satisfied_by(t));
/// ```
#[derive(Debug, Clone)]
pub struct TaskById {
    id: TaskId,
}

impl TaskById {
    pub fn new(id: TaskId) -> Self {
        Self { id }
    }
}

impl TaskSpecification for TaskById {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        task.id() == &self.id
    }
}

/// AND条件
///
/// 2つのSpecificationを組み合わせて、両方の条件を満たすタスクのみを抽出します。
///
/// # 使用シーン
/// - 「保留中」かつ「高優先度」のように、複数条件を同時に満たすタスクの抽出
/// - 「特定タグ」かつ「期限切れ」のような組み合わせでの絞り込み
///
/// # 例
/// ```rust,ignore
/// // 「保留中」かつ「高優先度」のタスク
/// let spec = Box::new(TaskByStatus::new(Status::Pending))
///     .and(Box::new(TaskByPriority::new(Priority::High)));
/// ```
pub struct AndSpecification {
    left: Box<dyn TaskSpecification>,
    right: Box<dyn TaskSpecification>,
}

impl TaskSpecification for AndSpecification {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        self.left.is_satisfied_by(task) && self.right.is_satisfied_by(task)
    }
}

/// OR条件
///
/// 2つのSpecificationを組み合わせて、どちらか一方の条件を満たすタスクを抽出します。
///
/// # 使用シーン
/// - 「高優先度」または「期限切れ」のように、緊急性の高いタスクを広く抽出
/// - 「完了」または「キャンセル済み」のような、終了状態のタスクをまとめて取得
///
/// # 例
/// ```rust,ignore
/// // 「高優先度」または「期限切れ」のタスク
/// let spec = Box::new(TaskByPriority::new(Priority::High))
///     .or(Box::new(TaskOverdue::new()));
/// ```
pub struct OrSpecification {
    left: Box<dyn TaskSpecification>,
    right: Box<dyn TaskSpecification>,
}

impl TaskSpecification for OrSpecification {
    fn is_satisfied_by(&self, task: &TaskAggregate) -> bool {
        self.left.is_satisfied_by(task) || self.right.is_satisfied_by(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::task::value_objects::{TaskDescription, TaskTitle};
    use chrono::{Duration, Utc};

    #[test]
    fn test_task_by_status() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskByStatus::new(Status::Pending);

        // Act & Assert
        assert!(spec.is_satisfied_by(&task));

        let spec_completed = TaskByStatus::new(Status::Completed);
        assert!(!spec_completed.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_priority() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );
        let spec = TaskByPriority::new(Priority::High);

        // Act & Assert
        assert!(spec.is_satisfied_by(&task));

        let spec_low = TaskByPriority::new(Priority::Low);
        assert!(!spec_low.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_tag() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let tag_id = TagId::new(1).unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![tag_id],
            None,
        );
        let spec = TaskByTag::new(tag_id);

        // Act & Assert
        assert!(spec.is_satisfied_by(&task));

        let other_tag_id = TagId::new(2).unwrap();
        let spec_other = TaskByTag::new(other_tag_id);
        assert!(!spec_other.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_overdue() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let past_date = Utc::now().naive_utc().date() - Duration::days(1);
        let due_date = Some(super::super::value_objects::DueDate::new(past_date).unwrap());
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            due_date,
        );
        let spec = TaskOverdue::new();

        // Act & Assert
        assert!(spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_id() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskById::new(TaskId::new(0).unwrap());

        // Act & Assert
        assert!(spec.is_satisfied_by(&task));

        let spec_other = TaskById::new(TaskId::new(999).unwrap());
        assert!(!spec_other.is_satisfied_by(&task));
    }

    #[test]
    fn test_and_specification() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );

        let spec_status = Box::new(TaskByStatus::new(Status::Pending));
        let spec_priority: Box<dyn TaskSpecification> =
            Box::new(TaskByPriority::new(Priority::High));
        let and_spec = spec_status.and(spec_priority);

        // Act & Assert
        assert!(and_spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_or_specification() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![],
            None,
        );

        let spec_status = Box::new(TaskByStatus::new(Status::Completed));
        let spec_priority: Box<dyn TaskSpecification> =
            Box::new(TaskByPriority::new(Priority::High));
        let or_spec = spec_status.or(spec_priority);

        // Act & Assert
        assert!(or_spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_complex_specification() {
        // Arrange
        let title = TaskTitle::new("テスト").unwrap();
        let description = TaskDescription::new("").unwrap();
        let tag_id = TagId::new(1).unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::High,
            vec![tag_id],
            None,
        );

        // (Status == Pending AND Priority == High) OR Tag == 1
        let spec_status: Box<dyn TaskSpecification> = Box::new(TaskByStatus::new(Status::Pending));
        let spec_priority: Box<dyn TaskSpecification> =
            Box::new(TaskByPriority::new(Priority::High));
        let spec_tag: Box<dyn TaskSpecification> = Box::new(TaskByTag::new(tag_id));

        let and_spec = AndSpecification {
            left: spec_status,
            right: spec_priority,
        };
        let complex_spec = OrSpecification {
            left: Box::new(and_spec),
            right: spec_tag,
        };

        // Act & Assert
        assert!(complex_spec.is_satisfied_by(&task));
    }

    // TaskByKeyword のテストケース

    #[test]
    fn test_task_by_keyword_single_keyword_title_match() {
        // Arrange: タイトルに「買い物」を含むタスク
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
        let spec = TaskByKeyword::new(vec!["買い物".to_string()], SearchField::All);

        // Act & Assert: タイトルに含まれるのでマッチするはず
        assert!(spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_single_keyword_title_case_insensitive() {
        // Arrange: 大文字小文字を無視
        let title = TaskTitle::new("Bug Report").unwrap();
        let description = TaskDescription::new("").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskByKeyword::new(vec!["bug".to_string()], SearchField::All);

        // Act & Assert: 大文字小文字を無視してマッチするはず
        assert!(spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_single_keyword_description_match() {
        // Arrange: 説明に「レポート」を含むタスク
        let title = TaskTitle::new("週次作業").unwrap();
        let description = TaskDescription::new("週次レポートを作成する").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskByKeyword::new(vec!["レポート".to_string()], SearchField::All);

        // Act & Assert: 説明に含まれるのでマッチするはず
        assert!(spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_field_title_only() {
        // Arrange: タイトルのみ検索対象
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

        // Act & Assert: タイトルに「買い物」があるのでマッチ
        let spec_title = TaskByKeyword::new(vec!["買い物".to_string()], SearchField::Title);
        assert!(spec_title.is_satisfied_by(&task));

        // 説明の「牛乳」はタイトルにないのでマッチしない
        let spec_milk = TaskByKeyword::new(vec!["牛乳".to_string()], SearchField::Title);
        assert!(!spec_milk.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_field_description_only() {
        // Arrange: 説明のみ検索対象
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

        // Act & Assert: 説明に「牛乳」があるのでマッチ
        let spec_milk = TaskByKeyword::new(vec!["牛乳".to_string()], SearchField::Description);
        assert!(spec_milk.is_satisfied_by(&task));

        // タイトルの「買い物」は説明にないのでマッチしない
        let spec_shopping = TaskByKeyword::new(vec!["買い物".to_string()], SearchField::Description);
        assert!(!spec_shopping.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_field_all() {
        // Arrange: タイトルと説明の両方が検索対象
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

        // Act & Assert: タイトルに「買い物」があるのでマッチ
        let spec_shopping = TaskByKeyword::new(vec!["買い物".to_string()], SearchField::All);
        assert!(spec_shopping.is_satisfied_by(&task));

        // 説明に「牛乳」があるのでマッチ
        let spec_milk = TaskByKeyword::new(vec!["牛乳".to_string()], SearchField::All);
        assert!(spec_milk.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_multiple_keywords_all_match() {
        // Arrange: 複数キーワード（AND条件）
        let title = TaskTitle::new("レポート作成").unwrap();
        let description = TaskDescription::new("月次レポートを作成する").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskByKeyword::new(
            vec!["レポート".to_string(), "作成".to_string()],
            SearchField::All,
        );

        // Act & Assert: 両方のキーワードが含まれるのでマッチするはず
        assert!(spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_multiple_keywords_partial_match() {
        // Arrange: 複数キーワードのうち一部のみ含む
        let title = TaskTitle::new("レポート作成").unwrap();
        let description = TaskDescription::new("月次レポートを作成する").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskByKeyword::new(
            vec!["レポート".to_string(), "提出".to_string()],
            SearchField::All,
        );

        // Act & Assert: 「レポート」はあるが「提出」がないのでマッチしない
        assert!(!spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_empty_keywords() {
        // Arrange: 空のキーワードリスト
        let title = TaskTitle::new("タスク").unwrap();
        let description = TaskDescription::new("説明").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskByKeyword::new(vec![], SearchField::All);

        // Act & Assert: 空のキーワードリストは全タスクにマッチ
        assert!(spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_no_match() {
        // Arrange: マッチしないキーワード
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
        let spec = TaskByKeyword::new(vec!["会議".to_string()], SearchField::All);

        // Act & Assert: 「会議」は含まれないのでマッチしない
        assert!(!spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_japanese_keywords() {
        // Arrange: 日本語キーワード
        let title = TaskTitle::new("プロジェクト会議").unwrap();
        let description = TaskDescription::new("次回のプロジェクト会議の議題を確認").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskByKeyword::new(vec!["プロジェクト".to_string()], SearchField::All);

        // Act & Assert: 日本語キーワードが正しく動作する
        assert!(spec.is_satisfied_by(&task));
    }

    #[test]
    fn test_task_by_keyword_whitespace_keywords() {
        // Arrange: 空白文字のみのキーワードは無視される
        let title = TaskTitle::new("タスク").unwrap();
        let description = TaskDescription::new("説明").unwrap();
        let task = TaskAggregate::new(
            title,
            description,
            Status::Pending,
            Priority::Medium,
            vec![],
            None,
        );
        let spec = TaskByKeyword::new(vec!["  ".to_string(), "".to_string()], SearchField::All);

        // Act & Assert: 空白のみのキーワードは除外され、空のキーワードリストとして扱われる
        assert!(spec.is_satisfied_by(&task));
    }
}
