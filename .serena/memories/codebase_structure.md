# コードベース構造

## ディレクトリ構成

```
yaru/
├── src/
│   ├── main.rs                    # バイナリのエントリーポイント
│   ├── lib.rs                     # ライブラリのエントリーポイント、run()関数
│   ├── cli.rs                     # CLI引数定義（Args, Commands, TaskCommands, TagCommands）
│   ├── config.rs                  # TOML設定ファイル管理
│   ├── json.rs                    # JSON操作の汎用関数（load_json, save_json）
│   ├── command.rs                # コマンドモジュールのエクスポート
│   ├── command/
│   │   ├── task/                  # タスク管理コマンド
│   │   │   ├── add.rs             # タスク追加（対話モード対応）
│   │   │   ├── list.rs            # タスク一覧表示（フィルタ機能）
│   │   │   ├── delete.rs          # タスク削除（確認ダイアログ）
│   │   │   └── show.rs            # タスク詳細表示
│   │   └── tag/                   # タグ管理コマンド
│   │       ├── add.rs             # タグ追加（対話モード対応）
│   │       ├── list.rs            # タグ一覧表示
│   │       ├── delete.rs          # タグ削除（参照整合性チェック）
│   │       └── show.rs            # タグ詳細表示
│   ├── domain.rs                  # ドメインモジュールのエクスポート
│   ├── domain/
│   │   ├── task.rs                # Task構造体、Status、Priority
│   │   └── tag.rs                 # Tag構造体
│   ├── repository.rs              # リポジトリモジュールのエクスポート
│   ├── repository/
│   │   ├── task.rs                # タスクリポジトリ実装
│   │   └── tag.rs                 # タグリポジトリ実装
│   ├── entity.rs                  # SeaORMエンティティのエクスポート
│   ├── entity/                    # SeaORM生成エンティティ
│   ├── display.rs                 # テーブル表示モジュールのエクスポート
│   └── display/                   # テーブル表示実装
├── migration/                     # データベースマイグレーション
├── target/                        # ビルド成果物
├── .github/                       # GitHub Actions設定
├── .git/                          # Gitリポジトリ
├── .serena/                       # Serena設定
├── .claude/                       # Claude設定
├── Cargo.toml                     # プロジェクト依存関係定義
├── Cargo.lock                     # 依存関係ロックファイル
├── justfile                       # タスクランナー設定
├── cog.toml                       # cocogitto設定
├── .editorconfig                  # エディタ設定
├── .gitignore                     # Git除外設定
├── README.md                      # プロジェクト説明
├── CLAUDE.md                      # Claude向けガイド
└── CHANGELOG.md                   # 変更履歴
```

## 層別アーキテクチャ

### CLI層 (cli.rs)
- `Args`: CLIの引数をパース
- `Commands`: トップレベルコマンド（Task, Tag）
- `TaskCommands`: タスク管理サブコマンド
- `TagCommands`: タグ管理サブコマンド
- `Filter`: フィルタ機能のパース

### コマンド層 (command/)
各サブコマンドの実装。データベース接続を受け取り、ビジネスロジックを実行。

**タスク管理:**
- `add.rs`: タスクの追加
- `list.rs`: タスクの一覧表示
- `delete.rs`: タスクの削除
- `show.rs`: タスクの詳細表示

**タグ管理:**
- `add.rs`: タグの追加
- `list.rs`: タグの一覧表示
- `delete.rs`: タグの削除
- `show.rs`: タグの詳細表示

### ドメイン層 (domain/)
**task.rs:**
- `Task`: タスクの構造体
  - フィールド: id, title, description, status, priority, tags, created_at, updated_at
- `Status`: Pending, Completed, InProgress
- `Priority`: Low, Medium, High

**tag.rs:**
- `Tag`: タグの構造体
  - フィールド: id, name, description, created_at, updated_at

### データアクセス層 (repository/)
リポジトリパターンを採用:
- `Repository<T>` トレイト: データ永続化の抽象インターフェース
- SeaORMを使用したデータベースアクセス
- TaskとTagのリポジトリ実装

### エンティティ層 (entity/)
SeaORM CLIで自動生成されるデータベースエンティティ

### ユーティリティ層
- `json.rs`: JSONファイル操作の汎用関数
- `config.rs`: TOML設定ファイル管理
- `display.rs`: テーブル表示（comfy-table使用）

## 設計パターン

### リポジトリパターン
データアクセスをトレイトで抽象化し、異なる実装を切り替え可能

### コマンドパターン
各サブコマンドを独立したモジュールとして実装

### トレイトベース設計
テスタビリティとモジュール性を重視
