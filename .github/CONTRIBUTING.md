# コントリビューションガイド

`yaru` へのコントリビューションに興味を持っていただきありがとうございます！
このドキュメントでは、プロジェクトへの貢献方法を説明します。

## 目次

- [開発環境のセットアップ](#開発環境のセットアップ)
- [開発フロー](#開発フロー)
- [テスト駆動開発（TDD）](#テスト駆動開発tdd)
- [コーディング規約](#コーディング規約)
- [コミットメッセージ](#コミットメッセージ)
- [プルリクエスト](#プルリクエスト)
- [Issue の作成](#issue-の作成)

## 開発環境のセットアップ

### 必要なツール

- Rust 1.83 以降（edition 2024 対応）
- Git

### セットアップ手順

```bash
# リポジトリをクローン
git clone https://github.com/YOUR_USERNAME/yaru.git
cd yaru

# ビルド
cargo build

# テスト実行
cargo test

# 実行してみる
cargo run -- list
```

## 開発フロー

1. Issue を作成または既存の Issue を確認
2. ブランチを作成（例: `feature/add-update-command`, `fix/date-format`）
3. TDD に従って開発（後述）
4. コミット
5. プルリクエストを作成

## テスト駆動開発（TDD）

**このプロジェクトは原則としてテスト駆動開発（TDD）で進めます。**

### TDD の手順

1. **テストを書く**
   - 期待される入出力に基づき、まずテストを作成
   - 実装コードは書かず、テストのみを用意

2. **テストを実行して失敗を確認**
   ```bash
   cargo test
   ```
   - テストが失敗することを確認（Red）

3. **テストをコミット**
   ```bash
   git add .
   git commit -m "test: タスク更新機能のテストを追加"
   ```

4. **実装する**
   - テストをパスさせる最小限の実装を行う
   - テストは変更せず、コードを修正し続ける

5. **テストが通ることを確認**
   ```bash
   cargo test
   ```
   - すべてのテストが通過するまで繰り返す（Green）

6. **リファクタリング（必要に応じて）**
   - テストが通った状態で、コードを改善（Refactor）

### テストの書き方

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_機能名() {
        // Arrange: テストの準備
        let temp_dir = tempdir().unwrap();

        // Act: テスト対象の実行
        let result = function_to_test();

        // Assert: 結果の検証
        assert_eq!(result, expected);
    }
}
```

## コーディング規約

### Rust コード

- `cargo fmt` でフォーマット
- `cargo clippy` で警告をチェック
- 日本語のエラーメッセージを使用（ユーザー向け）
- コメントは日本語でも英語でも可

```bash
# コードフォーマット
cargo fmt

# Lint チェック
cargo clippy -- -D warnings
```

### ファイル構成

プロジェクトのアーキテクチャに従ってください：

- `cli.rs`: CLI 引数のパース
- `commands/`: 各コマンドの実装
- `todo.rs`: ドメインモデル
- `repository/`: データアクセス層
- `utils/`: ユーティリティ関数

詳細は [CLAUDE.md](../CLAUDE.md) を参照してください。

## コミットメッセージ

Conventional Commits 形式を推奨します：

```
<type>: <subject>

<body>（オプション）
```

### Type の種類

- `feat`: 新機能
- `fix`: バグ修正
- `test`: テストの追加・修正
- `refactor`: リファクタリング
- `docs`: ドキュメントの変更
- `style`: コードフォーマット
- `chore`: ビルド、依存関係の更新など
- `ci`: CI/CD の変更

### 例

```bash
git commit -m "feat: タスク更新コマンドを追加"
git commit -m "test: リポジトリのロード機能のテストを追加"
git commit -m "fix: 日付フォーマットのバグを修正"
git commit -m "refactor: コマンド実行部分を関数に分離"
```

## プルリクエスト

### PR を作成する前に

- [ ] すべてのテストが通ることを確認（`cargo test`）
- [ ] コードがフォーマットされている（`cargo fmt`）
- [ ] Clippy の警告がない（`cargo clippy`）
- [ ] 必要に応じてドキュメントを更新

### PR の説明

テンプレートに従って記入してください：

- 変更内容の概要
- 関連する Issue
- テスト方法
- スクリーンショット（該当する場合）

## Issue の作成

適切なテンプレートを選択してください：

- **バグレポート**: バグや問題を報告
- **機能リクエスト**: 新機能の提案
- **リファクタリング**: コード改善の提案
- **開発環境・インフラ改善**: CI/CD、設定ファイルの改善

## 質問やサポート

- Issue で質問を投稿
- Pull Request でレビューを依頼

---

貢献をお待ちしています！ 🎉
