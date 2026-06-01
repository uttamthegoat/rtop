use crate::models::process_info::ProcessInfo;
use crate::models::system_stats::SystemStats;

pub struct AppState {
    pub processes: Vec<ProcessInfo>,
    pub system_stats: SystemStats,
    pub filter_text: String,
    pub search_mode: bool,
    pub selected_row: usize,
    pub show_tree: bool,
    pub scroll_offset: usize,
    pub should_quit: bool,
    pub needs_refresh: bool,
    pub terminal_width: u16,
    pub terminal_height: u16,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            system_stats: SystemStats::default(),
            filter_text: String::new(),
            search_mode: false,
            selected_row: 0,
            show_tree: false,
            scroll_offset: 0,
            should_quit: false,
            needs_refresh: false,
            terminal_width: 0,
            terminal_height: 0,
        }
    }
}
