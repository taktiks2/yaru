/// TUIアプリケーションの状態を管理する構造体
pub struct App {
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initial_state() {
        let app = App::new();
        assert!(!app.should_quit());
    }

    #[test]
    fn test_app_quit() {
        let mut app = App::new();
        app.quit();
        assert!(app.should_quit());
    }
}
