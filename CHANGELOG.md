# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.9.0](https://github.com/taktiks2/yaru/compare/825930222ba3f5a8e5c2b303f6513f19365b3209..0.9.0) - 2026-01-03
#### Features
- タスク統計情報表示機能を追加 - ([cf91a41](https://github.com/taktiks2/yaru/commit/cf91a419c4b063c71ce227df64983efb9a0aa96a)) - taktiks2
#### Refactoring
- calculate_stats関数を単一ループに最適化 - ([e09360c](https://github.com/taktiks2/yaru/commit/e09360c02729f427af32ca83c7b47a5491e768bc)) - taktiks2
- StatusとPriorityにCopyトレイトを追加 - ([5a8e09e](https://github.com/taktiks2/yaru/commit/5a8e09e27c8c01625d79e114891a344c270cf0c0)) - taktiks2
- StatusとPriorityにEq, Hashトレイトを追加 - ([8259302](https://github.com/taktiks2/yaru/commit/825930222ba3f5a8e5c2b303f6513f19365b3209)) - taktiks2
#### Miscellaneous Chores
- clippyによるリントを厳密にする - ([30bd23a](https://github.com/taktiks2/yaru/commit/30bd23a4aa2ef630749fb9a5629400160d9da611)) - taktiks2
#### Styling
- フォーマット - ([6e9e93f](https://github.com/taktiks2/yaru/commit/6e9e93fe21e7ad895702fa91b3fc1e285c53c934)) - taktiks2

- - -

## [0.8.0](https://github.com/taktiks2/yaru/compare/c315337765dfa6f5a73de78606c332493ef6067f..0.8.0) - 2026-01-02
#### Features
- editコマンドの実装を追加 - ([c315337](https://github.com/taktiks2/yaru/commit/c315337765dfa6f5a73de78606c332493ef6067f)) - taktiks2
#### Documentation
- copilotのレビュールールを追加 - ([8d22912](https://github.com/taktiks2/yaru/commit/8d229122ba35718b2cb9da98e79f0f130aa69a2a)) - taktiks2
#### Refactoring
- ステータス選択肢の順番変更 - ([db92d2d](https://github.com/taktiks2/yaru/commit/db92d2d6c5fcbd13379fb29303e265d122075797)) - taktiks2
- N+1問題の解消 - ([df04b55](https://github.com/taktiks2/yaru/commit/df04b552f47dc58a0dba37a5a8b22266a81c6441)) - taktiks2
- タグのバリデーションを一箇所にまとめる - ([a077a69](https://github.com/taktiks2/yaru/commit/a077a692a602b004f8c4f1c8fb721b6aa07b2a99)) - taktiks2
- 設定された処理はインタラクションで再度聞かないようにする - ([d3ffd3a](https://github.com/taktiks2/yaru/commit/d3ffd3a59ae0c633ed48fbcf786f0a3c5eb3ee07)) - taktiks2
- Params構造体の適用 - ([6143151](https://github.com/taktiks2/yaru/commit/61431518cd9ba3f458192f159a3aa2ef16f509b7)) - taktiks2
- 引数モードと対話モードの判定ロジックを明確化 - ([0deeaa8](https://github.com/taktiks2/yaru/commit/0deeaa858445635cba47f243e3a76e69f9912bb1)) - taktiks2

- - -

## [0.7.0](https://github.com/taktiks2/yaru/compare/534ad58dcde0667e20279ea314987d2a6bdb6f31..0.7.0) - 2026-01-02
#### Features
- タスクにdue_dateとcompleted_atフィールドを追加 - ([af84cc2](https://github.com/taktiks2/yaru/commit/af84cc27626599e717873a897255618a2a58de17)) - taktiks2
#### Documentation
- clippy実行時に-D warningsオプションを追加 - ([d1fcbcb](https://github.com/taktiks2/yaru/commit/d1fcbcba5c1b5bc8373ea5fd416e3add68bec609)) - taktiks2
- due_dateとcompleted_at機能に関する依存関係とドキュメントを更新 - ([bdcf3ca](https://github.com/taktiks2/yaru/commit/bdcf3ca2fe637a87361e046a82d984431e9dd1ca)) - taktiks2
#### Build System
- tasksテーブルにdue_dateとcompleted_atカラムを追加するマイグレーション - ([534ad58](https://github.com/taktiks2/yaru/commit/534ad58dcde0667e20279ea314987d2a6bdb6f31)) - taktiks2
#### Refactoring
- 不要な参照を削除してclippy警告を修正 - ([f3dd789](https://github.com/taktiks2/yaru/commit/f3dd7899e1589ed3ceb161e34e501bc0a9632855)) - taktiks2
- completed_at設定ロジックをor_elseで簡潔に記述 - ([745da67](https://github.com/taktiks2/yaru/commit/745da676a9fc9d8dc854a75399d8748c1ee74289)) - taktiks2
- format_optional_text関数の戻り値を&strに変更 - ([e31fb28](https://github.com/taktiks2/yaru/commit/e31fb284b9ebb55d4f7724fef35cf37cdb2da930)) - taktiks2
- マイグレーションファイル名をdue_date機能を含む名前に変更 - ([edc0ecf](https://github.com/taktiks2/yaru/commit/edc0ecfe19ed9f4e13db72137b992208cd678d30)) - taktiks2
- テーブル表示の"-"処理をヘルパー関数に集約 - ([57cb432](https://github.com/taktiks2/yaru/commit/57cb432041e5b6ebd65a707ec93e5061d693f347)) - taktiks2
- CLI引数のバリデーションをvalue_parserに統一 - ([aa152cb](https://github.com/taktiks2/yaru/commit/aa152cbb9beae1eeeb2d9f4e213fa1cce74d7139)) - taktiks2

- - -

## [0.6.0](https://github.com/taktiks2/yaru/compare/d91298401fdcad5e389fbf456cd66f5f4ea753e7..0.6.0) - 2026-01-01
#### Features
- タスク追加時の対話モードでVimモードを有効化 - ([62de22b](https://github.com/taktiks2/yaru/commit/62de22b4e9d2fa315683b7cddd59cfa8c6963c33)) - taktiks2
- タスク追加コマンドのインターフェースを拡張 - ([72bf616](https://github.com/taktiks2/yaru/commit/72bf616216ad89f4c4588755c19a8227e740045f)) - taktiks2
- CLIのインターフェース変更 - ([d8100c6](https://github.com/taktiks2/yaru/commit/d8100c63d2e3c2cee5c18546854407aba5a48793)) - taktiks2
#### Documentation
- seeder.rsに型パラメータとライフタイムの詳細な解説コメントを追加 - ([626e19b](https://github.com/taktiks2/yaru/commit/626e19bb271f7c003780da80b280439a30f1c708)) - taktiks2
#### Build System
- seeding機能の追加 - ([60fb699](https://github.com/taktiks2/yaru/commit/60fb699a5e47f2478d422e6e5d45f8ab623c1695)) - taktiks2
- Serena統合用のjustコマンドを追加 - ([3f6d4ad](https://github.com/taktiks2/yaru/commit/3f6d4adaf5e7899b412e54a503203fcebee6243f)) - taktiks2
- コマンドの修正 - ([d912984](https://github.com/taktiks2/yaru/commit/d91298401fdcad5e389fbf456cd66f5f4ea753e7)) - taktiks2
#### Continuous Integration
- cargo set-versionが実行できるようにする - ([19a464b](https://github.com/taktiks2/yaru/commit/19a464b595b5bec7514ee0702efd2623374a846c)) - taktiks2
- releaseフローの修正 - ([af81f72](https://github.com/taktiks2/yaru/commit/af81f729b674a1f4e5a7f7a5433cc4fff934e0f5)) - taktiks2
#### Refactoring
- 説明はデフォルト値を利用 - ([b120ae7](https://github.com/taktiks2/yaru/commit/b120ae72ec7d09788aada739f490028998c485ff)) - taktiks2
- タスク詳細表示でas_refの代わりにto_stringを使用 - ([aa6b5cf](https://github.com/taktiks2/yaru/commit/aa6b5cf5519e38bd7fa9e45a44bf046ee219c2b9)) - taktiks2
- タグ追加コマンドのインターフェースを改善 - ([4f2ed5e](https://github.com/taktiks2/yaru/commit/4f2ed5e92770f3abc9ecf800e2dfb3d74a03557a)) - taktiks2
- seederのトランザクション処理を改善 - ([594f5ef](https://github.com/taktiks2/yaru/commit/594f5ef15332069fde8f074036047b191cacfb00)) - taktiks2
- seeder用の関数を作成 - ([8b5c493](https://github.com/taktiks2/yaru/commit/8b5c4934899e2172d5afc8ad38bc1f4ea37db81d)) - taktiks2
- entityをライブラリ化と依存関係の整理 - ([21ba284](https://github.com/taktiks2/yaru/commit/21ba2843399b96834bb5faa25a7903ec500e817e)) - taktiks2
- commandsディレクトリをcommandにリネーム - ([aefcecf](https://github.com/taktiks2/yaru/commit/aefcecfc671426735753c86e4efca80182528212)) - taktiks2
#### Miscellaneous Chores
- lockの削除 - ([3937071](https://github.com/taktiks2/yaru/commit/3937071e73a274fa43a1e283c0d4b875720face8)) - taktiks2
- editionを統一 - ([53cc880](https://github.com/taktiks2/yaru/commit/53cc880c56d992138598cd1ecac3fc6ad9f4ae1b)) - taktiks2
- Serenaプロジェクト設定とメモリファイルを追加 - ([54eb522](https://github.com/taktiks2/yaru/commit/54eb52262762be6ab4a6f90a7259c9fbea31ace5)) - taktiks2
#### Styling
- フォーマット - ([918069e](https://github.com/taktiks2/yaru/commit/918069e31e3ce3dde140f083ee76775c6e17da06)) - taktiks2

- - -

## [0.5.0](https://github.com/taktiks2/yaru/compare/9cb94e1628391943dd23402d44f681a7e70bd878..0.5.0) - 2026-01-01
#### Features
- TaskRepositoryとTagRepositoryを実装 - ([b10e310](https://github.com/taktiks2/yaru/commit/b10e31091c2d741783d5a5073adfa1aadd090cb2)) - taktiks2
- SQLiteデータベースへの完全移行 - ([51f3d02](https://github.com/taktiks2/yaru/commit/51f3d02565b6569c6ea33fccdf78b60d8ba93fcb)) - taktiks2
- SqliteRepositoryを実装 - ([a636d71](https://github.com/taktiks2/yaru/commit/a636d7175af94b6c0d0db4196c903de7b44f177a)) - taktiks2
- SeaORM Entityを自動生成 - ([9b69dec](https://github.com/taktiks2/yaru/commit/9b69decdc913a9887fdd5f8d2b0d73b1b94c0c32)) - taktiks2
- tasks/tags/task_tagsテーブルのマイグレーションを実装 - ([967c68d](https://github.com/taktiks2/yaru/commit/967c68d4479736eec3db89d6ed0ab854cadb1f8d)) - taktiks2
#### Performance
- タスク取得時のN+1問題を解決 - ([58b1c8b](https://github.com/taktiks2/yaru/commit/58b1c8b16fcf4f79b671fed7785c523a2a6acc2a)) - taktiks2
#### Documentation
- リポジトリパターンをSQLite実装に更新 - ([04160b1](https://github.com/taktiks2/yaru/commit/04160b1ccb68ff719c0df1a4bdd2edb81de860dc)) - taktiks2
- CLAUDE.mdのRepository traitドキュメントを更新 - ([02db392](https://github.com/taktiks2/yaru/commit/02db39226e4181c54aa42fd2b2c4d7bf3c2ab817)) - taktiks2
- mod.rsの利用禁止ルールを追加 - ([cf00159](https://github.com/taktiks2/yaru/commit/cf00159a2bd86ce6670136d0a728f806ab8d0a2e)) - taktiks2
#### Tests
- テストコードをSQLite移行に対応 - ([6e9f6e9](https://github.com/taktiks2/yaru/commit/6e9f6e9d88c7d8c525db9dcd6fd0a892b698daff)) - taktiks2
#### Build System
- データベース用のコマンドを追加 - ([aa40d2f](https://github.com/taktiks2/yaru/commit/aa40d2fa5eb0609ee022979290e99643a0415522)) - taktiks2
- SeaORMとSQLite依存関係を追加 - ([3c91b88](https://github.com/taktiks2/yaru/commit/3c91b8846060ee1116e8ed3ca735e07bd569b9b2)) - taktiks2
#### Refactoring
- as_refへの変更 - ([5540f4e](https://github.com/taktiks2/yaru/commit/5540f4e6c7f20ee6294e60b439fd0aa9b29e9914)) - taktiks2
- tasks変数の削除 - ([26d7976](https://github.com/taktiks2/yaru/commit/26d7976e129047bba63aacc45a7254c7f19c1c2c)) - taktiks2
- コマンドレイヤーからall_tagsパラメータを削除 - ([3a9f891](https://github.com/taktiks2/yaru/commit/3a9f8919aa387a7d6fef94fe2cfdf793a1b6c115)) - taktiks2
- タグ表示をIDから名前に変更 - ([88c1eff](https://github.com/taktiks2/yaru/commit/88c1eff60534befe79ecbe034e5a56bddb670b38)) - taktiks2
- リポジトリの変換処理をトレイトベースに統一 - ([a0bd19e](https://github.com/taktiks2/yaru/commit/a0bd19e418159481eb3251569cab3800a7dd256e)) - taktiks2
- TaskのtagsをVec<i32>からVec<Tag>に変更 - ([a230af8](https://github.com/taktiks2/yaru/commit/a230af8fda4b5449045e8cadb9551692b8fc5fc7)) - taktiks2
- 未使用のJSONファイルパス設定を削除 - ([b519f0c](https://github.com/taktiks2/yaru/commit/b519f0c87105191dcb933b80dae6bf0239f7d891)) - taktiks2
- コマンドのエラーハンドリングとメッセージを改善 - ([f269715](https://github.com/taktiks2/yaru/commit/f26971524bf97b364db248a77a30020efc49d4cc)) - taktiks2
- StatusとPriorityのデータベース変換メソッドを追加 - ([ee53c20](https://github.com/taktiks2/yaru/commit/ee53c201c92d67dd5241917b25e057d1f3e92e91)) - taktiks2
- マイグレーションスキーマの外部キー制約とトリガー条件を改善 - ([8140028](https://github.com/taktiks2/yaru/commit/8140028fb5b7b705c399e97e1de89996b63f5384)) - taktiks2
- タスクリストのフィルタリング処理を改善 - ([d7b357e](https://github.com/taktiks2/yaru/commit/d7b357e61dbde2854bd32429245c641102fec863)) - taktiks2
- ID型をu64からi32に統一 - ([101fb1f](https://github.com/taktiks2/yaru/commit/101fb1f2b9d7ec6a5536a06272767c817a699c0d)) - taktiks2
- データベースマイグレーションのスキーマ定義を改善 - ([9f4d9a3](https://github.com/taktiks2/yaru/commit/9f4d9a3b0a117a86e4b8c122e3efc37d73ab1301)) - taktiks2
- DateTime型への変換 - ([83e3a7d](https://github.com/taktiks2/yaru/commit/83e3a7dcc27401f68bb42e168c02c3b9ad4724c6)) - taktiks2
- domain層の移動に伴いインポートパスを修正 - ([08e06ba](https://github.com/taktiks2/yaru/commit/08e06baeff55e17baafe1a07edf122c8bec7a21d)) - taktiks2
- ドメインモデルをdomain/配下に移動 - ([008ca9d](https://github.com/taktiks2/yaru/commit/008ca9d230801cc3ac2fd2fe60d7335da8c58fc4)) - taktiks2
- コマンド層を非同期化しTaskRepository/TagRepositoryを使用 - ([a5e36e9](https://github.com/taktiks2/yaru/commit/a5e36e9e614e3e6f3a9c5340c98b189319666c5a)) - taktiks2
- main.rsとlib.rsを非同期化 - ([2c85beb](https://github.com/taktiks2/yaru/commit/2c85beb23b311430308d1d95824f9b443609dff4)) - taktiks2
- SqliteRepositoryを削除しTaskRepository/TagRepositoryに移行 - ([c94df27](https://github.com/taktiks2/yaru/commit/c94df278f9fa3ab292d3ef60b3deaa86369f0283)) - taktiks2
- Repository<T>トレイトを非同期化しHasIdトレイトを削除 - ([801a52e](https://github.com/taktiks2/yaru/commit/801a52ef237db55ecf56fcb8f3799d2aab9e01e4)) - taktiks2
#### Miscellaneous Chores
- データベースパスを環境変数HOMEから動的に取得 - ([50bddf5](https://github.com/taktiks2/yaru/commit/50bddf5024825fb164a6e852ccb1f2856465ee6b)) - taktiks2
- 未使用のJSON関数にallow(dead_code)を追加 - ([6881fca](https://github.com/taktiks2/yaru/commit/6881fca0abb5981f3ac8d052ce5e84f448c0328f)) - taktiks2
- Sea-ORM Migrationの初期化 - ([9cb94e1](https://github.com/taktiks2/yaru/commit/9cb94e1628391943dd23402d44f681a7e70bd878)) - taktiks2
#### Styling
- format - ([c2ebf34](https://github.com/taktiks2/yaru/commit/c2ebf348959da5f124b461b1e8736ec1c3f09f2f)) - taktiks2
- フォーマット - ([ede4abb](https://github.com/taktiks2/yaru/commit/ede4abb6aeee0eb5cd92053cde9af5fad8c50e50)) - taktiks2

- - -

## [0.4.0](https://github.com/taktiks2/yaru/compare/fd8fe75aff051229705923d22cc33c08a5084180..0.4.0) - 2025-12-30
#### Features
- showコマンドの実装を追加 - ([312f261](https://github.com/taktiks2/yaru/commit/312f2617c19e430732030a9c38d682c421f56538)) - taktiks2
- CLI定義にshowサブコマンドを追加 - ([7cae69c](https://github.com/taktiks2/yaru/commit/7cae69cc816c8268a69b682ce31b4d9f8594ec8b)) - taktiks2
- displayモジュールに詳細表示用の関数を追加 - ([fd8fe75](https://github.com/taktiks2/yaru/commit/fd8fe75aff051229705923d22cc33c08a5084180)) - taktiks2
#### Documentation
- Showサブコマンドのコメントを正確な説明に修正 - ([c3da28c](https://github.com/taktiks2/yaru/commit/c3da28c3e68eef78d07fc6e043b4301d04ff6b61)) - taktiks2
- smart-commitコマンドのユーザー確認項目を明記 - ([42c1202](https://github.com/taktiks2/yaru/commit/42c12029c08863db48094a2d4deba6dda24635b4)) - taktiks2
- smart-commitコマンドのユーザー確認手順を明確化 - ([dc2d7b8](https://github.com/taktiks2/yaru/commit/dc2d7b83468633d3b311e3d8b2a85f54e4ac796c)) - taktiks2
- smart-commitコマンドの説明文を簡潔化 - ([66c084d](https://github.com/taktiks2/yaru/commit/66c084da20c1064f84e00ca2c02986951e369868)) - taktiks2
- smart-commitコマンドの手順を簡素化 - ([1678cb7](https://github.com/taktiks2/yaru/commit/1678cb711c8cf9e8788ad23269cab79ca2f93c6f)) - taktiks2
#### Continuous Integration
- リリースワークフローに並行実行制御を追加 - ([9bb26c6](https://github.com/taktiks2/yaru/commit/9bb26c61c4fde38551f925e2def1479d4ec81026)) - taktiks2
- Claude Codeワークフローをコメントイベントのみに限定 - ([79389ed](https://github.com/taktiks2/yaru/commit/79389ed4a5d8aec7b9fbe3f7ee8bcfb20d6872bc)) - taktiks2
#### Refactoring
- create_single_*_table関数をcreate_*_detail_table関数に統一 - ([8d2eb0b](https://github.com/taktiks2/yaru/commit/8d2eb0bcf8d123f63a80c06342bf6e272bc7da59)) - taktiks2
- anyhow::anyhow!をanyhow::bail!に統一 - ([6e5f726](https://github.com/taktiks2/yaru/commit/6e5f726a2b3be54a1ddf64aec9a762a000087a4b)) - taktiks2
#### Miscellaneous Chores
- smart-commitコマンドの定義を追加 - ([1c3f4bf](https://github.com/taktiks2/yaru/commit/1c3f4bf0ca95ff29f1bb3919926098be42efd908)) - taktiks2

- - -

## [0.3.0](https://github.com/taktiks2/yaru/compare/47cbb907c11f6d7d69b2cd9ab12f793ecc0af420..0.3.0) - 2025-12-29
#### Features
- タスク用のサブコマンド追加 - ([b7fe183](https://github.com/taktiks2/yaru/commit/b7fe18389968c6892c4b45d1c8889c0e5adb3758)) - taktiks2
#### Documentation
- CLAUDEとREADMEの更新 - ([af46173](https://github.com/taktiks2/yaru/commit/af461736ef63140d6f9c80df2f363a21250c43b9)) - taktiks2
#### Tests
- タスクコマンドの包括的なテストを追加 - ([4cdfca1](https://github.com/taktiks2/yaru/commit/4cdfca11fea7afb13d2ed4bf7dec3be911d56b54)) - taktiks2
#### Continuous Integration
- bump actions/checkout from 4 to 6 - ([47cbb90](https://github.com/taktiks2/yaru/commit/47cbb907c11f6d7d69b2cd9ab12f793ecc0af420)) - dependabot[bot]
#### Refactoring
- add_rowsメソッドの適用 - ([f14dcb8](https://github.com/taktiks2/yaru/commit/f14dcb83d114c47c5f0d8919e89722e4b11f8b51)) - taktiks2
- テーブル表示の空白フィールドを統一 - ([0512b89](https://github.com/taktiks2/yaru/commit/0512b896e65fde49fc5f2541af66f66c61eab54f)) - taktiks2
- テーブル表示処理を抽象化し、スタイルを統一 - ([9643c4d](https://github.com/taktiks2/yaru/commit/9643c4d6d244248c2da5510e37d9a7497e3793d8)) - taktiks2
- commandsモジュールをtaskとtagのサブモジュールに再構成 - ([2b3ee5a](https://github.com/taktiks2/yaru/commit/2b3ee5af568b18c734b90ccb6a0060ff1fd56f5f)) - taktiks2
#### Miscellaneous Chores
- (**deps**) bump tempfile from 3.23.0 to 3.24.0 - ([0ba29c5](https://github.com/taktiks2/yaru/commit/0ba29c58470870647307ef52aff4d41949f0ab48)) - dependabot[bot]
- (**deps**) bump toml from 0.8.23 to 0.9.10+spec-1.1.0 - ([d2a384c](https://github.com/taktiks2/yaru/commit/d2a384ce9c846610c72ac3270e9331c25ae57c07)) - dependabot[bot]
- (**deps**) bump serde_json from 1.0.145 to 1.0.148 - ([faf8f6f](https://github.com/taktiks2/yaru/commit/faf8f6f4e8c4208f396743b8cee5e75eb7c3e24f)) - dependabot[bot]

- - -

## [0.2.0](https://github.com/taktiks2/yaru/compare/d04330ef9ad5d17f6df35e95f6a59cd52c0f7bce..0.2.0) - 2025-12-29
#### Features
- (**cli**) タグ管理コマンドを追加 - ([cb0a8cb](https://github.com/taktiks2/yaru/commit/cb0a8cb53e260f3407a63ed9bcffe9a2f8b667c7)) - taktiks2
- (**commands**) タグ削除時の参照整合性チェックを追加 - ([2ac251c](https://github.com/taktiks2/yaru/commit/2ac251cca1936273fbd9171ee9bcd032adfee00a)) - taktiks2
- (**commands**) タスク追加時のタグ存在確認機能を追加 - ([3d154c9](https://github.com/taktiks2/yaru/commit/3d154c903bc6d8814738e5b65fda12d17441c1a1)) - taktiks2
- (**config**) タグファイルパスの設定を追加 - ([93c6212](https://github.com/taktiks2/yaru/commit/93c6212ad172129c85e621a449718aa9c1afb559)) - taktiks2
- (**display**) format_local_timeを公開 - ([3c05ff2](https://github.com/taktiks2/yaru/commit/3c05ff24587342080fca091fee9f04a948492b01)) - taktiks2
- (**repository**) TagRepositoryを追加 - ([b1b938d](https://github.com/taktiks2/yaru/commit/b1b938d51987fab6a3c8906a75213a48f11d605d)) - taktiks2
- (**tag**) Tag構造体を追加 - ([d04330e](https://github.com/taktiks2/yaru/commit/d04330ef9ad5d17f6df35e95f6a59cd52c0f7bce)) - taktiks2
- (**task**) タスクにtagsフィールドを追加 - ([f082c4a](https://github.com/taktiks2/yaru/commit/f082c4aed056c0ad9cfd6419c9542fce2d652bbd)) - taktiks2
#### Documentation
- (**json**) DeserializeOwnedを使用してドキュメントを改善 - ([9abf0f7](https://github.com/taktiks2/yaru/commit/9abf0f7f447134a020b25da9ccc79041a4a62c9e)) - taktiks2
#### Refactoring
- (**commands**) ジェネリックリポジトリに対応 - ([622cd96](https://github.com/taktiks2/yaru/commit/622cd9683f0f9e5b9c40e65c64d7006cdfe266f7)) - taktiks2
- (**lib**) タグ機能の統合に伴いリポジトリ連携を追加 - ([b9e0fc4](https://github.com/taktiks2/yaru/commit/b9e0fc41738648823e7843782bcfcdd506cd0e55)) - taktiks2
- (**repository**) 不要なallow(dead_code)属性を削除 - ([78c992c](https://github.com/taktiks2/yaru/commit/78c992c9f93cf1cf37aecc3ed17ca3d163ad6972)) - taktiks2
- (**repository**) リポジトリパターンをジェネリック化 - ([59524d6](https://github.com/taktiks2/yaru/commit/59524d6005a674fabd591e252722cf8baf3eaf0d)) - taktiks2
#### Miscellaneous Chores
- (**justfile**) タグ関連コマンドを追加 - ([f0c15be](https://github.com/taktiks2/yaru/commit/f0c15be4ac3e753e1d49617eb7ef15e4591860b0)) - taktiks2
- リントとフォーマットコマンドを追加、コードスタイルを統一 - ([050cc09](https://github.com/taktiks2/yaru/commit/050cc09220feba295b649d7a34c66377930d01b0)) - taktiks2
#### Styling
- (**display**) テストコードのフォーマットを統一 - ([ecfbbf6](https://github.com/taktiks2/yaru/commit/ecfbbf6286ccca941bc1843dfbfabd52beb62be4)) - taktiks2

- - -

## [0.1.1](https://github.com/taktiks2/yaru/compare/31131dfac130ec90c2c83b18750548e807444309..0.1.1) - 2025-12-29
#### Performance
- delete_taskのパフォーマンス改善 - ([15828e7](https://github.com/taktiks2/yaru/commit/15828e7d200d8b0af4d91722bfb4d64e2c3b1afa)) - taktiks2
#### Documentation
- 移行処理削除に合わせてドキュメントを更新 - ([ad04393](https://github.com/taktiks2/yaru/commit/ad04393c64506ea56df10dcc6b11e0a28765382f)) - taktiks2
- リファクタリングに合わせてドキュメントを更新 - ([bed81d6](https://github.com/taktiks2/yaru/commit/bed81d66eee06a842b49802af5478d1ffdcc2932)) - taktiks2
#### Refactoring
- todo.jsonからの移行処理を削除 - ([6c98461](https://github.com/taktiks2/yaru/commit/6c9846196aca3bc368b458d619c21520f555c69b)) - taktiks2
- エントリーポイントの型名・関数名を更新 - ([71c1f45](https://github.com/taktiks2/yaru/commit/71c1f455c0425e90d4836e0142974f9694e920c5)) - taktiks2
- 設定層のフィールド名を更新 - ([099ebdf](https://github.com/taktiks2/yaru/commit/099ebdf71827d145f214e4c88dfc4b0bfafeac8f)) - taktiks2
- 表示層の型名・関数名を更新 - ([aac4669](https://github.com/taktiks2/yaru/commit/aac466988d8de892ec926e072c439f8e08e06bb8)) - taktiks2
- コマンド層の型名・関数名を更新 - ([1b3689a](https://github.com/taktiks2/yaru/commit/1b3689ada790960e5746c70057b4e400655bd737)) - taktiks2
- Todo型をTask型に名称変更（コア型定義） - ([31131df](https://github.com/taktiks2/yaru/commit/31131dfac130ec90c2c83b18750548e807444309)) - taktiks2
#### Miscellaneous Chores
- PRマージコミットを無視する設定を追加 - ([1debcd4](https://github.com/taktiks2/yaru/commit/1debcd40a2d71de63541fc3a94b1e7ffacc9bd24)) - taktiks2
- perfコミットでパッチバンプするよう設定を追加 - ([d0eaae4](https://github.com/taktiks2/yaru/commit/d0eaae4f2073211d386d7b2ab0f371835d59e5e6)) - taktiks2
- バンプの設定追加 - ([17b6c07](https://github.com/taktiks2/yaru/commit/17b6c07cb838c29ae79cf092737c7aeb97e59909)) - taktiks2
- justfileをtasks.jsonに対応 - ([2426e94](https://github.com/taktiks2/yaru/commit/2426e94deecf9a4fcfe2ca23607e7350e9ab2562)) - taktiks2

- - -

## [0.1.0](https://github.com/taktiks2/yaru/compare/db57cadbcaf6d8a756451a58b304dbc628c0c266..0.1.0) - 2025-12-29
#### Features
- 設定ファイル読み込み機能の実装 - ([65e1bfb](https://github.com/taktiks2/yaru/commit/65e1bfb05012759661589f10ec223da8dcf791c3)) - taktiks2
- TOML設定ファイルの読み込み実装 - ([f179f78](https://github.com/taktiks2/yaru/commit/f179f7855c4bdb81d5fd8b24157b6cc83ca7e592)) - taktiks2
- Config構造体の実装 - ([2795cda](https://github.com/taktiks2/yaru/commit/2795cda5a9ff4adfa6c7d4e1b25b6474fb69ab6d)) - taktiks2
- 一覧表示のフィルター追加 - ([b1a09a3](https://github.com/taktiks2/yaru/commit/b1a09a362fc937be2442c98a9aefef3fd4021164)) - taktiks2
- ステータスの初期値設定 - ([2ec5ba8](https://github.com/taktiks2/yaru/commit/2ec5ba867b8713fb5019d59acf7df42656fd57d4)) - taktiks2
- ステータスと更新日の追加 - ([c82672d](https://github.com/taktiks2/yaru/commit/c82672d14cf10b32d7d68bd761e3b2def7ab7976)) - taktiks2
- エラー処理の実装 - ([52b0b65](https://github.com/taktiks2/yaru/commit/52b0b65c4cf8ce5f86b68347ea7d3e7b4a32bf8f)) - taktiks2
- yaruプロジェクトの生成 - ([db57cad](https://github.com/taktiks2/yaru/commit/db57cadbcaf6d8a756451a58b304dbc628c0c266)) - taktiks2
#### Documentation
- CLAUDE.mdとREADME.mdを追加、.gitignoreを更新 - ([e462d94](https://github.com/taktiks2/yaru/commit/e462d945ed3dbe95ae966ff2d98870e65c9782aa)) - taktiks2
#### Tests
- 設定ファイル読み込みロジックのテスト追加 - ([c1f5ebf](https://github.com/taktiks2/yaru/commit/c1f5ebf06013caca81d8e91940062808159e9cf1)) - taktiks2
- TOML設定ファイルの読み込みテスト追加 - ([eec5c75](https://github.com/taktiks2/yaru/commit/eec5c75e165fede15e8a0aac58c2f7d79d064d47)) - taktiks2
#### Refactoring
- 設定パス取得の重複コードを共通化 - ([4948537](https://github.com/taktiks2/yaru/commit/49485378bd1107c1d329e8e48a382475f3e05eca)) - taktiks2
- fmtの実行 - ([d1c5f8d](https://github.com/taktiks2/yaru/commit/d1c5f8d4fecbfcd29f0a1c020763ecf9797a930a)) - taktiks2
- 不要な統合テストの削除 - ([ea169d0](https://github.com/taktiks2/yaru/commit/ea169d07accf17f7f5add660e2ac6631c73585d3)) - taktiks2
- コマンド関数でリポジトリを引数として受け取るように変更 - ([f76ac79](https://github.com/taktiks2/yaru/commit/f76ac797fa99dafbb9475bd756e95f5f8c332808)) - taktiks2
- 設定ファイルをlib.rsに統合 - ([893bd14](https://github.com/taktiks2/yaru/commit/893bd148ba529364105168237148a526799d6d44)) - taktiks2
- thiserrorからanyhowへのエラーハンドリング変更 - ([c3abf4e](https://github.com/taktiks2/yaru/commit/c3abf4e9960a5a7200045231b24e3f7af680ab6e)) - taktiks2
- リポジトリパターンのトレイト化とunwrap()の削除 - ([d2324b6](https://github.com/taktiks2/yaru/commit/d2324b66e3006153b901396d935e4ea0e9c6b715)) - taktiks2
- モジュール構成の改善とコードの分離 - ([cf055e8](https://github.com/taktiks2/yaru/commit/cf055e80f0833fd1b97893990cd8a9599c46b323)) - taktiks2
- コードベース全体の改善とドキュメント追加 - ([e9cb438](https://github.com/taktiks2/yaru/commit/e9cb4389f61926528cc4959055d630f693ff7870)) - taktiks2
#### Miscellaneous Chores
- .gitignoreの変更 - ([1c539e0](https://github.com/taktiks2/yaru/commit/1c539e0725b936035ca24504ce29265d8f88a095)) - taktiks2
- バージョン管理の設定追加 - ([8fefade](https://github.com/taktiks2/yaru/commit/8fefade9c1a0096ba6f97d908c2cf11caadb0093)) - taktiks2
- cocogittoの設定ファイル追加 - ([47f478d](https://github.com/taktiks2/yaru/commit/47f478d994704fd1055b41f2679fca208137d6bb)) - taktiks2
- GitHub設定ファイルの追加 - ([40bb833](https://github.com/taktiks2/yaru/commit/40bb8335e2b14bfcf7854980e482447ccd3b82dc)) - taktiks2
- Claud Code用のコマンド追加 - ([dda8805](https://github.com/taktiks2/yaru/commit/dda8805893a0842faf5248f4b72fc503a3751162)) - taktiks2
- EditorConfigの追加 - ([9dbfc65](https://github.com/taktiks2/yaru/commit/9dbfc65a1d00fed6c818d24b147521ecaa6d4003)) - taktiks2
- GitHubのprテンプレートを追加 - ([7fc1dc2](https://github.com/taktiks2/yaru/commit/7fc1dc2747e2a0c656f54d38cd1c3bf85becdec4)) - taktiks2
- GitHubのissueテンプレートを追加 - ([6d26c44](https://github.com/taktiks2/yaru/commit/6d26c44b51fafcb61f2748de9eceed4f4bfac5d9)) - taktiks2
- GitHub Actions CIワークフローを追加 - ([18de137](https://github.com/taktiks2/yaru/commit/18de1370e9fe9bec68eec4253f8b8463cb1a3231)) - taktiks2

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).