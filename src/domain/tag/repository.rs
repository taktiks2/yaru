use crate::domain::tag::{aggregate::TagAggregate, value_objects::TagId};
use anyhow::Result;

/// TagRepository trait - タグの永続化を抽象化
///
/// DIP（依存性逆転の原則）に従い、ドメイン層にRepository traitを定義します。
/// 実装はInterface層（src/interface/persistence/）で行います。
#[async_trait::async_trait]
pub trait TagRepository: Send + Sync {
    /// IDでタグを検索
    ///
    /// # Arguments
    /// * `id` - 検索するタグのID
    ///
    /// # Returns
    /// * `Ok(Some(TagAggregate))` - タグが見つかった場合
    /// * `Ok(None)` - タグが見つからなかった場合
    /// * `Err` - エラーが発生した場合
    async fn find_by_id(&self, id: &TagId) -> Result<Option<TagAggregate>>;

    /// 全タグを取得
    ///
    /// # Returns
    /// * `Ok(Vec<TagAggregate>)` - 全タグのリスト
    /// * `Err` - エラーが発生した場合
    async fn find_all(&self) -> Result<Vec<TagAggregate>>;

    /// 新しいタグを保存
    ///
    /// IDが0の場合、新しいIDを割り当てます。
    ///
    /// # Arguments
    /// * `tag` - 保存するタグ
    ///
    /// # Returns
    /// * `Ok(TagAggregate)` - 保存されたタグ（IDが割り当てられている）
    /// * `Err` - エラーが発生した場合
    async fn save(&self, tag: TagAggregate) -> Result<TagAggregate>;

    /// 既存のタグを更新
    ///
    /// # Arguments
    /// * `tag` - 更新するタグ
    ///
    /// # Returns
    /// * `Ok(TagAggregate)` - 更新されたタグ
    /// * `Err` - エラーが発生した場合
    async fn update(&self, tag: TagAggregate) -> Result<TagAggregate>;

    /// IDでタグを削除
    ///
    /// # Arguments
    /// * `id` - 削除するタグのID
    ///
    /// # Returns
    /// * `Ok(true)` - タグが削除された場合
    /// * `Ok(false)` - タグが見つからなかった場合
    /// * `Err` - エラーが発生した場合
    async fn delete(&self, id: &TagId) -> Result<bool>;

    /// 名前でタグを検索
    ///
    /// # Arguments
    /// * `name` - 検索するタグの名前
    ///
    /// # Returns
    /// * `Ok(Some(TagAggregate))` - タグが見つかった場合
    /// * `Ok(None)` - タグが見つからなかった場合
    /// * `Err` - エラーが発生した場合
    #[allow(dead_code)]
    async fn find_by_name(&self, name: &str) -> Result<Option<TagAggregate>>;

    /// 複数のIDでタグを一括検索
    ///
    /// # Arguments
    /// * `ids` - 検索するタグのIDのスライス
    ///
    /// # Returns
    /// * `Ok(Vec<TagAggregate>)` - 見つかったタグのリスト（IDの順序は保証されない）
    /// * `Err` - エラーが発生した場合
    ///
    /// # Note
    /// 存在しないIDは結果に含まれません。
    /// 呼び出し側で存在確認が必要な場合は、戻り値の長さを引数の長さと比較してください。
    async fn find_by_ids(&self, ids: &[TagId]) -> Result<Vec<TagAggregate>>;
}
