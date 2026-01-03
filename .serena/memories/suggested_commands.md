# よく使うコマンド

## 基本的なビルドと実行

### ビルド
```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release
```

### 実行
```bash
# TUIモード（対話的インターフェース）
cargo run

# CLIモード - タスク操作
cargo run -- task list              # タスク一覧
cargo run -- task add "タスク名"    # タスク追加
cargo run -- task complete <ID>     # タスク完了
cargo run -- task delete <ID>       # タスク削除

# CLIモード - タグ操作
cargo run -- tag list               # タグ一覧
cargo run -- tag add "タグ名"       # タグ追加
cargo run -- tag delete <ID>        # タグ削除
```

## コード品質（just使用）

### フォーマット
```bash
just fmt
# または
cargo fmt
```

### リント（警告をエラーとして扱う）
```bash
just lint
# または
cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings
```

### フォーマット + リント
```bash
just check
```

## テスト

```bash
# 全テスト実行
cargo test

# 特定のテスト実行
cargo test <テスト名>

# テスト出力を表示
cargo test -- --nocapture
```

## データベース管理（just使用）

### マイグレーションとシーダー
```bash
# データベースリセット（down -> up）+ シーダー実行
just db-reset

# エンティティファイルの再生成（SeaORM）
just db-generate

# データベースリセット + エンティティ再生成
just db-refresh
```

### データベース接続
```bash
# SQLite CLIで接続
just db-connect
# または
sqlite3 ~/.config/yaru/yaru.db
```

### 全データ削除
```bash
# 設定ディレクトリごと削除
just clean-all
# これは ~/.config/yaru/ を削除します
```

## マイグレーション作成

```bash
# 新しいマイグレーションファイルを作成
cd migration
sea-orm-cli migrate generate <マイグレーション名>
```

## システムコマンド（Darwin/macOS）

### ファイル操作
```bash
ls -la          # ファイル一覧（詳細）
find . -name    # ファイル検索
grep -r         # ファイル内容検索
```

### Git操作
```bash
git status      # 状態確認
git add .       # ステージング
git commit -m   # コミット
git push        # プッシュ
git log         # ログ確認
```

### その他のツール
```bash
cargo tree      # 依存関係ツリー表示
cargo clean     # ビルド成果物削除
```

## 環境変数（データベース操作時）

マイグレーション実行時は以下の環境変数が自動設定されます（justfile経由）:
```bash
DATABASE_URL="sqlite://$HOME/.config/yaru/yaru.db?mode=rwc"
RUN_SEEDER=1
```

## 開発ワークフロー

### 通常の開発フロー
1. コードを修正
2. `just check` でフォーマット+リント
3. `cargo test` でテスト
4. `cargo run` で動作確認
5. コミット

### データベース変更を伴う開発
1. マイグレーション作成: `cd migration && sea-orm-cli migrate generate <名前>`
2. マイグレーション実装
3. `just db-reset` でマイグレーション適用
4. `just db-generate` でエンティティ再生成
5. リポジトリ実装を更新
6. テスト実行

## Serena関連

```bash
# Serenaのセットアップ（初回のみ）
just serena-setup

# プロジェクトのインデックス作成
just serena-index
```
