# yaru

シンプルで使いやすい日本語対応のCLI Todoアプリケーション

## 特徴

- **シンプルなインターフェース**: 直感的なコマンドでタスク管理
- **日本語対応**: 完全日本語対応のUI
- **軽量**: Rustで実装され、高速に動作
- **JSONベース**: 人間が読めるJSON形式でデータを保存
- **フィルタ機能**: ステータスでタスクをフィルタリング
- **対話モード**: タイトルや状態を対話的に入力可能

## インストール

### 前提条件

- Rust 1.70以降

### ビルドとインストール

```bash
# リポジトリをクローン
git clone <repository-url>
cd yaru

# ビルドとインストール
cargo install --path .
```

## 使い方

### タスクの追加

```bash
# コマンドラインオプションで追加
yaru add --title "買い物に行く" --status pending

# 対話モードで追加（オプション省略時）
yaru add
```

利用可能なステータス:
- `pending`: 保留中（デフォルト）
- `in-progress`: 進行中
- `completed`: 完了

### タスクの一覧表示

```bash
# 全てのタスクを表示
yaru list

# ステータスでフィルタリング
yaru list --filter status:pending
yaru list --filter status:completed
yaru list --filter status:in-progress

# エイリアスも使用可能
yaru list --filter status:todo      # pending と同じ
yaru list --filter status:done      # completed と同じ
yaru list --filter status:progress  # in-progress と同じ
```

### タスクの削除

```bash
# IDを指定して削除
yaru delete --id 1

# 確認ダイアログが表示されます
```

### ヘルプ

```bash
# 全体のヘルプ
yaru --help

# サブコマンドのヘルプ
yaru list --help
yaru add --help
yaru delete --help
```

## 設定

### 設定ファイル

設定ファイルは `~/.config/yaru/config.toml` に配置されます。

```toml
[storage]
todo_file = "/path/to/your/todo.json"
```

設定ファイルが存在しない場合、デフォルトで `~/.config/yaru/todo.json` が使用されます。

### データファイル

Todoデータは以下の形式でJSON形式で保存されます:

```json
[
  {
    "id": 1,
    "title": "買い物に行く",
    "status": "Pending",
    "created_at": "2025-12-27T10:00:00+00:00",
    "updated_at": "2025-12-27T10:00:00+00:00"
  }
]
```

## 開発

### 前提条件

このプロジェクトではConventional Commitsを採用しています。
コミットメッセージの検証にcocogittoを使用します。

```bash
# cocogittoのインストール
cargo install --locked cocogitto

# Git Hooksのインストール（コミット時の自動検証）
# プロジェクトルートで実行
cog install-hook commit-msg
```

### コミットメッセージのルール

以下のprefixを使用してください：

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

例:
```bash
git commit -m "feat: タスクの優先度機能を追加"
git commit -m "fix: リスト表示時のソート順を修正"
git commit -m "docs: READMEにインストール手順を追加"
```

### テストの実行

```bash
# 全てのテストを実行
cargo test

# 特定のモジュールのテストを実行
cargo test repository

# 標準出力を表示してテスト
cargo test -- --nocapture
```

### コードフォーマット

```bash
cargo fmt
```

### Lintチェック

```bash
cargo clippy
```

## アーキテクチャ

yaruはクリーンなアーキテクチャを採用しています:

- **リポジトリパターン**: データアクセスを抽象化し、将来的な拡張性を確保
- **コマンドパターン**: 各サブコマンドを独立したモジュールとして実装
- **トレイトベース**: テスタビリティとモジュール性を重視

## ライセンス

このプロジェクトのライセンスについては、リポジトリのライセンスファイルを参照してください。
