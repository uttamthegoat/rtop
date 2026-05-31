use crossterm::event::{KeyCode, KeyEvent};

pub enum Action {
    Quit,
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Search,
    Kill,
    ToggleTree,
    Refresh,
    TabNext,
    Enter,
    Escape,
    Unknown,
}

pub fn map_key(key: KeyEvent) -> Action {
    match (key.modifiers, key.code) {
        (_, KeyCode::Char('q')) => Action::Quit,
        (_, KeyCode::Esc) => Action::Escape,
        (_, KeyCode::Up) | (_, KeyCode::Char('k')) => Action::Up,
        (_, KeyCode::Down) | (_, KeyCode::Char('j')) => Action::Down,
        (_, KeyCode::PageUp) => Action::PageUp,
        (_, KeyCode::PageDown) => Action::PageDown,
        (_, KeyCode::Home) => Action::Home,
        (_, KeyCode::End) => Action::End,
        (_, KeyCode::Char('/')) => Action::Search,
        (_, KeyCode::Char('K')) => Action::Kill,
        (_, KeyCode::Char('t')) => Action::ToggleTree,
        (_, KeyCode::Char('r')) => Action::Refresh,
        (_, KeyCode::Tab) => Action::TabNext,
        (_, KeyCode::Enter) => Action::Enter,
        _ => Action::Unknown,
    }
}
