# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`yaru` は日本語対応のシンプルなCLIタスク管理アプリケーションです。Rustで実装されており、JSONファイルにデータを永続化します。

## Common Commands

### ビルドとテスト
```bash
# ビルド
cargo build

# リリースビルド
cargo build --release

# 全てのテストを実行
cargo test

# 特定のモジュールのテストを実行
cargo test <module_name>  # 例: cargo test repository

# 特定のテスト関数を実行
cargo test <test_name>    # 例: cargo test test_save_json

# テスト実行時に標準出力を表示
cargo test -- --nocapture

# コードフォーマット
cargo fmt

# コードチェック
cargo clippy
```

### アプリケーション実行
```bash
# インストール
cargo install --path .

# 実行（開発中）
cargo run -- <subcommand>

# 例:
cargo run -- task list
cargo run -- task add --title "タスク" --status pending
cargo run -- task delete --id 1
cargo run -- tag list
cargo run -- tag add --name "重要"
cargo run -- tag delete --id 1
```

## Architecture

### モジュール構成

- **lib.rs**: アプリケーションのエントリーポイント。`run()` 関数でCLI引数をパース、設定を読み込み、コマンドを実行
- **main.rs**: バイナリのエントリーポイント。`run()` を呼び出しエラーハンドリングのみ行う

### コアモジュール

#### CLI層 (`cli.rs`)
- `Args`: CLIの引数をパース
- `Commands`: トップレベルコマンド（Task, Tag）を定義
- `TaskCommands`: タスク管理サブコマンド（List, Add, Delete）を定義
- `TagCommands`: タグ管理サブコマンド（List, Add, Delete）を定義
- `Filter`: フィルタ機能（例: `status:done`）のパース

#### コマンド層 (`commands/`)
各サブコマンドの実装。リポジトリを受け取り、ビジネスロジックを実行:

タスク管理:
- `add.rs`: タスクの追加（対話モード対応、タグ存在確認）
- `list.rs`: タスクの一覧表示（フィルタ機能付き）
- `delete.rs`: タスクの削除（確認ダイアログ付き）

タグ管理:
- `tag_add.rs`: タグの追加（対話モード対応）
- `tag_list.rs`: タグの一覧表示
- `tag_delete.rs`: タグの削除（参照整合性チェック付き）

#### ドメイン層
`task.rs`:
- `Task`: タスクの構造体（id, title, description, status, priority, tags, created_at, updated_at）
- `Status`: タスクのステータス（Pending, Completed, InProgress）
  - `from_filter_value()`: フィルタ文字列からStatusへの変換
- `Priority`: タスクの優先度（Low, Medium, High）

`tag.rs`:
- `Tag`: タグの構造体（id, name, description, created_at, updated_at）

#### データアクセス層 (`repository/`)
リポジトリパターンを採用:
- `Repository<T>` トレイト: データ永続化の抽象インターフェース（ジェネリック）
- `JsonRepository<T>`: JSON形式の汎用実装
  - `load()`: JSONファイルからデータを読み込み
  - `save()`: データをJSONファイルに保存
  - `find_next_id()`: 次のIDを生成
  - `ensure_data_exists()`: データファイルの初期化
  - `Task` と `Tag` の両方に使用可能

#### ユーティリティ層
- `json.rs`: JSONファイル操作の汎用関数
  - `load_json<T>()`: HRTB（Higher-Rank Trait Bounds）を使用した柔軟な読み込み
  - `save_json<T>()`: `?Sized` トレイト境界でサイズ不定型に対応
- `config.rs`: TOML形式の設定ファイル管理
  - デフォルトパス: `~/.config/yaru/config.toml`
  - データファイル: `~/.config/yaru/tasks.json`
- `display/`: テーブル表示（comfy-tableを使用）

### 設計パターン

