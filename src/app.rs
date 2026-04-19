use crate::domain::{self, MatchData, MatchFinishedData, MatchState, RoundState};
use crossterm::event::{Event, KeyCode, KeyEventKind};

pub enum Screen {
    Home,
    Rules,
    Match(MatchState),
}

pub const HOME_MENU_LEN: usize = 2;

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    /// Home menu: 0 = Start Game, 1 = Rules.
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
            Screen::Rules => {
                if code == KeyCode::Esc {
                    self.screen = Screen::Home;
                }
            }
            Screen::Match(m_state) => match m_state {
                MatchState::MatchInProgress(m_data) => {
                    if code == KeyCode::Esc {
                        self.screen = Screen::Home;
                    } else {
                        Self::handle_match_in_progress(m_data, code);
                        if let Some(winner) = m_data.match_winner_if_any() {
                            let match_data =
                                std::mem::replace(m_data, domain::MatchData::new());
                            *m_state = MatchState::MatchFinised(MatchFinishedData {
                                match_data,
                                winner,
                            });
                        }
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
                self.home_selected = (self.home_selected + HOME_MENU_LEN - 1) % HOME_MENU_LEN;
            }
            KeyCode::Down => {
                self.home_selected = (self.home_selected + 1) % HOME_MENU_LEN;
            }
            KeyCode::Enter => match self.home_selected {
                0 => {
                    self.screen = Screen::Match(domain::MatchState::MatchInProgress(
                        domain::MatchData::new(),
                    ));
                }
                1 => {
                    self.screen = Screen::Rules;
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
