# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`yaru` は日本語対応のシンプルなCLI Todoアプリケーションです。Rustで実装されており、JSONファイルにデータを永続化します。

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
cargo run -- list
cargo run -- add --title "タスク" --status pending
cargo run -- delete --id 1
```

## Architecture

### モジュール構成

- **lib.rs**: アプリケーションのエントリーポイント。`run()` 関数でCLI引数をパース、設定を読み込み、コマンドを実行
- **main.rs**: バイナリのエントリーポイント。`run()` を呼び出しエラーハンドリングのみ行う

### コアモジュール

#### CLI層 (`cli.rs`)
- `Args`: CLIの引数をパース
- `Commands`: サブコマンド（List, Add, Delete）を定義
- `Filter`: フィルタ機能（例: `status:done`）のパース

#### コマンド層 (`commands/`)
各サブコマンドの実装。リポジトリを受け取り、ビジネスロジックを実行:
- `add.rs`: Todoの追加（対話モード対応）
- `list.rs`: Todoの一覧表示（フィルタ機能付き）
- `delete.rs`: Todoの削除（確認ダイアログ付き）

#### ドメイン層 (`todo.rs`)
- `Todo`: Todoタスクの構造体（id, title, status, created_at, updated_at）
- `Status`: タスクのステータス（Pending, Completed, InProgress）
  - `from_filter_value()`: フィルタ文字列からStatusへの変換

#### データアクセス層 (`repository/`)
リポジトリパターンを採用:
- `TodoRepository` トレイト: データ永続化の抽象インターフェース
- `JsonTodoRepository`: JSON形式の実装
  - `load_todos()`: JSONファイルからTodoリストを読み込み
  - `save_todos()`: TodoリストをJSONファイルに保存
  - `find_next_id()`: 次のIDを生成
  - `ensure_data_exists()`: データファイルの初期化

#### ユーティリティ層
- `json.rs`: JSONファイル操作の汎用関数
  - `load_json<T>()`: HRTB（Higher-Rank Trait Bounds）を使用した柔軟な読み込み
  - `save_json<T>()`: `?Sized` トレイト境界でサイズ不定型に対応
- `config.rs`: TOML形式の設定ファイル管理
  - デフォルトパス: `~/.config/yaru/config.toml`
  - データファイル: `~/.config/yaru/todo.json`
- `display/`: テーブル表示（comfy-tableを使用）

### 設計パターン

#### リポジトリパターン
データアクセスをトレイトで抽象化し、将来的にSQLiteなど別の実装に切り替え可能:
```rust
pub trait TodoRepository {
    fn load_todos(&self) -> Result<Vec<Todo>>;
    fn save_todos(&self, todos: &[Todo]) -> Result<()>;
}
```

#### コマンド関数の統一インターフェース
全てのコマンド関数はリポジトリを引数として受け取る:
```rust
pub fn add_todo(repo: &impl TodoRepository, ...) -> Result<()>
pub fn list_todos(repo: &impl TodoRepository, ...) -> Result<()>
pub fn delete_todo(repo: &impl TodoRepository, ...) -> Result<()>
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

### 新しいコマンドの追加手順

1. `cli.rs` の `Commands` enum に新しいバリアントを追加
2. `commands/` に新しいモジュールファイルを作成
3. `commands.rs` でエクスポート
4. `lib.rs` の `handle_command()` で新しいコマンドを処理

### JSON形式の特殊なトレイト境界

`json.rs` のコメントに詳細な説明あり:
- `load_json`: HRTB（`for<'de> Deserialize<'de>`）を使用
- `save_json`: `?Sized` を使用してサイズ不定型に対応
