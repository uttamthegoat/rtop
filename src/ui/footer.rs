use crate::state::AppState;
use crate::ui::theme::Theme;
use ratatui::layout::{Alignment, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub fn render_footer(frame: &mut Frame, area: Rect, state: &AppState, theme: &Theme) {
    let mut spans: Vec<Span> = vec![];

    if let Some(ref msg) = state.status_message {
        spans.push(Span::styled(format!(" {} ", msg), theme.text));
    } else {
        let keys = vec![
            ("q", "Quit"),
            ("↑/↓", "Nav"),
            ("/", "Search"),
            ("k", "Kill"),
            ("t", "Tree"),
            ("r", "Refresh"),
        ];
        spans = keys
            .iter()
            .flat_map(|(key, desc)| {
                vec![
                    Span::styled(format!(" {} ", key), theme.title),
                    Span::styled(*desc, theme.text_dim),
                ]
            })
            .collect();
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).alignment(Alignment::Left).block(
        Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(theme.border),
    );

    frame.render_widget(paragraph, area);
}
