use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub highlight: Color,
    pub cpu_bar: Color,
    pub memory_bar: Color,
    pub disk_bar: Color,
    pub network_bar: Color,
    pub gpu_bar: Color,
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
            highlight: Color::Cyan,
            cpu_bar: Color::Cyan,
            memory_bar: Color::Green,
            disk_bar: Color::Blue,
            network_bar: Color::Blue,
            gpu_bar: Color::Magenta,
            text: Color::White,
            text_dim: Color::DarkGray,
            border: Color::DarkGray,
            title: Color::Cyan,
            selected_row: Style::default()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            cpu_core_colors: vec![
                Color::Cyan,
                Color::Blue,
                Color::Green,
                Color::Magenta,
                Color::Red,
                Color::White,
            ],
        }
    }
}

pub fn get_theme(_name: &str) -> Theme {
    Theme::default()
}
