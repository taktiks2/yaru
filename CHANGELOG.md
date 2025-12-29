# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

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