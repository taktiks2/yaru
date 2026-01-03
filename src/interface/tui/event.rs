use crate::interface::tui::app::App;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// キーイベントを処理する
pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        // Ctrl+Cで終了
        KeyCode::Char('c') | KeyCode::Char('C')
            if key.modifiers.contains(KeyModifiers::CONTROL) =>
        {
            app.quit();
        }
        // qキーで終了
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.quit();
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_q_key_quits() {
        let mut app = App::new();
        let key_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);

        handle_key_event(&mut app, key_event);

        assert!(app.should_quit());
    }

    #[test]
    fn test_handle_uppercase_q_quits() {
        let mut app = App::new();
        let key_event = KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::SHIFT);

        handle_key_event(&mut app, key_event);

        assert!(app.should_quit());
    }

    #[test]
    fn test_handle_ctrl_c_quits() {
        let mut app = App::new();
        let key_event = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);

        handle_key_event(&mut app, key_event);

        assert!(app.should_quit());
    }

    #[test]
    fn test_handle_other_key_does_not_quit() {
        let mut app = App::new();
        let key_event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);

        handle_key_event(&mut app, key_event);

        assert!(!app.should_quit());
    }
}
