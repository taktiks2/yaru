# yaruプロジェクトのタスクランナー

# データベースURL（環境変数HOMEから動的に取得）
db_url := "sqlite://" + env_var('HOME') + "/.config/yaru/yaru.db?mode=rwc"

# コードのフォーマットを実行
fmt:
    cargo fmt

# コードのリントを実行
lint:
    cargo clippy

# フォーマットとリントを順番に実行
check: fmt lint
    @echo "フォーマットとリントが完了しました"

# tasks.jsonとconfig.tomlの両方を削除
clean-all:
    rm -rf ~/.config/yaru/
    @echo "yaruの設定ディレクトリを削除しました"

# データベースのマイグレーションをリセット（down -> up）
db-reset:
    #!/usr/bin/env bash
    export DATABASE_URL="{{db_url}}"
    cd migration && cargo run -- down && cargo run -- up
    echo "データベースをリセットしました"

# エンティティファイルを再生成
db-generate:
    #!/usr/bin/env bash
    export DATABASE_URL="{{db_url}}"
    sea-orm-cli generate entity -o src/entity
    mv src/entity/mod.rs src/entity.rs
    echo "エンティティファイルを生成しました"

# データベースリセット + エ���ティティ再生成
db-refresh: db-reset db-generate
    @echo "データベースのリセットとエンティティ生成が完了しました"

# sqlite3でデータベースに接続
db-connect:
    sqlite3 {{env_var('HOME')}}/.config/yaru/yaru.db

# serenaの初期化処理
serena-setup:
  claude mcp add serena -- uvx --from git+https://github.com/oraios/serena serena start-mcp-server --context claude-code --project "$(pwd)"

# project.ymlが生成されてから実行する
serena-index:
  uvx --from git+https://github.com/oraios/serena serena project index
