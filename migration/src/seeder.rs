use sea_orm::TransactionTrait;
use sea_orm_migration::prelude::*;

/// シーダー実行のヘルパー関数
///
/// 環境変数RUN_SEEDERが設定されている場合のみシーダーを実行する。
/// マイグレーションの`up()`メソッド内で以下のように使用する:
///
/// ```rust
/// use crate::seeder;
///
/// async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
///     // テーブル作成など
///
///     seeder::run_if_enabled(manager, |txn| async move {
///         // シーディング処理
///         Ok(())
///     }).await?;
///
///     Ok(())
/// }
/// ```
///
/// # 型パラメータ
///
/// * `F: FnOnce(sea_orm::DatabaseTransaction) -> Fut`
///   - `F`: クロージャまたは関数の型（Functionの慣例）
///   - `FnOnce`: 一度だけ呼び出せるトレイト（所有権を消費）
///   - 引数: `DatabaseTransaction`（トランザクション）を受け取る
///   - 戻り値: `Fut`型のFutureを返す
///
/// * `Fut: std::future::Future<Output = Result<(), DbErr>>`
///   - `Fut`: 非同期処理（Future）の型（Futureの慣例）
///   - `Output`: Futureが完了したときの結果型は`Result<(), DbErr>`
///   - つまり、成功時は`()`（unit型）、失敗時は`DbErr`を返す
///
/// # ライフタイム
///
/// * `SchemaManager<'_>`
///   - `'_`: ライフタイム省略記法（Rust 2018以降）
///   - コンパイラがライフタイムを自動推論する
///   - `SchemaManager`は内部でDB接続への参照を保持しているため、
///     参照の生存期間を追跡する必要がある
///
/// # 設計パターン
///
/// この2段階の型パラメータ（F→Fut）は、**高階関数で非同期クロージャを受け取る**
/// ための標準的なパターン:
/// - `F`がクロージャ自体、`Fut`がそのクロージャが返すFutureを表す
/// - `async move { ... }`のようなクロージャを渡すと、`F`がクロージャ型、
///   `Fut`が内部のFutureになる
/// - `seeder_fn(txn).await?`で実際にFutureを実行
pub async fn run_if_enabled<F, Fut>(manager: &SchemaManager<'_>, seeder_fn: F) -> Result<(), DbErr>
where
    F: FnOnce(sea_orm::DatabaseTransaction) -> Fut,
    Fut: std::future::Future<Output = Result<(), DbErr>>,
{
    if std::env::var("RUN_SEEDER").is_ok() {
        println!("環境変数RUN_SEEDERが設定されているため、シーダーを実行します...");

        let db = manager.get_connection();
        let txn = db.begin().await?;

        seeder_fn(txn).await?;

        println!("シーダーの実行が完了しました！");
    }

    Ok(())
}
