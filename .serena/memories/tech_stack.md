# 技術スタック

## 言語・エディション
- **Rust**: edition 2024
- バージョン: 0.9.0

## 主要な依存関係

### CLIとTUI
- **clap** 4.5.53: CLIパーサー（deriveフィーチャー使用）
- **ratatui** 0.30.0: TUI（Terminal User Interface）フレームワーク
- **inquire** 0.9.1: 対話的入力（editor, date機能）

### データベースとORM
- **SeaORM** 1.1.2: ORM（SQLite対応）
  - フィーチャー: sqlx-sqlite, runtime-tokio-rustls, macros, with-chrono
- **migration**: マイグレーション管理（ワークスペースメンバー）
- **entity**: エンティティ自動生成（ワークスペースメンバー）

### 非同期ランタイム
- **tokio** 1.42: 非同期ランタイム（rt, macros機能）
- **async-trait** 0.1: 非同期トレイト

### 表示とUI
- **comfy-table** 7.2.1: テーブル表示
- **indicatif** 0.18.3: プログレスバー
- **console** 0.16.2: コンソール操作

### ユーティリティ
- **anyhow** 1.0: エラーハンドリング
- **chrono** 0.4.42: 日付時刻処理（serde対応）
- **serde** 1.0.219: シリアライゼーション（derive機能）
- **serde_json** 1.0.148: JSON処理
- **toml** 0.9: TOML設定ファイル処理

### 開発・テスト
- **tempfile** 3.24: テスト用一時ファイル（dev-dependency）
- **sea-orm-cli**: エンティティ生成ツール

## Workspaceの構成
- ルート: メインアプリケーション
- migration: データベースマイグレーション
- entity: SeaORMエンティティ（自動生成）
