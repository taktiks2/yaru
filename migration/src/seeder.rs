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
/// * `F: for<'b> FnOnce(&'b DatabaseTransaction) -> Pin<Box<dyn Future<Output = Result<(), DbErr>> + Send + 'b>>`
///   - `F`: クロージャまたは関数の型（Functionの慣例）
///   - `FnOnce`: 一度だけ呼び出せるトレイト（所有権を消費）
///   - `for<'b>`: 高階トレイト境界（HRTB）- あらゆるライフタイム'bに対して有効
///   - 引数: `&'b DatabaseTransaction`（トランザクションへの参照）
///   - 戻り値: `Pin<Box<dyn Future>>`（ヒープ上にピン留めされたFuture）
///   - `+ Send`: Futureがスレッド間で安全に送信可能
///   - `+ 'b`: Futureはトランザクションの参照のライフタイムに束縛される
///
/// # ライフタイム
///
/// * `SchemaManager<'_>`
///   - `'_`: ライフタイム省略記法（Rust 2018以降）
///   - コンパイラがライフタイムを自動推論する
///
/// * `for<'b>` (HRTB - Higher-Rank Trait Bounds)
///   - トランザクションの参照とFutureのライフタイムを適切に関連付ける
///   - クロージャが任意のライフタイムの参照を受け取れるようにする
///   - `'b` はクロージャ呼び出し時に具体化される
///
/// * `Pin<Box<dyn Future + 'b>>`
///   - `Box`: Futureをヒープに配置（サイズが不定のため）
///   - `Pin`: Futureをメモリ上で移動不可にする（async/awaitの要件）
///   - `dyn Future`: トレイトオブジェクト（動的ディスパッチ）
///   - `+ 'b`: Futureが参照するトランザクションのライフタイム制約
///
/// # なぜ Box::pin が必要なのか？
///
/// この関数シグネチャで `Box::pin` が必要になる理由は、**3つの制約の組み合わせ**です：
///
/// ## 1. HRTB（Higher-Rank Trait Bounds）の制約
///
/// `for<'b>` は「あらゆるライフタイム `'b` に対して有効」という意味です。
/// つまり、呼び出し時に決まる任意のライフタイムに対応する必要があります。
///
/// ## 2. ライフタイムの依存関係
///
/// `async { seed_data(txn).await }` が返すFutureは `txn` の参照を保持します。
/// したがって、Futureのライフタイムは `txn` のライフタイムに束縛されます。
///
/// ```rust
/// // ❌ 理想だが表現できない
/// F: for<'b> FnOnce(&'b DatabaseTransaction) -> Fut<'b>
/// //                                               ^^^^
/// //                                               ジェネリックな型パラメータでは
/// //                                               'bへの依存を表現できない
/// ```
///
/// ## 3. トレイトオブジェクトで解決
///
/// ```rust
/// // ✅ dyn Future + 'b でライフタイム依存を直接記述
/// F: for<'b> FnOnce(&'b DatabaseTransaction)
///     -> Pin<Box<dyn Future<Output = Result<(), DbErr>> + Send + 'b>>
/// //             ^^^                                                ^^
/// //             トレイトオブジェクト                          ライフタイム依存
/// ```
///
/// - **`dyn Future + 'b`**: トレイトオブジェクトにライフタイム制約を直接記述
/// - **`Box`**: トレイトオブジェクトはサイズ不定なのでヒープ配置が必須
/// - **`Pin`**: Futureは自己参照構造を持つため、メモリ上で移動不可にする必要がある
///
/// ## 失敗例との比較
///
/// ```rust
/// // ❌ 失敗例1: Futのライフタイム依存が表現できない
/// F: for<'b> FnOnce(&'b DatabaseTransaction) -> Fut,
/// Fut: Future<Output = Result<(), DbErr>>,
///
/// // ❌ 失敗例2: 'aと'bの関係が不明確
/// F: for<'b> FnOnce(&'b DatabaseTransaction) -> Fut,
/// Fut: Future<Output = Result<(), DbErr>> + 'a,  // 'aは何を指す？
///
/// // ✅ 成功例: トレイトオブジェクトで'bを直接埋め込む
/// F: for<'b> FnOnce(&'b DatabaseTransaction) -> Pin<Box<dyn Future + 'b>>
/// ```
///
/// つまり、`Box::pin` は**HRTBと参照を渡すFutureのライフタイム依存を両立**させる
/// ための解決策です。ジェネリックな型パラメータではライフタイムの依存関係を
/// 表現できないため、`dyn Future + 'b` というトレイトオブジェクトを使って
/// 明示的に制約を記述する必要があります。
///
/// # 設計パターン
///
/// この設計は、**参照を受け取る非同期クロージャ**を扱うための標準的なパターン:
/// - トランザクションは参照として渡される（所有権移動ではない）
/// - `Box::pin(async { ... })` で Future をヒープ上にピン留めする
/// - HRTBにより、ライフタイムが自動的に推論される
///
/// # 使用例
///
/// ```rust
/// seeder::run_if_enabled(manager, |txn| {
///     Box::pin(async {
///         // シーディング処理
///         seed_data(txn).await
///     })
/// }).await?;
/// ```
pub async fn run_if_enabled<F>(manager: &SchemaManager<'_>, seeder_fn: F) -> Result<(), DbErr>
where
    F: for<'b> FnOnce(
        &'b sea_orm::DatabaseTransaction,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), DbErr>> + Send + 'b>,
    >,
{
    if std::env::var("RUN_SEEDER").is_ok() {
        println!("環境変数RUN_SEEDERが設定されているため、シーダーを実行します...");

        let db = manager.get_connection();
        let txn = db.begin().await?;

        seeder_fn(&txn).await?;

        txn.commit().await?;

        println!("シーダーの実行が完了しました！");
    }

    Ok(())
}
