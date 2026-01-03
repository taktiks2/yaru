use anyhow::Result;

use super::aggregate::TagAggregate;
use super::value_objects::TagId;

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
}
