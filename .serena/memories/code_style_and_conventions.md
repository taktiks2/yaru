# コードスタイルと規約

## 基本スタイル（.editorconfig準拠）
- **文字コード**: UTF-8
- **改行**: LF（Unix-style）
- **末尾の空白**: 削除する
- **ファイル末尾**: 改行を追加する
- **Rustファイル**: スペース4つでインデント

## 命名規則
- **関数・変数**: `snake_case`
- **構造体・Enum・トレイト**: `PascalCase`
- **定数**: `SCREAMING_SNAKE_CASE`

## エラーハンドリング
- `anyhow::Result<T>`を戻り値の型として使用
- エラーメッセージは**日本語**で記述
- `.context("日本語のエラーメッセージ")`でコンテキストを追加
- 早期リターンには`anyhow::bail!("日本語のエラーメッセージ")`を使用

```rust
let config = load_config()
    .context("設定ファイルの読み込みに失敗しました")?;

if invalid_state {
    anyhow::bail!("無効な状態です: {}", details);
}
```

## ドキュメントとコメント
- **ドキュメントコメント**: `///`を使用し、**日本語**で記述
- 構造体、Enum、関数、モジュールには必ずドキュメントコメントを記載
- **インラインコメント**: `//`を使用し、**日本語**で記述
- 処理の意図が自明でない箇所にのみコメントを記載

## 値オブジェクト（Value Object）パターン
- **Newtypeパターン**を使用（例: `TaskTitle(String)`）
- コンストラクタ（`new`メソッド）でバリデーションを実施
- バリデーションエラーは日本語で`anyhow::bail!`
- 適切なトレイトを`derive`（`Debug`, `Clone`, `PartialEq`, `Eq`など）
- `value()`メソッドで内部の値にアクセス

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTitle(String);

impl TaskTitle {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            anyhow::bail!("タイトルは空にできません");
        }
        if value.len() > 100 {
            anyhow::bail!("タイトルは100文字以内にしてください");
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
```

## モジュール構成
- 値オブジェクトなどの概念は**個別ファイル**として定義
- **`mod.rs`は非推奨**: モジュールを定義する際は`mod.rs`ではなく、**モジュール名.rs**を使用する
  - 例: `src/domain/task/value_objects.rs` (mod.rsではない)
- モジュール名.rsで`pub use`により再エクスポート
- `use`文はクレートルートからの**絶対パス**を使用
- `use`文は層ごとにグループ化（`domain`, `application`, `infrastructure`, `interface`）

```rust
// src/domain/task/value_objects.rs
pub mod task_title;
pub mod task_description;

pub use task_title::TaskTitle;
pub use task_description::TaskDescription;
```

## トレイトとデリベーション
- 必要なトレイトを適切に`derive`する
- 非同期トレイトには`#[async_trait]`を使用
- よく使うトレイト: `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Serialize`, `Deserialize`

```rust
use async_trait::async_trait;

#[async_trait]
impl TaskRepository for SeaOrmTaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<TaskAggregate>> {
        // ...
    }
}
```

## テスト（TDD原則）
- **TDD（テスト駆動開発）**を原則とする
- テストは`#[cfg(test)] mod tests`内に記述
- エッジケースを含む包括的なテストを作成
- テスト関数名は`test_<対象>_<条件>`の形式

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_title_valid() {
        let title = TaskTitle::new("有効なタイトル").unwrap();
        assert_eq!(title.value(), "有効なタイトル");
    }

    #[test]
    fn test_task_title_empty() {
        let result = TaskTitle::new("");
        assert!(result.is_err());
    }
}
```

## コード品質
- **Clippy警告を厳格に扱う**: `-D warnings`オプションでコンパイル
- 不要な参照、未使用の変数は削除
- N+1問題などのパフォーマンス問題に注意
- バリデーションロジックは一箇所にまとめる
