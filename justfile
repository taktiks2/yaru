# yaruプロジェクトのタスクランナー

# tasks.jsonを整形して出力
show-data:
    @if [ -f ~/.config/yaru/tasks.json ]; then \
        jq '.' ~/.config/yaru/tasks.json; \
    elif [ -f ~/.config/yaru/todo.json ]; then \
        echo "旧ファイル(todo.json)を表示:"; \
        jq '.' ~/.config/yaru/todo.json; \
    else \
        echo "データファイルが存在しません"; \
    fi

# tasks.jsonファイルを削除（旧todo.jsonも削除）
clean-data:
    rm -f ~/.config/yaru/tasks.json ~/.config/yaru/todo.json
    @echo "データファイルを削除しました"

# tasks.jsonとconfig.tomlの両方を削除
clean-all:
    rm -rf ~/.config/yaru/
    @echo "yaruの設定ディレクトリを削除しました"
