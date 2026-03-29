use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::Paragraph,
};

use crate::app::{App, Screen};

/// Home banner art (UTF-8); must stay in sync with `home_banner.txt` line count.
const HOME_BANNER: &str = include_str!("home_banner.txt");
const HOME_BANNER_HEIGHT: u16 = 21;

pub fn render(app: &App, frame: &mut Frame) {
    match app.screen {
        Screen::Home => render_home(app, frame),
        Screen::Match(_) => render_match(frame),
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

fn render_match(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(frame.area());

    frame.render_widget(Paragraph::new("Match").style(Style::default()), chunks[0]);
    frame.render_widget(Paragraph::new("Press 'Esc' to return"), chunks[1]);
}
