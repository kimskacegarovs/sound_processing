mod keyboard_input;
mod knob;
mod tui;

use keyboard_input::handle_input;
use knob::Knob;
use tui::draw_ui;

use std::io;

fn main() -> io::Result<()> {
    let result = run();
    ratatui::restore();
    result
}

fn run() -> io::Result<()> {
    let mut knob = Knob::new("TEST KNOB", 0);
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| draw_ui(frame, &knob))?;

        if !handle_input(&mut knob)? {
            break;
        }
    }

    Ok(())
}
