mod audio;
mod keyboard_input;
mod knob;
mod tui;

use audio::AudioMonitor;
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
    let audio_monitor = AudioMonitor::start_default_input();
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| draw_ui(frame, &knob, &audio_monitor))?;

        if !handle_input(&mut knob)? {
            break;
        }
    }

    Ok(())
}
