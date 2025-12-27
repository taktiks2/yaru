---
description: GitHubのissueテンプレートに則ったissueを作成する
argument-hint: [bug|feature] [タイトル]
allowed-tools: Bash(gh:*), Read
---

# GitHub Issue作成コマンド

引数 `$1` に基づいて、プロジェクトのissueテンプレートに則ったGitHub issueを作成してください。

## Issue タイプ

- `bug`: バグレポート（ラベル: bug）
- `feature`: 機能リクエスト（ラベル: enhancement）

## 手順

1. **テンプレートの確認**
   - `$1` が "bug" の場合: `.github/ISSUE_TEMPLATE/bug_report.md` を読み込む
   - `$1` が "feature" の場合: `.github/ISSUE_TEMPLATE/feature_request.md` を読み込む

2. **issueボディの作成**
   - テンプレートの構造に従って、各セクションを含めたissueボディを作成
   - タイトルは `$2` 以降の引数を使用（`$ARGUMENTS` から `$1` を除いた部分）
   - タイトルプレフィックスを自動追加:
     - バグ: `[BUG] `
     - 機能: `[FEATURE] `

3. **issueの作成**
   - `gh issue create` コマンドを使用
   - `--title` でタイトルを指定
   - `--body` でテンプレートに則った本文を指定
   - `--label` で適切なラベルを指定
   - 作成後、issue URLを表示

## 使用例

```
/create-issue bug データが保存されない
/create-issue feature 優先度フィールドの追加
```

## 注意事項

- タイトルが指定されていない場合は、ユーザーに入力を求める
- issueボディは対話的に入力できるよう、セクションごとに質問する
- 作成前に内容を確認し、ユーザーの承認を得る
