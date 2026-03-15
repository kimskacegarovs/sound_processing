mod audio;
mod keyboard_input;
mod knob;
mod tui;
mod pitch;

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
    let mut gain_knob = Knob::new("Gain", 50);
    let audio_monitor = AudioMonitor::start_default_input(&gain_knob);
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| draw_ui(frame, &gain_knob, &audio_monitor))?;
        let keep_running = handle_input(&mut gain_knob)?;
        audio_monitor.set_gain_from_knob(&gain_knob);

        if !keep_running {
            break;
        }
    }

    Ok(())
}
