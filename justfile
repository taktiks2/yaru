# yaruプロジェクトのタスクランナー

# コードのフォーマットを実行
fmt:
    cargo fmt

# コードのリントを実行
lint:
    cargo clippy

# フォーマットとリントを順番に実行
check: fmt lint
    @echo "フォーマットとリントが完了しました"

# tasks.jsonとtags.jsonの両方を整形して出力
show-all:
    @echo "=== Tasks ==="
    @just show-data
    @echo ""
    @echo "=== Tags ==="
    @just show-tags

# tasks.jsonを整形して出力
show-data:
    @if [ -f ~/.config/yaru/tasks.json ]; then \
        jq '.' ~/.config/yaru/tasks.json; \
    else \
        echo "データファイルが存在しません"; \
    fi

# tags.jsonを整形して出力
show-tags:
    @if [ -f ~/.config/yaru/tags.json ]; then \
        jq '.' ~/.config/yaru/tags.json; \
    else \
        echo "タグデータファイルが存在しません"; \
    fi

# tasks.jsonファイルを削除
clean-data:
    rm -f ~/.config/yaru/tasks.json
    @echo "データファイルを削除しました"

# tasks.jsonとconfig.tomlの両方を削除
clean-all:
    rm -rf ~/.config/yaru/
    @echo "yaruの設定ディレクトリを削除しました"
