use std::io;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crate::knob::Knob;

pub fn handle_input(knob: &mut Knob) -> io::Result<bool> {
    if event::poll(Duration::from_millis(16))? {
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
