use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
use crate::knob::Knob;

pub fn draw_ui(frame: &mut Frame, knob: &Knob) {
    let areas  = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(knob.name.as_str()))
        .percent(knob.value as u16);

    frame.render_widget(gauge, areas[0]);

    let text = format!("{}: {}", knob.name, knob.value);
    let paragraph =
        Paragraph::new(text).block(Block::default().title("Guitar FX").borders(Borders::ALL));

    frame.render_widget(paragraph, areas[1]);
}