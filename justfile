# yaruプロジェクトのタスクランナー

# todo.jsonを整形して出力
show-data:
    @if [ -f ~/.config/yaru/todo.json ]; then \
        jq '.' ~/.config/yaru/todo.json; \
    else \
        echo "todo.jsonが存在しません"; \
    fi

# todo.jsonファイルを削除
clean-data:
    rm -f ~/.config/yaru/todo.json
    @echo "todo.jsonを削除しました"

# todo.jsonとconfig.tomlの両方を削除
clean-all:
    rm -rf ~/.config/yaru/
    @echo "yaruの設定ディレクトリを削除しました"
