#[derive(Clone, Debug)]
pub struct PitchInfo {
    pub frequency_hz: f32,
    pub note: String,
    pub cents_off: f32,
}

impl PitchInfo {
    pub fn from_frequency(frequency_hz: f32) -> PitchInfo {
        if frequency_hz <= 0.0 {
            return PitchInfo {
                frequency_hz,
                note: "--".to_string(),
                cents_off: 0.0,
            };
        }

        let midi = frequency_to_midi(frequency_hz);
        let midi_nearest = midi.round() as i32;
        let cents_off = (midi - midi_nearest as f32) * 100.0;

        let note_name = midi_to_note_name(midi_nearest).to_string();
        let octave = midi_to_octave(midi_nearest);
        let note = format!("{}{}", note_name, octave);

        PitchInfo {
            frequency_hz,
            note,
            cents_off,
        }
    }
}

const A4_FREQ: f32 = 440.0;
const A4_MIDI: f32 = 69.0;

fn frequency_to_midi(frequency: f32) -> f32 {
    A4_MIDI + 12.0 * (frequency / A4_FREQ).log2()
}

fn midi_to_note_name(midi: i32) -> &'static str {
    match midi.rem_euclid(12) {
        0 => "C",
        1 => "C#",
        2 => "D",
        3 => "D#",
        4 => "E",
        5 => "F",
        6 => "F#",
        7 => "G",
        8 => "G#",
        9 => "A",
        10 => "A#",
        11 => "B",
        _ => "?",
    }
}

fn midi_to_octave(midi: i32) -> i32 {
    midi / 12 - 1
}
