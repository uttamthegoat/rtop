use crate::models::system_stats::SystemStats;
use crate::ui::theme::Theme;
use crate::utils::formatting::format_bytes;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_gpu_widget(frame: &mut Frame, area: Rect, stats: &SystemStats, theme: &Theme) {
    let block = Block::default()
        .title(" GPU ")
        .borders(Borders::ALL)
        .border_style(theme.border)
        .title_style(theme.title);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let gpu = &stats.gpu;
    if !gpu.present {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1)])
            .split(inner);
        frame.render_widget(Paragraph::new(Text::from(" No GPU detected")), layout[0]);
        return;
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); 4])
        .split(inner);

    frame.render_widget(
        Paragraph::new(Text::from(format!(" {}", gpu.name))),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new(Text::from(format!(" Temp:  {:>5.0}°C", gpu.temperature))),
        layout[1],
    );

    frame.render_widget(
        Paragraph::new(Text::from(format!(
            " Util:  {:>5.1}%",
            gpu.utilization
        ))),
        layout[2],
    );

    let mem_line = if gpu.memory_total == 0 {
        " Mem:   Shared".to_string()
    } else {
        format!(
            " Mem:   {} / {}",
            format_bytes(gpu.memory_used),
            format_bytes(gpu.memory_total)
        )
    };

    frame.render_widget(
        Paragraph::new(Text::from(mem_line)),
        layout[3],
    );
}
