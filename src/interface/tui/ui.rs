use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// TUIの画面を描画する
pub fn render(frame: &mut Frame) {
    let area = frame.area();

    // 垂直方向に3分割（上部余白、中央コンテンツ、下部余白）
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30), // 上部余白
            Constraint::Percentage(40), // 中央コンテンツ
            Constraint::Percentage(30), // 下部余白
        ])
        .split(area);

    // タイトル
    let title = Paragraph::new("yaru")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(title, chunks[1]);

    // メッセージ（タイトルの下に表示）
    let message_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // タイトル用
            Constraint::Min(0),    // メッセージ用
        ])
        .split(chunks[1]);

    let message = Paragraph::new(vec![
        Line::from("準備中"),
        Line::from(""),
        Line::from(Span::styled(
            "q: 終了 | Ctrl+C: 終了",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .alignment(Alignment::Center);

    frame.render_widget(message, message_chunks[1]);
}
