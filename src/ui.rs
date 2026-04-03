use std::collections::HashSet;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::Paragraph,
};

use crate::{
    app::{App, Screen},
    domain::{self, MatchState, RoundState, Turn},
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
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let main_area = outer[0];
    let footer_area = outer[1];
    let banner = HOME_BANNER.trim_end_matches(&['\n', '\r'][..]);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints([
            Constraint::Length(HOME_BANNER_HEIGHT),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(frame.area());

    frame.render_widget(Paragraph::new(banner).style(Style::new()), chunks[0]);
    frame.render_widget(
        Paragraph::new("Player vs Player").style(home_option_style(app.home_selected == 0)),
        chunks[2],
    );
    frame.render_widget(
        Paragraph::new("Player vs Bot").style(home_option_style(app.home_selected == 1)),
        chunks[3],
    );
    frame.render_widget(
        Paragraph::new("Bot vs Bot").style(home_option_style(app.home_selected == 2)),
        chunks[4],
    );
    frame.render_widget(Paragraph::new("Press ESC to Quit"), footer_area);
}

fn render_match(app: &App, frame: &mut Frame) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let header_area = outer[0];
    let main_area = outer[1];
    let footer_area = outer[2];

    let header_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(10),
            Constraint::Fill(1),
            Constraint::Length(10),
        ])
        .split(header_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(main_area);

    match &app.screen {
        Screen::Match(m_state) => match m_state {
            MatchState::MatchInProgress(m_data) => match &m_data.current_round {
                RoundState::WordSelection(ws_data) => {
                    let max_len = ws_data.word_length as usize;
                    let cur_len = ws_data.input.len();
                    let remaining = max_len - cur_len;

                    let masked_word = format!("{}{}", ws_data.input, "_".repeat(remaining));

                    let prompt = if cur_len != max_len {
                        "Write a Secret Word"
                    } else {
                        "Press ENTER to Confirm"
                    };

                    frame.render_widget(
                        Paragraph::new("Player One")
                            .style(home_option_style(ws_data.turn == Turn::PlayerOne)),
                        header_cols[0],
                    );
                    frame.render_widget(
                        Paragraph::new("Player Two")
                            .style(home_option_style(ws_data.turn == Turn::PlayerTwo)),
                        header_cols[2],
                    );
                    frame.render_widget(
                        Paragraph::new(prompt).alignment(Alignment::Center),
                        header_cols[1],
                    );

                    frame.render_widget(Paragraph::new(masked_word), chunks[0]);
                }
                RoundState::Guessing(g_data) => {
                    let masked_player_one_word = format!(
                        "Player One's Word: {}",
                        mask_word(&g_data.guessed_letters, &g_data.player_one_word)
                    );
                    let masked_player_two_word = format!(
                        "Player Two's Word: {}",
                        mask_word(&g_data.guessed_letters, &g_data.player_two_word)
                    );

                    frame.render_widget(
                        Paragraph::new("Player One")
                            .style(home_option_style(g_data.turn == Turn::PlayerOne)),
                        header_cols[0],
                    );
                    frame.render_widget(
                        Paragraph::new("Press a KEY to Guess").alignment(Alignment::Center),
                        header_cols[1],
                    );
                    frame.render_widget(
                        Paragraph::new("Player Two")
                            .style(home_option_style(g_data.turn == Turn::PlayerTwo)),
                        header_cols[2],
                    );

                    frame.render_widget(Paragraph::new(masked_player_one_word), chunks[1]);
                    frame.render_widget(Paragraph::new(masked_player_two_word), chunks[2]);
                }
                RoundState::Finished(result) => {
                    let message = match result {
                        crate::domain::RoundResult::Draw => "This Round's a Draw".to_string(),
                        crate::domain::RoundResult::Won(winner) => {
                            format!("{} Won the Round", winner.player.name)
                        }
                    };

                    frame.render_widget(Paragraph::new("Player One"), header_cols[0]);
                    frame.render_widget(Paragraph::new("Player Two"), header_cols[2]);

                    frame.render_widget(
                        Paragraph::new(message).alignment(Alignment::Center),
                        chunks[0],
                    );
                    frame.render_widget(
                        Paragraph::new("Press ENTER to Continue").alignment(Alignment::Center),
                        header_cols[1],
                    );
                }
            },
            MatchState::MatchFinised(_) => {}
        },
        _ => {}
    }

    frame.render_widget(Paragraph::new("Press ESC to Return Home"), footer_area);
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
        .collect()
}
