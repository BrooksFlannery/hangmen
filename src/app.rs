use std::collections::HashSet;

use crate::domain::{self, GuessingData, MatchFinishedData, MatchState, PlayerProfile, RoundState};
use crossterm::event::{Event, KeyCode, KeyEventKind};

pub const HOME_OPTION_COUNT: usize = 2;

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
}

pub fn handle_event(app: &mut App, event: Event) {
    let Event::Key(key) = event else {
        return;
    };
    if key.kind != KeyEventKind::Press {
        return;
    }

    match &mut app.screen {
        Screen::Home => match key.code {
            KeyCode::Up => {
                app.home_selected = (app.home_selected + HOME_OPTION_COUNT - 1) % HOME_OPTION_COUNT;
            }
            KeyCode::Down => {
                app.home_selected = (app.home_selected + 1) % HOME_OPTION_COUNT;
            }
            KeyCode::Enter => match app.home_selected {
                0 => {
                    app.screen =
                        Screen::Match(domain::MatchState::MatchInProgress(domain::MatchData::new()))
                }
                1 => app.should_quit = true,
                _ => {}
            },
            _ => {}
        },
        Screen::Match(m_state) => match m_state {
            MatchState::MatchInProgress(m_data) => match key.code {
                KeyCode::Esc => {
                    app.screen = Screen::Home;
                }
                _ => match &mut m_data.current_round {
                    RoundState::WordSelection(ws_data) => match key.code {
                        KeyCode::Char(c) => {
                            let c = c.to_ascii_lowercase();
                            if c.is_ascii_alphabetic()
                                && ws_data.input.len() < ws_data.word_length as usize
                            {
                                ws_data.input.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            ws_data.input.pop();
                        }
                        KeyCode::Enter => {
                            if ws_data.input.len() == ws_data.word_length as usize {
                                match ws_data.turn {
                                    domain::Turn::PlayerOne => {
                                        ws_data.player_one_word = Some(ws_data.input.clone());
                                        ws_data.turn = domain::Turn::PlayerTwo;
                                    }
                                    domain::Turn::PlayerTwo => {
                                        ws_data.player_two_word = Some(ws_data.input.clone());
                                        ws_data.turn = domain::Turn::PlayerOne;
                                    }
                                }
                                ws_data.input.clear();
                            }

                            if ws_data.player_one_word.is_some()
                                && ws_data.player_two_word.is_some()
                            {
                                let p1 = ws_data.player_one_word.take().unwrap();
                                let p2 = ws_data.player_two_word.take().unwrap();
                                let cur_turn = ws_data.turn;
                                m_data.current_round = RoundState::Guessing(domain::GuessingData {
                                    turn: cur_turn,
                                    player_one_word: p1,
                                    player_two_word: p2,
                                    guessed_letters: std::collections::HashSet::new(),
                                })
                            }
                        }
                        _ => {}
                    },
                    RoundState::Guessing(g_data) => match key.code {
                        KeyCode::Char(c) => {
                            let c = c.to_ascii_lowercase();
                            if c.is_ascii_alphabetic() {
                                g_data.guessed_letters.insert(c);
                            }
                            //Is this the rust way to do this?
                            let player_one_won =
                                check_win(&g_data.guessed_letters, &g_data.player_two_word);
                            let player_two_won =
                                check_win(&g_data.guessed_letters, &g_data.player_one_word);
                            //Is this the rust way to do this?
                            if player_one_won && player_two_won {
                                m_data.current_round =
                                    RoundState::Finished(domain::RoundResult::Draw);
                            } else if player_one_won {
                                m_data.current_round = RoundState::Finished(
                                    domain::RoundResult::Won(m_data.player_one.clone()),
                                )
                            } else if player_two_won {
                                m_data.current_round = RoundState::Finished(
                                    domain::RoundResult::Won(m_data.player_two.clone()),
                                )
                            } else {
                                match g_data.turn {
                                    domain::Turn::PlayerOne => {
                                        g_data.turn = domain::Turn::PlayerTwo;
                                    }
                                    domain::Turn::PlayerTwo => {
                                        g_data.turn = domain::Turn::PlayerOne;
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    RoundState::Finished(result) => match key.code {
                        KeyCode::Enter => {
                            m_data.current_round = RoundState::new(domain::Turn::PlayerOne);
                        }
                        _ => {}
                    },
                },
            },
            MatchState::MatchFinised(_) => {
                app.screen = Screen::Home;
            }
        },
    }
}

fn check_win(guessed: &HashSet<char>, word: &str) -> bool {
    word.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| guessed.contains(&c.to_ascii_lowercase()))
}
