# タスク完了時のチェックリスト

タスクを完了する際は、以下のチェックリストを実行してください。

## TDD（テスト駆動開発）のワークフロー

このプロジェクトではTDDを原則として採用しています：

1. **テストファースト**
   - 期待される入出力に基づき、まずテストを作成
   - 実装コードは書かず、テストのみを用意
   - テストを実行し、失敗を確認
   - テストが正しいことを確認できた段階でコミット

2. **実装フェーズ**
   - テストをパスさせる実装を進める
   - 実装中はテストを変更せず、コードを修正し続ける
   - すべてのテストが通過するまで繰り返す

## コード品質チェック

### 1. フォーマット
```bash
cargo fmt
# または
just fmt
```

### 2. リントチェック
```bash
cargo clippy
# または
just lint
```

### 3. 両方をまとめて実行
```bash
just check
```

## テストの実行

### 全テストの実行
```bash
cargo test
```

### 関連するテストのみ実行
```bash
# 特定のモジュール
cargo test <module_name>

# 特定のテスト関数
cargo test <test_name>
```

## コミット

### 1. 変更内容の確認
```bash
git status
git diff
```

### 2. Conventional Commitsに従ったコミット
```bash
# 適切なprefixを使用
git commit -m "feat: 機能の説明"
git commit -m "fix: バグ修正の説明"
git commit -m "test: テストの追加/修正"
git commit -m "refactor: リファクタリングの説明"
```

**注意:** cocogittoがインストールされている場合、commit-msgフックが自動的にメッセージを検証します。

## ビルドの確認

### デバッグビルド
```bash
cargo build
```

### リリースビルド（必要に応じて）
```bash
cargo build --release
```

## チェックリストまとめ

- [ ] `cargo fmt` または `just fmt` でフォーマット
- [ ] `cargo clippy` または `just lint` でリントチェック
- [ ] `cargo test` で全テストが通過
- [ ] `cargo build` でビルドが成功
- [ ] Conventional Commitsの形式でコミット
- [ ] TDDの原則に従っている（テストファースト）
