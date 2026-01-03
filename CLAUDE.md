# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

yaruは、Rust製のタスク管理CLIアプリケーションです。TUIモード（コマンド引数なし）とCLIモード（コマンド引数あり）の2つの動作モードを持ちます。

- **TUIモード**: `cargo run`で起動し、ratatuiベースの対話的なインターフェースを提供
- **CLIモード**: `cargo run -- task add "タスク名"`のようにコマンドライン引数を指定して実行

## 開発コマンド

### ビルドとテスト

```bash
# プロジェクトのビルド
cargo build

# リリースビルド
cargo build --release

# 実行（TUIモード）
cargo run

# 実行（CLIモード）
cargo run -- task list
cargo run -- task add "タスク名"
cargo run -- tag list

# テスト実行
cargo test

# 特定のテストのみ実行
cargo test <テスト名>
```

### コード品質

justfileを使用したタスクランナーが利用可能です：

```bash
# コードフォーマット
just fmt
# または
cargo fmt

# リント実行（自動修正あり）
just lint
# または
cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings

# フォーマット + リント
just check
```

**重要**: リントは`-D warnings`オプション付きで実行され、警告をエラーとして扱います。

### データベース管理

データベースは`~/.config/yaru/yaru.db` (SQLite) に配置されます。

```bash
# マイグレーションのリセット（down -> up）とシーダー実行
just db-reset

# エンティティファイルの再生成（SeaORM）
just db-generate

# データベースリセット + エンティティ再生成
just db-refresh

# SQLite CLIで接続
just db-connect

# 全データ削除（設定ディレクトリごと削除）
just clean-all
```

マイグレーション実行時は環境変数`DATABASE_URL`と`RUN_SEEDER`が自動設定されます。

## アーキテクチャ

プロジェクトは**ドメイン駆動設計（DDD）のレイヤードアーキテクチャ**を採用しています：

```
src/
├── domain/          # ドメイン層（ビジネスロジック）
│   ├── task/        # タスク集約ルート、値オブジェクト、リポジトリインターフェース
│   ├── tag/         # タグ集約ルート、リポジトリインターフェース
│   └── services/    # ドメインサービス
├── application/     # アプリケーション層（ユースケース）
│   ├── use_cases/   # タスク・タグのユースケース実装
│   └── dto/         # データ転送オブジェクト
├── infrastructure/  # インフラストラクチャ層
│   ├── database/    # SeaORMによるDB接続管理
│   └── config/      # 設定ファイル管理
└── interface/       # インターフェース層
    ├── cli/         # CLIコマンドハンドラ（clap）
    ├── tui/         # TUIインターフェース（ratatui）
    └── persistence/ # リポジトリ実装（SeaORM, in-memory）
```

### 依存関係の方向

- **domain** ← application ← infrastructure
- **domain** ← application ← interface
- インフラストラクチャ層とインターフェース層は相互に依存しない
- ドメイン層は他のどの層にも依存しない（依存性逆転の原則）

### 重要なパターン

1. **Repository パターン**: ドメイン層でインターフェースを定義し、interface/persistence層で実装
   - `SeaOrmTaskRepository`: SQLiteを使用した永続化
   - `InMemoryTaskRepository`: テスト用のメモリ実装

2. **Aggregate Root**: Task, Tagがそれぞれ集約ルート
   - `domain/task/aggregate.rs`: Task集約ルート
   - `domain/tag/aggregate.rs`: Tag集約ルート

3. **Value Object**: ドメイン概念を値オブジェクトとして表現
   - Status, Priority, TaskName, TaskDescription, TagNameなど

4. **Use Case**: アプリケーション層でビジネスロジックを組み立て
   - `application/use_cases/task/`: タスク関連ユースケース
   - `application/use_cases/tag/`: タグ関連ユースケース

## SeaORM とマイグレーション

- ORMとして**SeaORM 1.1.2**を使用
- マイグレーションは`migration/`ディレクトリで管理
- エンティティは`entity/`ディレクトリに自動生成（`sea-orm-cli`使用）
- Workspaceとして構成: ルート, migration, entity

新しいマイグレーションの作成:
```bash
cd migration
sea-orm-cli migrate generate <マイグレーション名>
```

## TUI と CLI の統合

エントリーポイント（`src/lib.rs::run()`）で分岐:
- コマンド引数あり → `run_cli_with_command()` → CLIハンドラ実行
- コマンド引数なし → `run_tui()` → TUIモード起動

両モードともリポジトリパターンを通じて同一のドメインロジックを使用。

## テスト

テストは各モジュール内に`#[cfg(test)]`モジュールとして配置されています。以下のファイルにテストが含まれています：

- `src/interface/tui/event.rs`
- `src/interface/tui/app.rs`
- `src/interface/cli/args.rs`
- `src/interface/persistence/in_memory/task_repository.rs`
- `src/interface/persistence/sea_orm/mapper.rs`

