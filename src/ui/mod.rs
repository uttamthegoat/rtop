mod layout;
mod theme;
mod process_table;
mod cpu_widget;
mod memory_widget;
mod disk_widget;
mod network_widget;
mod gpu_widget;
mod footer;

use crate::state::AppState;
pub use process_table::*;
pub use cpu_widget::*;
pub use memory_widget::*;
pub use disk_widget::*;
pub use network_widget::*;
pub use gpu_widget::*;
pub use footer::*;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();
    let theme = theme::get_theme("default");

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(10),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(65),
            Constraint::Percentage(35),
        ])
        .split(main_layout[0]);

    let mid_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(main_layout[1]);

    render_cpu_widget(frame, top_layout[0], &state.system_stats, &theme);
    render_memory_widget(frame, top_layout[1], &state.system_stats, &theme);
    render_disk_widget(frame, mid_layout[0], &state.system_stats, &theme);
    render_network_widget(frame, mid_layout[1], &state.system_stats, &theme);
    render_gpu_widget(frame, mid_layout[2], &state.system_stats, &theme);
    render_process_table(frame, main_layout[2], state, &theme);
    render_footer(frame, main_layout[3], &theme);
}
