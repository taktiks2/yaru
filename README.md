# yaru

Rust製のタスク管理CLIアプリケーション

## 概要

yaruは、コマンドラインで使える軽量で高速なタスク管理ツールです。TUI（Terminal User Interface）モードとCLIモードの2つの動作モードを提供し、用途に応じて使い分けることができます。

## 特徴

- **2つの動作モード**
  - **TUIモード**: 対話的なターミナルインターフェース（引数なしで起動）
  - **CLIモード**: コマンドライン引数による操作（スクリプトやCI/CDに最適）
- **軽量・高速**: Rust製で高パフォーマンス
- **タグ管理**: タスクにタグを付けて分類・検索
- **優先度・ステータス管理**: タスクの優先度と進捗状態を管理
- **ローカルストレージ**: SQLiteによるデータ永続化

## インストール

### ビルド要件

- Rust 1.80以上（edition 2024対応）

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/yourusername/yaru.git
cd yaru

# ビルド
cargo build --release

# バイナリは target/release/yaru に生成されます
```

## 使い方

### TUIモード

引数なしで起動すると、対話的なTUIインターフェースが立ち上がります。

```bash
cargo run
# または
./target/release/yaru
```

### CLIモード

コマンドライン引数を指定して実行します。

#### タスク操作

```bash
# タスク一覧を表示
cargo run -- task list

# タスクを追加
cargo run -- task add "新しいタスク"

# タスクを完了
cargo run -- task complete <タスクID>

# タスクを削除
cargo run -- task delete <タスクID>
```

#### タグ操作

```bash
# タグ一覧を表示
cargo run -- tag list

# タグを追加
cargo run -- tag add "タグ名"

# タグを削除
cargo run -- tag delete <タグID>
```

## データベース

タスクデータは以下の場所に保存されます：

- `~/.config/yaru/yaru.db` (SQLite)

## 開発

### セットアップ

```bash
# 依存関係のインストール
cargo build

# テスト実行
cargo test

# フォーマット
cargo fmt

# リント
cargo clippy --all-targets --all-features -- -D warnings
```

### justを使用したタスクランナー

```bash
# コードフォーマット
just fmt

# リント（自動修正）
just lint

# フォーマット + リント
just check

# データベースのリセット
just db-reset

# エンティティの再生成
just db-generate
```

### アーキテクチャ

yaruはドメイン駆動設計（DDD）のレイヤードアーキテクチャを採用しています。

```
src/
├── domain/          # ドメイン層（ビジネスロジック）
├── application/     # アプリケーション層（ユースケース）
├── infrastructure/  # インフラストラクチャ層
└── interface/       # インターフェース層（CLI/TUI/永続化）
```

詳細な開発ガイドラインは [CLAUDE.md](./CLAUDE.md) を参照してください。

## 技術スタック

- **言語**: Rust (edition 2024)
- **CLI**: clap
- **TUI**: ratatui
- **ORM**: SeaORM (SQLite)
- **非同期**: Tokio

## ライセンス

未定

## 貢献

イシューやプルリクエストを歓迎します。開発に参加する際は [CLAUDE.md](./CLAUDE.md) のコーディングルールを確認してください。
