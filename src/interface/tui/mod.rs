pub mod app;
pub mod event;
pub mod ui;

use anyhow::Result;
use ratatui::crossterm::{
    event as crossterm_event,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

use app::App;

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
        terminal.draw(|f| ui::render(f))?;

        // イベント処理（100msタイムアウト）
        if crossterm_event::poll(Duration::from_millis(100))? {
            if let crossterm_event::Event::Key(key) = crossterm_event::read()? {
                event::handle_key_event(&mut app, key);
            }
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