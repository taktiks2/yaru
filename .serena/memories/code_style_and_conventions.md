# コードスタイルと規約

## モジュール構成

### 重要: `mod.rs` は使用しない
Rustの現代的なモジュール構成では、`mod.rs` は非推奨です。

**推奨パターン:**
```
src/
├── lib.rs
├── command.rs          # commandモジュールの定義とエクスポート
├── command/            # サブモジュールの実装
│   ├── task/
│   │   ├── add.rs
│   │   ├── list.rs
│   │   └── delete.rs
│   └── tag/
│       ├── add.rs
│       ├── list.rs
│       └── delete.rs
```

**モジュール宣言の例:**
```rust
// command.rs
pub mod task;
pub mod tag;

pub use task::add::add_task;
pub use task::list::list_tasks;
```

## コミットメッセージ

### Conventional Commits
cocogittoを使用してConventional Commitsを強制しています。

**利用可能なprefix:**
- `feat:` - 新機能の追加
- `fix:` - バグ修正
- `docs:` - ドキュメントの変更
- `style:` - コードの意味に影響しない変更（フォーマットなど）
- `refactor:` - リファクタリング
- `perf:` - パフォーマンス改善
- `test:` - テストの追加・修正
- `chore:` - ビルドプロセスやツールの変更
- `ci:` - CI設定の変更
- `build:` - ビルドシステムの変更

**例:**
```bash
git commit -m "feat: タスクの優先度機能を追加"
git commit -m "fix: リスト表示時のソート順を修正"
```

## エラーハンドリング

### anyhow::Result の使用
- 統一的なエラーハンドリングに `anyhow::Result` を使用
- `.context()` でエラーメッセージに文脈を追加
- ユーザーフレンドリーな日本語エラーメッセージ
- エラーを返す際は `anyhow::bail!` を使用（`anyhow::anyhow!` + `return Err()` の代わり）

**例:**
```rust
use anyhow::{Result, Context};

pub async fn some_function() -> Result<()> {
    let file = std::fs::read_to_string(path)
        .context("ファイルの読み込みに失敗しました")?;
    
    if condition {
        anyhow::bail!("条件が満たされていません");
    }
    
    Ok(())
}
```

## デザインパターン

### リポジトリパターン
データアクセスをトレイトで抽象化：
```rust
pub trait Repository<T> {
    async fn find_by_id(&self, id: i32) -> Result<Option<T>>;
    async fn find_all(&self) -> Result<Vec<T>>;
    async fn search<F>(&self, predicate: F) -> Result<Vec<T>>
    where F: Fn(&T) -> bool;
    async fn create(&self, item: &T) -> Result<T>;
    async fn delete(&self, id: i32) -> Result<bool>;
}
```

### コマンド関数の統一インターフェース
全てのコマンド関数は非同期でデータベース接続を引数として受け取る：
```rust
pub async fn add_task(db: &DatabaseConnection, ...) -> Result<()>
pub async fn list_tasks(db: &DatabaseConnection, ...) -> Result<()>
```

## テスト戦略
- ユニットテスト: 各モジュールの `#[cfg(test)]` モジュール内に配置
- `tempfile` クレートで一時ディレクトリを使用してファイル操作をテスト
- リポジトリパターンによりモック実装が容易

## コードフォーマット
- `cargo fmt` を使用
- 標準のRustフォーマッティング規則に従う
