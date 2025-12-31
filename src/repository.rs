use anyhow::Result;

/// ジェネリックなリポジトリトレイト
/// データの永続化方法を抽象化し、異なる実装を切り替え可能にする
pub trait Repository<T> {
    /// IDでエンティティを検索
    async fn find_by_id(&self, id: u64) -> Result<Option<T>>;

    /// 全エンティティを取得
    async fn find_all(&self) -> Result<Vec<T>>;

    /// 条件でエンティティを検索
    async fn search<F>(&self, predicate: F) -> Result<Vec<T>>
    where
        F: Fn(&T) -> bool;

    /// 新しいエンティティを作成（AUTO INCREMENT）
    async fn create(&self, item: &T) -> Result<T>;

    /// IDでエンティティを削除
    async fn delete(&self, id: u64) -> Result<bool>;
}

// サブモジュールをエクスポート
pub mod task;
pub mod tag;
