use crate::ui::theme::Theme;
use ratatui::layout::{Alignment, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub fn render_footer(frame: &mut Frame, area: Rect, theme: &Theme) {
    let keys = vec![
        ("q", "Quit"),
        ("↑/↓", "Nav"),
        ("/", "Search"),
        ("k", "Kill"),
        ("t", "Tree"),
        ("r", "Refresh"),
    ];

    let spans: Vec<Span> = keys
        .iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::styled(format!(" {} ", key), theme.title),
                Span::styled(*desc, theme.text_dim),
            ]
        })
        .collect();

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).alignment(Alignment::Left).block(
        Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(theme.border),
    );

    frame.render_widget(paragraph, area);
}
