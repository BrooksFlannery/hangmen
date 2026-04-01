use std::collections::HashSet;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::Paragraph,
};

use crate::{
    app::{App, Screen},
    domain::{MatchData, MatchState, RoundState},
};

/// Home banner art (UTF-8); must stay in sync with `home_banner.txt` line count.
const HOME_BANNER: &str = include_str!("home_banner.txt");
const HOME_BANNER_HEIGHT: u16 = 21;

pub fn render(app: &App, frame: &mut Frame) {
    match app.screen {
        Screen::Home => render_home(app, frame),
        Screen::Match(_) => render_match(app, frame),
    }
}

fn home_option_style(selected: bool) -> Style {
    if selected {
        Style::new().reversed()
    } else {
        Style::new()
    }
}

fn render_home(app: &App, frame: &mut Frame) {
    let banner = HOME_BANNER.trim_end_matches(&['\n', '\r'][..]);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(HOME_BANNER_HEIGHT),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(frame.area());

    frame.render_widget(Paragraph::new(banner).style(Style::new()), chunks[0]);
    frame.render_widget(
        Paragraph::new("Play").style(home_option_style(app.home_selected == 0)),
        chunks[2],
    );
    frame.render_widget(
        Paragraph::new("Exit").style(home_option_style(app.home_selected == 1)),
        chunks[3],
    );
}

fn render_match(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(frame.area());

    match &app.screen {
        Screen::Match(m_state) => match m_state {
            MatchState::MatchInProgress(m_data) => match &m_data.current_round {
                RoundState::WordSelection(ws_data) => {
                    let max_len = ws_data.word_length as usize;
                    let cur_len = ws_data.input.len();
                    let remaining = max_len - cur_len;

                    let masked_word = format!("{}{}", ws_data.input, "_".repeat(remaining));

                    let prompt = if cur_len != max_len {
                        format!("Write a secret word: {}", masked_word)
                    } else {
                        format!("Press ENTER to play: {}", ws_data.input)
                    };

                    let player = match ws_data.turn {
                        crate::domain::Turn::PlayerOne => "Player One",
                        crate::domain::Turn::PlayerTwo => "Player Two",
                    };

                    frame.render_widget(Paragraph::new(player), chunks[0]);
                    frame.render_widget(Paragraph::new(prompt), chunks[1]);
                }
                RoundState::Guessing(g_data) => {
                    let player_turn = match g_data.turn {
                        crate::domain::Turn::PlayerOne => "Press a key to guess (Player One)",
                        crate::domain::Turn::PlayerTwo => "Press a key to guess (Player Two)",
                    };

                    let masked_player_one_word = format!(
                        "Player One's Word: {}",
                        mask_word(&g_data.guessed_letters, &g_data.player_one_word)
                    );
                    let masked_player_two_word = format!(
                        "Player Two's Word: {}",
                        mask_word(&g_data.guessed_letters, &g_data.player_two_word)
                    );

                    frame.render_widget(Paragraph::new(player_turn), chunks[0]);
                    frame.render_widget(Paragraph::new(masked_player_one_word), chunks[1]);
                    frame.render_widget(Paragraph::new(masked_player_two_word), chunks[2]);
                }
                RoundState::Finished(result) => {
                    let message = match result {
                        crate::domain::RoundResult::Draw => "It's a draw".to_string(),
                        crate::domain::RoundResult::Won(winner) => {
                            format!("{} won", winner.player.name)
                        }
                    };

                    frame.render_widget(message, chunks[0]);
                    frame.render_widget(
                        Paragraph::new("Press ENTER to start next round"),
                        chunks[2],
                    );
                }
            },
            MatchState::MatchFinised(_) => {}
        },
        _ => {}
    }

    frame.render_widget(Paragraph::new("Press ESC to leave"), chunks[3]); // I would like to have this pegged to the bottom of the terminal if possible
}

fn mask_word(guessed: &HashSet<char>, word: &str) -> String {
    word.chars()
        .map(|c| {
            let lc = c.to_ascii_lowercase();
            if !lc.is_ascii_alphabetic() || guessed.contains(&lc) {
                c
            } else {
                '_'
            }
        })
        .collect() //what is .collect, what is |c|, how does map work in rust
}
