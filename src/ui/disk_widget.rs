use crate::models::system_stats::SystemStats;
use crate::ui::theme::Theme;
use crate::utils::formatting::format_bytes;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_disk_widget(frame: &mut Frame, area: Rect, stats: &SystemStats, theme: &Theme) {
    let block = Block::default()
        .title(" Disks ")
        .borders(Borders::ALL)
        .border_style(theme.border)
        .title_style(theme.title);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let disks = &stats.disks;
    if disks.is_empty() {
        return;
    }

    let rows_per = 3;
    let max = disks.len().min(inner.height as usize / rows_per);
    if max == 0 {
        return;
    }

    let constraints = vec![Constraint::Length(1); max * rows_per];
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    for (i, disk) in disks.iter().take(max).enumerate() {
        let total = format_bytes(disk.total);
        let used = format_bytes(disk.used);
        let avail = format_bytes(disk.available);
        let free_pct = 100.0 - disk.percent;
        let b = i * 3;

        frame.render_widget(
            Paragraph::new(Text::from(format!(
                " {}:{:>10}",
                disk.mount_point, total
            ))),
            layout[b],
        );

        frame.render_widget(
            Paragraph::new(Text::from(format!(
                "    Used: {:>5.1}% --- {:>7}",
                disk.percent, used
            ))),
            layout[b + 1],
        );

        frame.render_widget(
            Paragraph::new(Text::from(format!(
                "    Free: {:>5.1}% --- {:>7}",
                free_pct, avail
            ))),
            layout[b + 2],
        );
    }
}
