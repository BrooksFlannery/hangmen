use crossterm::event::{Event, KeyCode, KeyEventKind};

pub const HOME_OPTION_COUNT: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Match,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    /// Index into the Home menu: 0 = Start Match, 1 = Exit.
    pub home_selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Home,
            should_quit: false,
            home_selected: 0,
        }
    }
}

pub fn handle_event(app: &mut App, event: Event) {
    let Event::Key(key) = event else {
        return;
    };
    if key.kind != KeyEventKind::Press {
        return;
    }

    match app.screen {
        Screen::Home => match key.code {
            KeyCode::Up => {
                app.home_selected =
                    (app.home_selected + HOME_OPTION_COUNT - 1) % HOME_OPTION_COUNT;
            }
            KeyCode::Down => {
                app.home_selected = (app.home_selected + 1) % HOME_OPTION_COUNT;
            }
            KeyCode::Enter => match app.home_selected {
                0 => app.screen = Screen::Match,
                1 => app.should_quit = true,
                _ => {}
            },
            _ => {}
        },
        Screen::Match => match key.code {
            KeyCode::Esc | KeyCode::Char('b') => app.screen = Screen::Home,
            _ => {}
        },
    }
}
