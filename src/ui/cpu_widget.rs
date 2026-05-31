use crate::models::system_stats::SystemStats;
use crate::ui::theme::Theme;
use crate::utils::formatting::format_percent;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_cpu_widget(frame: &mut Frame, area: Rect, stats: &SystemStats, theme: &Theme) {
    let title = format!(
        " CPU ({} cores) {} ",
        stats.cpu_per_core.len(),
        format_percent(stats.cpu_usage)
    );
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(theme.border)
        .title_style(theme.title);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let num_cores = stats.cpu_per_core.len();
    if num_cores == 0 {
        return;
    }

    let rows = ((num_cores + 1) / 2).min(inner.height as usize);
    if rows == 0 {
        return;
    }

    let row_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); rows])
        .split(inner);

    for r in 0..rows {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(row_layout[r]);

        let left = r;
        let right = r + rows;

        let mut render_core = |i: usize, area: Rect| {
            if i >= num_cores {
                return;
            }
            let pct = stats.cpu_per_core[i];
            let temps = &stats.cpu_temperatures;
            let line = if !temps.is_empty() {
                let temp = temps[i % temps.len()];
                format!(" Core {}: {:>5.1}%  {:>5.1}\u{00B0}C", i, pct, temp)
            } else {
                format!(" Core {}: {:>5.1}%", i, pct)
            };
            frame.render_widget(Paragraph::new(Text::from(line)), area);
        };

        render_core(left, cols[0]);
        render_core(right, cols[1]);
    }
}