#### リポジトリパターン
データアクセスをトレイトで抽象化し、将来的にSQLiteなど別の実装に切り替え可能:
```rust
pub trait Repository<T> {
    fn load(&self) -> Result<Vec<T>>;
    fn save(&self, items: &[T]) -> Result<()>;
    fn find_next_id(&self) -> Result<i32>;
    fn ensure_data_exists(&self) -> Result<()>;
}
```

#### コマンド関数の統一インターフェース
全てのコマンド関数はリポジトリを引数として受け取る:

タスク管理:
```rust
pub fn add_task(task_repo: &impl Repository<Task>, tag_repo: &impl Repository<Tag>, ...) -> Result<()>
pub fn list_tasks(repo: &impl Repository<Task>, ...) -> Result<()>
pub fn delete_task(repo: &impl Repository<Task>, ...) -> Result<()>
```

タグ管理:
```rust
pub fn add_tag(repo: &impl Repository<Tag>, ...) -> Result<()>
pub fn list_tags(repo: &impl Repository<Tag>) -> Result<()>
pub fn delete_tag(tag_repo: &impl Repository<Tag>, task_repo: &impl Repository<Task>, ...) -> Result<()>
```

### エラーハンドリング

- `anyhow::Result` を使用した統一的なエラーハンドリング
- `.context()` でエラーメッセージに文脈を追加
- ユーザーフレンドリーな日本語エラーメッセージ

### テスト戦略

- ユニットテスト: 各モジュールの `#[cfg(test)]` モジュール内に配置
- `tempfile` クレートで一時ディレクトリを使用してファイル操作をテスト
- リポジトリパターンによりモック実装が容易

## Development Notes

### モジュール構成のベストプラクティス

**重要: `mod.rs` は使用しない**

Rustの現代的なモジュール構成では、`mod.rs` は非推奨です。以下のパターンを使用してください：

#### 推奨パターン
```
src/
├── lib.rs
├── commands.rs          # commandsモジュールの定義とエクスポート
├── commands/            # サブモジュールの実装
│   ├── add.rs
│   ├── list.rs
│   └── delete.rs
├── repository.rs        # repositoryモジュールの定義とエクスポート
└── repository/          # サブモジュールの実装
    ├── json.rs
    └── sqlite.rs
```

#### 非推奨パターン（使用しないこと）
```
src/
├── lib.rs
├── commands/
│   ├── mod.rs          # ❌ 使用しない
│   ├── add.rs
│   └── list.rs
```

#### モジュール宣言の例
`commands.rs`:
```rust
// サブモジュールの宣言
pub mod add;
pub mod list;
pub mod delete;

// 必要に応じて公開
pub use add::add_task;
pub use list::list_tasks;
pub use delete::delete_task;
```

### 新しいコマンドの追加手順

#### タスクサブコマンドの追加
1. `cli.rs` の `TaskCommands` enum に新しいバリアントを追加
2. `commands/` に新しいモジュールファイルを作成
3. `commands.rs` でエクスポート
4. `lib.rs` の `handle_task_command()` で新しいコマンドを処理

#### タグサブコマンドの追加
1. `cli.rs` の `TagCommands` enum に新しいバリアントを追加
2. `commands/` に新しいモジュールファイルを作成（例: `tag_xxx.rs`）
3. `commands.rs` でエクスポート
4. `lib.rs` の `handle_tag_command()` で新しいコマンドを処理

#### 新しいトップレベルコマンドの追加
1. `cli.rs` の `Commands` enum に新しいバリアントを追加
2. 対応する `XxxCommands` enum を作成
3. `commands/` に関連するモジュールを作成
4. `lib.rs` に `handle_xxx_command()` 関数を追加
5. `lib.rs` の `handle_command()` で新しいコマンドを処理

### JSON形式の特殊なトレイト境界

`json.rs` のコメントに詳細な説明あり:
- `load_json`: HRTB（`for<'de> Deserialize<'de>`）を使用
- `save_json`: `?Sized` を使用してサイズ不定型に対応
