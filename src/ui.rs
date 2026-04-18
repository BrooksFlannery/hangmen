use std::collections::HashSet;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::Paragraph,
};

use crate::{
    app::{App, Screen},
    domain::{MatchState, RoundState, Turn},
};

/// Home banner art (UTF-8); must stay in sync with `home_banner.txt` line count.
const HOME_BANNER: &str = include_str!("home_banner.txt");
const HOME_BANNER_HEIGHT: u16 = 21;
const HOME_BANNER_WIDTH: u16 = 58;

pub fn render(app: &App, frame: &mut Frame) {
    match app.screen {
        Screen::Home => render_home(app, frame),
        Screen::Match(_) => render_match(app, frame),
    }
}

fn render_keyboard_ascii(guessed: &HashSet<char>) -> String {
    const INNER_WIDTH: usize = 21;

    fn row_line(letters: &str, indent: usize, guessed: &HashSet<char>) -> String {
        const SEP: char = ' ';
        let mut out = String::new();

        out.push_str(&" ".repeat(indent));
        for (i, c) in letters.chars().enumerate() {
            if i > 0 {
                out.push(SEP);
            }
            if guessed.contains(&c) {
                out.push(' ');
            } else {
                out.push(c);
            }
        }

        out
    }

    let mut lines = Vec::with_capacity(5);
    lines.push("┌─────────────────────┐".to_string());

    for (letters, indent) in [("qwertyuiop", 1usize), ("asdfghjkl", 2usize), ("zxcvbnm", 3usize)]
    {
        let inner = row_line(letters, indent, guessed);
        let pad = INNER_WIDTH.saturating_sub(inner.len());
        lines.push(format!("│{}{}│", inner, " ".repeat(pad)));
    }

    lines.push("└─────────────────────┘".to_string());
    lines.join("\n")
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
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let main_area = outer[0];
    let footer_area = outer[1];
    let banner = HOME_BANNER.trim_end_matches(&['\n', '\r'][..]);

    let main_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(HOME_BANNER_WIDTH),
            Constraint::Fill(1),
        ])
        .split(main_area);

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
        .split(main_cols[1]);

    frame.render_widget(Paragraph::new(banner).style(Style::new()), chunks[0]);
    frame.render_widget(
        Paragraph::new("Player vs Player")
            .style(home_option_style(app.home_selected == 0))
            .alignment(Alignment::Center),
        chunks[2],
    );
    frame.render_widget(
        Paragraph::new("Player vs Bot")
            .style(home_option_style(app.home_selected == 1))
            .alignment(Alignment::Center),
        chunks[3],
    );
    frame.render_widget(
        Paragraph::new("Bot vs Bot")
            .style(home_option_style(app.home_selected == 2))
            .alignment(Alignment::Center),
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

    let main_area = outer[1];
    let footer_area = outer[2];

    let main_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(24),
            Constraint::Fill(1),
        ])
        .split(main_area);

    match &app.screen {
        Screen::Match(m_state) => match m_state {
            MatchState::MatchInProgress(m_data) => match &m_data.current_round {
                RoundState::WordSelection(ws_data) => {
                    let rows = [
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ];
                    let left_col = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(rows)
                        .split(main_cols[0]);
                    let center_col = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(rows)
                        .split(main_cols[1]);
                    let right_col = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(rows)
                        .split(main_cols[2]);

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
                            .style(home_option_style(ws_data.turn == Turn::PlayerOne))
                            .alignment(Alignment::Center),
                        left_col[0],
                    );
                    frame.render_widget(
                        Paragraph::new(prompt).alignment(Alignment::Center),
                        center_col[0],
                    );
                    frame.render_widget(
                        Paragraph::new("Player Two")
                            .style(home_option_style(ws_data.turn == Turn::PlayerTwo))
                            .alignment(Alignment::Center),
                        right_col[0],
                    );

                    frame.render_widget(
                        Paragraph::new(masked_word).alignment(Alignment::Center),
                        center_col[1],
                    );
                }
                RoundState::Guessing(g_data) => {
                    let rows = [
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(5),
                        Constraint::Min(0),
                    ];
                    let left_col = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(rows)
                        .split(main_cols[0]);
                    let center_col = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(rows)
                        .split(main_cols[1]);
                    let right_col = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(rows)
                        .split(main_cols[2]);

                    let masked_player_one_word =
                        mask_word(&g_data.guessed_letters, &g_data.player_one_word);

                    let masked_player_two_word =
                        mask_word(&g_data.guessed_letters, &g_data.player_two_word);

                    frame.render_widget(
                        Paragraph::new("Player One")
                            .style(home_option_style(g_data.turn == Turn::PlayerOne))
                            .alignment(Alignment::Center),
                        left_col[0],
                    );
                    frame.render_widget(
                        Paragraph::new("Press a KEY to Guess").alignment(Alignment::Center),
                        center_col[1],
                    );
                    frame.render_widget(
                        Paragraph::new("Player Two")
                            .style(home_option_style(g_data.turn == Turn::PlayerTwo))
                            .alignment(Alignment::Center),
                        right_col[0],
                    );

                    frame.render_widget(
                        Paragraph::new(masked_player_one_word).alignment(Alignment::Center),
                        left_col[2],
                    );
                    frame.render_widget(
                        Paragraph::new(masked_player_two_word).alignment(Alignment::Center),
                        right_col[2],
                    );

                    let keyboard = render_keyboard_ascii(&g_data.guessed_letters);
                    frame.render_widget(
                        Paragraph::new(keyboard).alignment(Alignment::Center),
                        center_col[3],
                    );
                }
                RoundState::Finished(result) => {
                    let rows = [
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ];
                    let left_col = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(rows)
                        .split(main_cols[0]);
                    let center_col = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(rows)
                        .split(main_cols[1]);
                    let right_col = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(rows)
                        .split(main_cols[2]);

                    let message = match result {
                        crate::domain::RoundResult::Draw => "This Round's a Draw".to_string(),
                        crate::domain::RoundResult::Won(winner) => {
                            format!("{} Won the Round", winner.player.name)
                        }
                    };

                    frame.render_widget(
                        Paragraph::new("Player One").alignment(Alignment::Center),
                        left_col[0],
                    );
                    frame.render_widget(
                        Paragraph::new("Press ENTER to Continue").alignment(Alignment::Center),
                        center_col[0],
                    );
                    frame.render_widget(
                        Paragraph::new("Player Two").alignment(Alignment::Center),
                        right_col[0],
                    );
                    frame.render_widget(
                        Paragraph::new(message).alignment(Alignment::Center),
                        center_col[1],
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
