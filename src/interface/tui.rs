pub mod app;
pub mod event;
pub mod ui;

use anyhow::Result;
use app::App;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        self, execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::{io, time::Duration};

/// TUIモードで実行する
pub async fn run_tui() -> Result<()> {
    // ターミナルセットアップ
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // クリーンアップを保証するガード
    let _cleanup = CleanupGuard;

    // アプリケーション初期化
    let mut app = App::new();

    // イベントループ
    loop {
        // 画面描画
        terminal.draw(ui::render)?;

        // イベント処理（100msタイムアウト）
        if crossterm::event::poll(Duration::from_millis(100))?
            && let crossterm::event::Event::Key(key) = crossterm::event::read()?
        {
            event::handle_key_event(&mut app, key);
        }

        // 終了チェック
        if app.should_quit() {
            break;
        }
    }

    Ok(())
}

/// ターミナルのクリーンアップを保証する構造体
struct CleanupGuard;

impl Drop for CleanupGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}
