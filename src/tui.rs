use crate::audio::AudioMonitor;
use crate::knob::Knob;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};

pub fn draw_ui(frame: &mut Frame, knob: &Knob, audio_monitor: &AudioMonitor) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(3),
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

    let audio_text = if audio_monitor.is_active() {
        format!(
            "{} | Gain: {} | RMS {:.3} ({:.1} dBFS)",
            audio_monitor.status(),
            audio_monitor.gain(),
            input_rms,
            audio_monitor.dbfs()
        )
    } else {
        format!("Audio input unavailable | {}", audio_monitor.status())
    };

    let paragraph =
        Paragraph::new(audio_text).block(Block::default().title("Overview").borders(Borders::ALL));

    frame.render_widget(paragraph, areas[2]);
}