InMemoryRepositoryを使用したユニットテストが可能です。

## 使用技術スタック

- **言語**: Rust (edition 2024)
- **非同期ランタイム**: Tokio
- **CLI**: clap (derive機能)
- **TUI**: ratatui
- **ORM**: SeaORM (SQLite)
- **対話的入力**: inquire
- **テーブル表示**: comfy-table
- **進捗表示**: indicatif
- **日付時刻**: chrono (serde対応)
- **設定**: TOML

## コーディングルール

### コードスタイル

- **インデント**: スペース4つ（Rustファイル）、`.editorconfig`に定義
- **改行**: LF（Unix-style）
- **文字コード**: UTF-8
- **末尾の空白**: 削除する
- **ファイル末尾**: 改行を追加する

### 命名規則

- **関数・変数**: `snake_case`
- **構造体・Enum・トレイト**: `PascalCase`
- **定数**: `SCREAMING_SNAKE_CASE`

### エラーハンドリング

- `anyhow::Result<T>`を戻り値の型として使用
- エラーメッセージは**日本語**で記述
- `.context("日本語のエラーメッセージ")`でコンテキストを追加
- 早期リターンには`anyhow::bail!("日本語のエラーメッセージ")`を使用

```rust
// 良い例
let config = load_config()
    .context("設定ファイルの読み込みに失敗しました")?;

if invalid_state {
    anyhow::bail!("無効な状態です: {}", details);
}
```

### ドキュメントとコメント

- **ドキュメントコメント**: `///`を使用し、**日本語**で記述
- 構造体、Enum、関数、モジュールには必ずドキュメントコメントを記載
- **インラインコメント**: `//`を使用し、**日本語**で記述
- 処理の意図が自明でない箇所にのみコメントを記載

```rust
/// タスクのステータスを表すValue Object
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// 保留中
    Pending,
    /// 進行中
    InProgress,
    /// 完了
    Completed,
}

// domain_eventsはクローン時には空にする
domain_events: Vec::new(),
```

### 値オブジェクト（Value Object）

- **Newtype パターン**を使用（例: `TaskTitle(String)`）
- コンストラクタ（`new`メソッド）でバリデーションを実施
- バリデーションエラーは日本語で`anyhow::bail!`
- 適切なトレイトを`derive`（`Debug`, `Clone`, `PartialEq`, `Eq`など）
- `value()`メソッドで内部の値にアクセス

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTitle(String);

impl TaskTitle {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            anyhow::bail!("タイトルは空にできません");
        }
        if value.len() > 100 {
            anyhow::bail!("タイトルは100文字以内にしてください");
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
```

### モジュール構成

- 値オブジェクトなどの概念は**個別ファイル**として定義
- **`mod.rs`は非推奨**: モジュールを定義する際は`mod.rs`ではなく、**モジュール名.rs**を使用する
  - 例: `src/domain/task/value_objects/mod.rs` ❌ → `src/domain/task/value_objects.rs` ✅
- モジュール名.rsで`pub use`により再エクスポート
- `use`文はクレートルートからの**絶対パス**を使用
- `use`文は層ごとにグループ化（`domain`, `application`, `infrastructure`, `interface`）

```rust
// src/domain/task/value_objects.rs (mod.rsではない)
pub mod task_title;
pub mod task_description;

pub use task_title::TaskTitle;
pub use task_description::TaskDescription;
```

### トレイトとデリベーション

- 必要なトレイトを適切に`derive`する
- 非同期トレイトには`#[async_trait]`を使用
- よく使うトレイト: `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Serialize`, `Deserialize`

```rust
use async_trait::async_trait;

#[async_trait]
impl TaskRepository for SeaOrmTaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskAggregate>> {
        // ...
    }
}
```

### テスト

- **TDD（テスト駆動開発）**を原則とする
- テストは`#[cfg(test)] mod tests`内に記述
- エッジケースを含む包括的なテストを作成
- テスト関数名は`test_<対象>_<条件>`の形式

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_title_valid() {
        let title = TaskTitle::new("有効なタイトル").unwrap();
        assert_eq!(title.value(), "有効なタイトル");
    }

    #[test]
    fn test_task_title_empty() {
        let result = TaskTitle::new("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("空"));
    }
}
```

### コード品質

- **Clippy警告を厳格に扱う**: `-D warnings`オプションでコンパイル
- 不要な参照、未使用の変数は削除
- N+1問題などのパフォーマンス問題に注意
- バリデーションロジックは一箇所にまとめる

## その他の注意事項

- **Copilot/Claude**: 日本語でのレビューと会話を基本とする（`.github/copilot-instructions.md`参照）
- **変更履歴**: Conventional Commitsに従い、`CHANGELOG.md`に記録
- **バージョン管理**: cog.tomlを使用した自動バージョニング
