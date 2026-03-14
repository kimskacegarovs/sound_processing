mod knob;

use knob::Knob;

use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    prelude::Frame,
    widgets::{Block, Borders, Gauge, Paragraph},
};

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn draw_ui(frame: &mut Frame, knob: &Knob) {
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

fn handle_input(knob: &mut Knob) -> io::Result<bool> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), _) => return Ok(false),
                    (KeyCode::Left, KeyModifiers::CONTROL) => knob.decrease_by(10),
                    (KeyCode::Right, KeyModifiers::CONTROL) => knob.increase_by(10),
                    (KeyCode::Left, _) => knob.decrease(),
                    (KeyCode::Right, _) => knob.increase(),
                    _ => {}
                }
            }
        }
    }

    Ok(true)
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut knob = Knob::new("TEST KNOB", 0);

    loop {
        terminal.draw(|frame| draw_ui(frame, &knob))?;

        if !handle_input(&mut knob)? {
            break;
        }
    }

    Ok(())
}
