# 推奨コマンド

## ビルドとテスト

### ビルド
```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release
```

### テスト
```bash
# 全てのテストを実行
cargo test

# 特定のモジュールのテストを実行
cargo test repository
cargo test <module_name>

# 特定のテスト関数を実行
cargo test test_save_json
cargo test <test_name>

# テスト実行時に標準出力を表示
cargo test -- --nocapture
```

### コード品質チェック

#### justコマンドを使用（推奨）
```bash
# コードフォーマット
just fmt

# リントチェック
just lint

# フォーマット + リント
just check
```

#### Cargoコマンドを直接使用
```bash
# コードフォーマット
cargo fmt

# リントチェック
cargo clippy
```

## アプリケーション実行

### インストール
```bash
cargo install --path .
```

### 開発中の実行
```bash
# タスク管理
cargo run -- task list
cargo run -- task add --title "タスク" --status pending
cargo run -- task delete --id 1
cargo run -- task show --id 1

# タグ管理
cargo run -- tag list
cargo run -- tag add --name "重要"
cargo run -- tag delete --id 1
cargo run -- tag show --id 1
```

## データベース管理

### マイグレーション
```bash
# データベースのリセット（down -> up）
just db-reset

# エンティティファイルの再生成
just db-generate

# リセット + エンティティ再生成
just db-refresh

# データベースに接続（SQLite CLI）
just db-connect
```

### データのクリーンアップ
```bash
# 全ての設定とデータを削除
just clean-all
```

## Git/コミット関連

### Conventional Commits
```bash
# Git Hooksのインストール
cog install-hook commit-msg

# コミット例
git commit -m "feat: 新機能を追加"
git commit -m "fix: バグを修正"
git commit -m "docs: ドキュメントを更新"
```

## Darwin（macOS）システム固有のコマンド

### 一般的なコマンド
```bash
# ファイル検索
find . -name "*.rs"

# 内容検索
grep -r "pattern" src/

# ディレクトリサイズ確認
du -sh ~/.config/yaru/

# ディスク使用量
df -h
```

### Homebrewパッケージ管理
```bash
# パッケージのインストール
brew install <package>

# パッケージの更新
brew upgrade <package>

# インストール済みパッケージの確認
brew list
```

## Serena関連

### Serenaのセットアップ
```bash
# Serenaの初期化
just serena-setup

# プロジェクトのインデックス作成
just serena-index
```
