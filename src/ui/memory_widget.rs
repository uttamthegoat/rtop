use crate::models::system_stats::SystemStats;
use crate::ui::theme::Theme;
use crate::utils::formatting::format_bytes;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_memory_widget(frame: &mut Frame, area: Rect, stats: &SystemStats, theme: &Theme) {
    let block = Block::default()
        .title(" Memory ")
        .borders(Borders::ALL)
        .border_style(theme.border)
        .title_style(theme.title);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); 4])
        .split(inner);

    let lines = [
        format!(
            " Used:    {}",
            fmt_pad(format_bytes(stats.memory_used), 9)
        ),
        format!(
            " Avail:   {}",
            fmt_pad(format_bytes(stats.memory_available), 9)
        ),
        format!(
            " Cached:  {}",
            fmt_pad(format_bytes(stats.memory_cached), 9)
        ),
        format!(
            " Free:    {}",
            fmt_pad(format_bytes(stats.memory_free), 9)
        ),
    ];

    for (i, line) in lines.iter().enumerate() {
        frame.render_widget(Paragraph::new(Text::from(line.as_str())), layout[i]);
    }
}

fn fmt_pad(s: String, width: usize) -> String {
    if s.len() >= width {
        s
    } else {
        format!("{}{}", " ".repeat(width - s.len()), s)
    }
}
