use crate::domain::task::{
    aggregate::TaskAggregate, specification::TaskSpecification, value_objects::TaskId,
};
use anyhow::Result;

/// TaskRepository trait - タスクの永続化を抽象化
///
/// DIP（依存性逆転の原則）に従い、ドメイン層にRepository traitを定義します。
/// 実装はInterface層（src/interface/persistence/）で行います。
#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    /// IDでタスクを検索
    ///
    /// # Arguments
    /// * `id` - 検索するタスクのID
    ///
    /// # Returns
    /// * `Ok(Some(TaskAggregate))` - タスクが見つかった場合
    /// * `Ok(None)` - タスクが見つからなかった場合
    /// * `Err` - エラーが発生した場合
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskAggregate>>;

    /// 全タスクを取得
    ///
    /// # Returns
    /// * `Ok(Vec<TaskAggregate>)` - 全タスクのリスト
    /// * `Err` - エラーが発生した場合
    async fn find_all(&self) -> Result<Vec<TaskAggregate>>;

    /// Specificationに基づいてタスクを検索
    ///
    /// # Arguments
    /// * `spec` - 検索条件を表すSpecification
    ///
    /// # Returns
    /// * `Ok(Vec<TaskAggregate>)` - 条件を満たすタスクのリスト
    /// * `Err` - エラーが発生した場合
    async fn find_by_specification(
        &self,
        spec: Box<dyn TaskSpecification>,
    ) -> Result<Vec<TaskAggregate>>;

    /// 新しいタスクを保存
    ///
    /// IDが0の場合、新しいIDを割り当てます。
    ///
    /// # Arguments
    /// * `task` - 保存するタスク
    ///
    /// # Returns
    /// * `Ok(TaskAggregate)` - 保存されたタスク（IDが割り当てられている）
    /// * `Err` - エラーが発生した場合
    async fn save(&self, task: TaskAggregate) -> Result<TaskAggregate>;

    /// 既存のタスクを更新
    ///
    /// # Arguments
    /// * `task` - 更新するタスク
    ///
    /// # Returns
    /// * `Ok(TaskAggregate)` - 更新されたタスク
    /// * `Err` - エラーが発生した場合
    async fn update(&self, task: TaskAggregate) -> Result<TaskAggregate>;

    /// IDでタスクを削除
    ///
    /// # Arguments
    /// * `id` - 削除するタスクのID
    ///
    /// # Returns
    /// * `Ok(true)` - タスクが削除された場合
    /// * `Ok(false)` - タスクが見つからなかった場合
    /// * `Err` - エラーが発生した場合
    async fn delete(&self, id: &TaskId) -> Result<bool>;
}
