# コードベースの構造

## ディレクトリ構成

```
yaru/
├── src/                    # ソースコード（DDDレイヤードアーキテクチャ）
│   ├── domain/             # ドメイン層（ビジネスロジック）
│   │   ├── task/           # タスク集約
│   │   │   ├── aggregate.rs       # TaskAggregate（集約ルート）
│   │   │   ├── repository.rs      # TaskRepositoryトレイト定義
│   │   │   ├── specification.rs   # 仕様パターン
│   │   │   ├── events.rs          # ドメインイベント
│   │   │   └── value_objects/     # 値オブジェクト
│   │   │       ├── task_id.rs
│   │   │       ├── task_title.rs
│   │   │       ├── task_description.rs
│   │   │       ├── status.rs
│   │   │       ├── priority.rs
│   │   │       ├── due_date.rs
│   │   │       └── ...
│   │   ├── tag/            # タグ集約
│   │   │   ├── aggregate.rs       # TagAggregate（集約ルート）
│   │   │   ├── repository.rs      # TagRepositoryトレイト定義
│   │   │   └── value_objects/     # 値オブジェクト
│   │   └── services/       # ドメインサービス
│   │       └── task_statistics_service.rs
│   ├── application/        # アプリケーション層（ユースケース）
│   │   ├── use_cases/
│   │   │   ├── task/       # タスク関連ユースケース
│   │   │   │   ├── add_task.rs
│   │   │   │   ├── list_tasks.rs
│   │   │   │   ├── edit_task.rs
│   │   │   │   ├── delete_task.rs
│   │   │   │   ├── show_task.rs
│   │   │   │   └── show_stats.rs
│   │   │   └── tag/        # タグ関連ユースケース
│   │   │       ├── add_tag.rs
│   │   │       ├── list_tags.rs
│   │   │       ├── edit_tag.rs
│   │   │       ├── delete_tag.rs
│   │   │       └── show_tag.rs
│   │   └── dto/            # データ転送オブジェクト
│   │       ├── task_dto.rs
│   │       ├── tag_dto.rs
│   │       └── stats_dto.rs
│   ├── infrastructure/     # インフラストラクチャ層
│   │   ├── database/
│   │   │   └── connection.rs     # DB接続管理
│   │   └── config/
│   │       └── app_config.rs     # アプリケーション設定
│   └── interface/          # インターフェース層
│       ├── cli/            # CLIインターフェース
│       │   ├── args.rs            # clapによる引数定義
│       │   ├── task_handler.rs    # タスクコマンドハンドラ
│       │   ├── tag_handler.rs     # タグコマンドハンドラ
│       │   └── display/           # 表示フォーマット
│       │       ├── task_table.rs
│       │       ├── tag_table.rs
│       │       └── stats_table.rs
│       ├── tui/            # TUIインターフェース
│       │   ├── app.rs             # アプリケーション状態
│       │   ├── event.rs           # イベントハンドリング
│       │   └── ui.rs              # UI描画
│       └── persistence/    # リポジトリ実装
│           ├── sea_orm/           # SeaORM実装（本番用）
│           │   ├── task_repository.rs
│           │   ├── tag_repository.rs
│           │   └── mapper.rs      # ドメイン↔エンティティマッピング
│           └── in_memory/         # インメモリ実装（テスト用）
│               ├── task_repository.rs
│               └── tag_repository.rs
├── migration/              # データベースマイグレーション
├── entity/                 # SeaORM自動生成エンティティ
├── justfile                # タスクランナー設定
├── Cargo.toml              # Workspace設定
├── CLAUDE.md               # 開発ガイドライン
├── README.md               # プロジェクト説明
├── .editorconfig           # エディタ設定
└── .github/                # GitHub設定

```

## エントリーポイント
- **src/main.rs**: バイナリエントリーポイント（`yaru::run()`を呼び出す）
- **src/lib.rs**: ライブラリエントリーポイント
  - `run()`: コマンド引数で分岐
    - 引数なし → `run_tui()` → TUIモード
    - 引数あり → `run_cli_with_command()` → CLIモード

## 重要なパターン

### Repositoryパターン
- **定義**: domain層でトレイトを定義
- **実装**: interface/persistence層で実装
  - `SeaOrmTaskRepository`: SQLite永続化
  - `InMemoryTaskRepository`: テスト用メモリ実装

### Aggregate Root
- `TaskAggregate`: タスクの集約ルート（domain/task/aggregate.rs）
- `TagAggregate`: タグの集約ルート（domain/tag/aggregate.rs）

### Value Object
ドメイン概念を値オブジェクトとして表現（Newtypeパターン）:
- Status, Priority, TaskTitle, TaskDescription, DueDate
- TagName, TagId など

## 依存関係の方向
- domain ← application ← infrastructure
- domain ← application ← interface
- infrastructure層とinterface層は相互に依存しない
- domain層は他のどの層にも依存しない（依存性逆転の原則）
