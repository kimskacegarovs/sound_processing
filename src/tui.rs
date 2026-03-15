use crate::audio::AudioMonitor;
use crate::knob::Knob;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};

pub fn draw_ui(frame: &mut Frame, knob: &Knob, audio_monitor: &AudioMonitor) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // gain
            Constraint::Length(3), // rms
            Constraint::Length(3), // pitch
            Constraint::Min(3), // overview
        ])
        .split(frame.area());

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(knob.name.as_str()),
        )
        .label(format!("{}%", knob.value))
        .percent(knob.value as u16);

    frame.render_widget(gauge, areas[0]);

    let input_rms = audio_monitor.rms();
    let input_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Input RMS"))
        .percent((input_rms * 100.0).round() as u16);

    frame.render_widget(input_gauge, areas[1]);

    let pitch_text = match audio_monitor.pitch_info() {
        Some(p) => format!(
            "Note: {} | Frequency: {:.1} Hz | Detune: {:+.1} cents",
            p.note, p.frequency_hz, p.cents_off
        ),
        None => "Note: --\nFrequency: --\nDetune: --".to_string(),
    };

    let pitch_panel =
        Paragraph::new(pitch_text).block(Block::default().title("Pitch").borders(Borders::ALL));

    frame.render_widget(pitch_panel, areas[2]);


    let audio_text = if audio_monitor.is_active() {
        format!(
            "{} | Gain: {} | RMS {:.3} ({:.1} dBFS)",
            audio_monitor.status(),
            audio_monitor.gain(),
            input_rms,
            audio_monitor.dbfs(),
        )
    } else {
        format!("Audio input unavailable | {}", audio_monitor.status())
    };

    let overview =
        Paragraph::new(audio_text).block(Block::default().title("Overview").borders(Borders::ALL));

    frame.render_widget(overview, areas[3]);
}
