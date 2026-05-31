use crate::state::AppState;
use crate::ui::theme::Theme;
use crate::utils::formatting::format_bytes;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Row, Table, TableState};
use ratatui::Frame;
use ratatui::prelude::Style;

pub fn render_process_table(frame: &mut Frame, area: Rect, state: &AppState, theme: &Theme) {
    let header_style = Style::default().fg(theme.title).bg(theme.background);
    let selected_style = theme.selected_row;

    let headers = ["PID", "USER", "CPU%", "MEM", "STATE", "THR", "NAME"];
    let header_cells: Vec<Line> = headers
        .iter()
        .map(|h| {
            Line::from(Span::styled(*h, header_style))
        })
        .collect();

    let header = Row::new(header_cells);

    let rows: Vec<Row> = state
        .processes
        .iter()
        .map(|p| {
            let cpu = format!("{:5.1}", p.cpu_percent);
            let mem = format_bytes(p.memory_rss);
            let cells = vec![
                p.pid.to_string(),
                p.user.clone(),
                cpu,
                mem,
                p.state.clone(),
                p.threads.to_string(),
                p.name.clone(),
            ];
            let style = Style::default().fg(theme.text);
            Row::new(cells).style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            ratatui::layout::Constraint::Length(8),
            ratatui::layout::Constraint::Length(10),
            ratatui::layout::Constraint::Length(8),
            ratatui::layout::Constraint::Length(11),
            ratatui::layout::Constraint::Length(6),
            ratatui::layout::Constraint::Length(6),
            ratatui::layout::Constraint::Min(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Processes ")
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(theme.border)
            .title_style(theme.title),
    )
    .row_highlight_style(selected_style);

    let mut table_state = TableState::new().with_selected(Some(state.selected_row));
    frame.render_stateful_widget(table, area, &mut table_state);
}
