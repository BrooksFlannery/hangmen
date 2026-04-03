use crate::domain::{self, MatchData, MatchState, RoundState};
use crossterm::event::{Event, KeyCode, KeyEventKind};

pub const HOME_OPTION_COUNT: usize = 3;

pub enum Screen {
    Home,
    Match(MatchState),
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

    pub fn handle_event(&mut self, event: Event) {
        let Event::Key(key) = event else {
            return;
        };
        if key.kind != KeyEventKind::Press {
            return;
        }
        self.handle_key(key.code);
    }

    fn handle_key(&mut self, code: KeyCode) {
        match &mut self.screen {
            Screen::Home => self.handle_home_key(code),
            Screen::Match(m_state) => match m_state {
                MatchState::MatchInProgress(m_data) => {
                    if code == KeyCode::Esc {
                        self.screen = Screen::Home;
                    } else {
                        Self::handle_match_in_progress(m_data, code);
                    }
                }
                MatchState::MatchFinised(_) => {
                    self.screen = Screen::Home;
                }
            },
        }
    }

    fn handle_home_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Up => {
                self.home_selected =
                    (self.home_selected + HOME_OPTION_COUNT - 1) % HOME_OPTION_COUNT;
            }
            KeyCode::Down => {
                self.home_selected = (self.home_selected + 1) % HOME_OPTION_COUNT;
            }
            KeyCode::Enter => match self.home_selected {
                0 => {
                    self.screen = Screen::Match(domain::MatchState::MatchInProgress(
                        domain::MatchData::new(),
                    ));
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_match_in_progress(m_data: &mut MatchData, code: KeyCode) {
        use RoundState::*;
        if matches!(&m_data.current_round, WordSelection(_)) {
            match code {
                KeyCode::Char(c) => m_data.word_selection_push_char(c),
                KeyCode::Backspace => m_data.word_selection_backspace(),
                KeyCode::Enter => m_data.word_selection_enter(),
                _ => {}
            }
        } else if matches!(&m_data.current_round, Guessing(_)) {
            if let KeyCode::Char(c) = code {
                m_data.guessing_char(c);
            }
        } else if matches!(&m_data.current_round, Finished(_)) && code == KeyCode::Enter {
            m_data.round_finished_enter();
        }
    }
}
