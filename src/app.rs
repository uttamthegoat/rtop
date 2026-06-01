use crate::config::Config;
use crate::event::{Event, EventLoop};
use crate::input::{map_key, Action};
use crate::state::AppState;
use crate::system::cpu::CpuCollector;
use crate::system::disk::DiskCollector;
use crate::system::gpu::GpuCollector;
use crate::system::memory::MemoryCollector;
use crate::system::network::NetworkCollector;
use crate::system::process::{ProcessCollector, kill_process};
use crate::ui;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::time::Duration;

pub struct App {
    state: AppState,
    config: Config,
    cpu_collector: CpuCollector,
    memory_collector: MemoryCollector,
    disk_collector: DiskCollector,
    network_collector: NetworkCollector,
    process_collector: ProcessCollector,
    gpu_collector: GpuCollector,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let mut state = AppState::new();
        state.show_tree = config.show_tree;
        Ok(Self {
            state,
            cpu_collector: CpuCollector::new(),
            memory_collector: MemoryCollector::new(),
            disk_collector: DiskCollector::new(),
            network_collector: NetworkCollector::new(),
            process_collector: ProcessCollector::new(),
            gpu_collector: GpuCollector::new(),
            config,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        let tick_rate = Duration::from_millis(self.config.refresh_rate_ms);
        let mut event_loop = EventLoop::new(tick_rate);

        let result = self.main_loop(&mut terminal, &mut event_loop).await;

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }

    async fn main_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        events: &mut EventLoop,
    ) -> Result<()> {
        loop {
            if self.state.should_quit {
                break;
            }

            if let Some(event) = events.next().await {
                match event {
                    Event::Tick => {
                        self.state.status_message = None;
                        self.refresh_data().await;
                        self.draw(terminal)?;
                    }
                    Event::Key(key) => {
                        if self.state.search_mode {
                            self.handle_search_key(key);
                        } else {
                            let action = map_key(key);
                            self.handle_action(action);
                            self.draw(terminal)?;
                        }
                        if self.state.needs_refresh {
                            self.refresh_data().await;
                            self.state.needs_refresh = false;
                        }
                    }
                    Event::Resize(w, h) => {
                        self.state.terminal_width = w;
                        self.state.terminal_height = h;
                        self.draw(terminal)?;
                    }
                    Event::Mouse(_) => {}
                }
            }
        }
        Ok(())
    }

    async fn refresh_data(&mut self) {
        self.cpu_collector.refresh().await;
        self.memory_collector.refresh().await;
        self.disk_collector.refresh().await;
        self.network_collector.refresh().await;
        self.process_collector.refresh().await;
        self.gpu_collector.refresh().await;

        self.state.system_stats.cpu_usage = self.cpu_collector.usage();
        self.state.system_stats.memory_total = self.memory_collector.total();
        self.state.system_stats.memory_used = self.memory_collector.used();
        self.state.system_stats.memory_available = self.memory_collector.available();
        self.state.system_stats.memory_cached = self.memory_collector.cached();
        self.state.system_stats.memory_free = self.memory_collector.free();
        self.state.system_stats.memory_percent = self.memory_collector.percent();
        self.state.system_stats.swap_total = self.memory_collector.swap_total();
        self.state.system_stats.swap_used = self.memory_collector.swap_used();
        self.state.system_stats.swap_percent = self.memory_collector.swap_percent();
        self.state.system_stats.cpu_per_core = self.cpu_collector.per_core_usage().to_vec();
        self.state.system_stats.cpu_temperatures = self.cpu_collector.temperatures().to_vec();
        self.state.system_stats.disks = self.disk_collector.disks().to_vec();

        self.state.system_stats.network_info = self
            .network_collector
            .extended_info()
            .into_iter()
            .filter(|n| n.interface != "lo")
            .collect();

        self.state.system_stats.gpu = self.gpu_collector.info().clone();

        let mut processes = self.process_collector.processes();
        processes.sort_by(|a, b| b.cpu_percent.total_cmp(&a.cpu_percent));

        if self.state.search_mode && !self.state.filter_text.is_empty() {
            let filter = self.state.filter_text.to_lowercase();
            processes.retain(|p| {
                p.name.to_lowercase().contains(&filter)
                    || p.command.to_lowercase().contains(&filter)
            });
        }

        self.state.processes = processes;
        let max = self.state.processes.len().saturating_sub(1);
        if self.state.selected_row > max {
            self.state.selected_row = max;
        }
    }

    fn handle_search_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.state.filter_text.push(c);
                self.state.needs_refresh = true;
            }
            KeyCode::Backspace => {
                self.state.filter_text.pop();
                self.state.needs_refresh = true;
            }
            KeyCode::Esc => {
                self.state.search_mode = false;
                self.state.filter_text.clear();
                self.state.needs_refresh = true;
            }
            KeyCode::Enter => {
                self.state.search_mode = false;
                self.state.needs_refresh = true;
            }
            _ => {}
        }
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.state.should_quit = true,
            Action::Up => {
                if self.state.selected_row > 0 {
                    self.state.selected_row -= 1;
                }
            }
            Action::Down => {
                if self.state.selected_row < self.state.processes.len().saturating_sub(1) {
                    self.state.selected_row += 1;
                }
            }
            Action::PageUp => {
                let page_size = self.state.terminal_height.saturating_sub(10).max(1) as usize;
                self.state.selected_row = self.state.selected_row.saturating_sub(page_size);
            }
            Action::PageDown => {
                let page_size = self.state.terminal_height.saturating_sub(10).max(1) as usize;
                self.state.selected_row = (self.state.selected_row + page_size)
                    .min(self.state.processes.len().saturating_sub(1));
            }
            Action::Home => self.state.selected_row = 0,
            Action::End => {
                self.state.selected_row = self.state.processes.len().saturating_sub(1);
            }
            Action::Search => {
                self.state.search_mode = !self.state.search_mode;
                if !self.state.search_mode {
                    self.state.filter_text.clear();
                }
            }
            Action::Kill => {
                if let Some(process) = self.state.processes.get(self.state.selected_row) {
                    match kill_process(process.pid) {
                        Ok(()) => self.state.status_message = Some(format!("Killed PID {}", process.pid)),
                        Err(e) => self.state.status_message = Some(format!("Kill failed: {}", e)),
                    }
                }
            }
            Action::ToggleTree => self.state.show_tree = !self.state.show_tree,
            Action::Refresh => {
                self.state.needs_refresh = true;
            }
            Action::TabNext => {}
            Action::Enter => {}
            Action::Escape => {
                if self.state.search_mode {
                    self.state.search_mode = false;
                    self.state.filter_text.clear();
                }
            }
            Action::Unknown => {}
        }
    }

    fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<()> {
        let state = &self.state;
        terminal.draw(|f| {
            ui::render(f, state);
        })?;
        Ok(())
    }
}
