use crate::models::system_stats::SystemStats;
use crate::ui::theme::Theme;
use crate::utils::formatting::format_bytes;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_network_widget(frame: &mut Frame, area: Rect, stats: &SystemStats, theme: &Theme) {
    let block = Block::default()
        .title(" Network ")
        .borders(Borders::ALL)
        .border_style(theme.border)
        .title_style(theme.title);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let interfaces = &stats.network_info;
    if interfaces.is_empty() {
        return;
    }

    let rows_per = 5;
    let max = interfaces.len().min(inner.height as usize / rows_per);
    if max == 0 {
        return;
    }

    let constraints = vec![Constraint::Length(1); max * rows_per];
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    for (i, info) in interfaces.iter().take(max).enumerate() {
        let rx = format_bytes(info.current_rx);
        let top_rx = format_bytes(info.top_rx);
        let tot_rx = format_bytes(info.total_rx);
        let tx = format_bytes(info.current_tx);
        let top_tx = format_bytes(info.top_tx);
        let tot_tx = format_bytes(info.total_tx);

        let b = i * 5;

        frame.render_widget(
            Paragraph::new(Text::from(format!(" {}", info.interface))),
            layout[b],
        );

        frame.render_widget(
            Paragraph::new(Text::from(format!(
                "{:>18}    {:>17}",
                "Download", "Upload"
            ))),
            layout[b + 1],
        );

        frame.render_widget(
            Paragraph::new(Text::from(format!(
                " {:<7}: {:>8}    {:<7}: {:>8}",
                "Current", rx, "Current", tx
            ))),
            layout[b + 2],
        );

        frame.render_widget(
            Paragraph::new(Text::from(format!(
                " {:<7}: {:>8}    {:<7}: {:>8}",
                "Top", top_rx, "Top", top_tx
            ))),
            layout[b + 3],
        );

        frame.render_widget(
            Paragraph::new(Text::from(format!(
                " {:<7}: {:>8}    {:<7}: {:>8}",
                "Total", tot_rx, "Total", tot_tx
            ))),
            layout[b + 4],
        );
    }
}
