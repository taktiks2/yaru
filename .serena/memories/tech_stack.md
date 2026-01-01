# 技術スタック

## 言語・エディション
- Rust (edition 2024)

## 主要な依存関係

### CLI・UI
- **clap** (4.5.53): コマンドライン引数パース（deriveフィーチャー使用）
- **inquire** (0.9.1): 対話的な入力（editorフィーチャー使用）
- **comfy-table** (7.2.1): テーブル表示
- **console** (0.16.2): カラフルなコンソール出力
- **indicatif** (0.18.3): プログレスバー表示

### データ処理
- **serde** (1.0.219): シリアライゼーション（deriveフィーチャー使用）
- **serde_json** (1.0.148): JSON形式のシリアライゼーション
- **toml** (0.9): TOML形式の設定ファイル読み込み
- **chrono** (0.4.42): 日時処理（serdeフィーチャー使用）

### データベース
- **sea-orm** (1.1.2): ORM（SQLite対応）
  - sqlx-sqlite: SQLiteバックエンド
  - runtime-tokio-rustls: 非同期ランタイム
  - macros: マクロサポート
  - with-chrono: chrono統合
- **tokio** (1.42): 非同期ランタイム（rt, macrosフィーチャー使用）
- **migration**: データベースマイグレーション（ワークスペース内のクレート）

### エラーハンドリング
- **anyhow** (1.0): 統一的なエラーハンドリング

### 開発・テスト
- **tempfile** (3.24): テスト時の一時ファイル操作（dev-dependency）

## ビルドツール
- **Cargo**: Rustの標準ビルドツール
- **just**: タスクランナー（開発コマンドの実行に使用）

## 開発ツール
- **cocogitto**: Conventional Commitsの検証
- **sea-orm-cli**: SeaORMのエンティティ生成
