use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub highlight: Color,
    pub cpu_bar: Color,
    pub memory_bar: Color,
    pub disk_bar: Color,
    pub network_bar: Color,
    pub text: Color,
    pub text_dim: Color,
    pub border: Color,
    pub title: Color,
    pub selected_row: Style,
    pub cpu_core_colors: Vec<Color>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Reset,
            foreground: Color::White,
            highlight: Color::Yellow,
            cpu_bar: Color::Cyan,
            memory_bar: Color::Green,
            disk_bar: Color::Yellow,
            network_bar: Color::Blue,
            text: Color::White,
            text_dim: Color::DarkGray,
            border: Color::DarkGray,
            title: Color::White,
            selected_row: Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            cpu_core_colors: vec![
                Color::Cyan,
                Color::Magenta,
                Color::Yellow,
                Color::Red,
                Color::Green,
                Color::Blue,
            ],
        }
    }
}

pub fn get_theme(_name: &str) -> Theme {
    Theme::default()
}
