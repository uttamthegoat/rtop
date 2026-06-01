use crate::models::process_info::ProcessInfo;
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

    let headers = ["PID", "USER", "CPU%", "MEM", "STATE", "THR", "NAME", "COMMAND"];
    let header_cells: Vec<Line> = headers
        .iter()
        .map(|h| {
            Line::from(Span::styled(*h, header_style))
        })
        .collect();

    let header = Row::new(header_cells);

    let visible_height = area.height.saturating_sub(2) as usize;

    let (rows, _scroll, rel_selected) = if state.show_tree {
        let children = build_tree(&state.processes);
        let roots: Vec<usize> = (0..state.processes.len())
            .filter(|&i| {
                let ppid = state.processes[i].ppid;
                ppid == 0 || !state.processes.iter().any(|p| p.pid == ppid)
            })
            .collect();
        let mut flat = Vec::new();
        flatten_tree(&children, &roots, 0, &state.processes, &mut flat);

        let tree_idx = if let Some(selected) = state.processes.get(state.selected_row) {
            flat.iter().position(|(p, _)| p.pid == selected.pid).unwrap_or(0)
        } else {
            0
        };

        let total = flat.len();
        let scroll = scroll_offset(tree_idx, visible_height, total);
        let end = (scroll + visible_height).min(total);
        let visible = &flat[scroll..end];

        let rows: Vec<Row> = visible.iter().map(|(p, depth)| {
            let cpu = format!("{:5.1}", p.cpu_percent);
            let mem = format_bytes(p.memory_rss);
            let indent = "  ".repeat(*depth);
            let cells = vec![
                p.pid.to_string(),
                p.user.to_string(),
                cpu,
                mem,
                format!(" {}", p.state),
                p.threads.to_string(),
                format!("{}{}", indent, p.name),
                p.command.to_string(),
            ];
            Row::new(cells).style(Style::default().fg(theme.text))
        }).collect();
        (rows, scroll, tree_idx.saturating_sub(scroll))
    } else {
        let total = state.processes.len();
        let scroll = scroll_offset(state.selected_row, visible_height, total);
        let end = (scroll + visible_height).min(total);
        let visible = &state.processes[scroll..end];

        let rows: Vec<Row> = visible.iter().map(|p| {
            let cpu = format!("{:5.1}", p.cpu_percent);
            let mem = format_bytes(p.memory_rss);
            let cells = vec![
                p.pid.to_string(),
                p.user.to_string(),
                cpu,
                mem,
                format!(" {}", p.state),
                p.threads.to_string(),
                p.name.to_string(),
                p.command.to_string(),
            ];
            Row::new(cells).style(Style::default().fg(theme.text))
        }).collect();
        (rows, scroll, state.selected_row.saturating_sub(scroll))
    };

    let title = if state.search_mode {
        format!(" Search: {} ", state.filter_text)
    } else {
        " Processes ".to_string()
    };

    let table = Table::new(
        rows,
        [
            ratatui::layout::Constraint::Length(8),
            ratatui::layout::Constraint::Length(10),
            ratatui::layout::Constraint::Length(8),
            ratatui::layout::Constraint::Length(11),
            ratatui::layout::Constraint::Length(6),
            ratatui::layout::Constraint::Length(6),
            ratatui::layout::Constraint::Length(20),
            ratatui::layout::Constraint::Min(25),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(title)
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(theme.border)
            .title_style(theme.title),
    )
    .row_highlight_style(selected_style);

    let mut table_state = TableState::new().with_selected(Some(rel_selected));
    frame.render_stateful_widget(table, area, &mut table_state);
}

fn scroll_offset(selected: usize, visible: usize, total: usize) -> usize {
    if visible >= total || total == 0 {
        0
    } else if selected >= visible / 2 {
        let s = selected - visible / 2;
        s.min(total - visible)
    } else {
        0
    }
}

fn build_tree(processes: &[ProcessInfo]) -> Vec<Vec<usize>> {
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); processes.len()];
    let pid_to_idx: std::collections::HashMap<u32, usize> = processes
        .iter()
        .enumerate()
        .map(|(i, p)| (p.pid, i))
        .collect();
    for (i, p) in processes.iter().enumerate() {
        if let Some(&parent) = pid_to_idx.get(&p.ppid) {
            if parent != i {
                children[parent].push(i);
            }
        }
    }
    children
}

fn flatten_tree<'a>(
    children: &[Vec<usize>],
    indices: &[usize],
    depth: usize,
    processes: &'a [ProcessInfo],
    result: &mut Vec<(&'a ProcessInfo, usize)>,
) {
    for &i in indices {
        if let Some(p) = processes.get(i) {
            result.push((p, depth));
            if let Some(grandchildren) = children.get(i) {
                flatten_tree(children, grandchildren, depth + 1, processes, result);
            }
        }
    }
}
