use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use crate::knob::Knob;

pub struct AudioMonitor {
    rms_bits: Arc<AtomicU32>,
    gain_bits: Arc<AtomicU32>,
    status: String,
    active: bool,
    _stream: Option<Stream>,
}

impl AudioMonitor {
    pub fn start_default_input(gain_knob: &Knob) -> Self {
        let rms_bits = Arc::new(AtomicU32::new(0.0f32.to_bits()));
        let gain_bits = Arc::new(AtomicU32::new(gain_knob.value as u32));

        match try_start_default_input(rms_bits.clone(), gain_bits.clone()) {
            Ok((stream, status)) => Self {
                rms_bits,
                gain_bits,
                status,
                active: true,
                _stream: Some(stream),
            },
            Err(error) => Self {
                rms_bits,
                gain_bits,
                status: error.to_string(),
                active: false,
                _stream: None,
            },
        }
    }

    pub fn set_gain_from_knob(&self, knob: &Knob) {
        self.gain_bits.store(knob.value as u32, Ordering::Relaxed);
    }

    pub fn rms(&self) -> f32 {
        f32::from_bits(self.rms_bits.load(Ordering::Relaxed)).clamp(0.0, 1.0)
    }

    pub fn dbfs(&self) -> f32 {
        20.0 * self.rms().max(1.0e-6).log10()
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

fn try_start_default_input(
    rms_bits: Arc<AtomicU32>,
    gain_bits: Arc<AtomicU32>,
) -> io::Result<(Stream, String)> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No default audio input device found"))?;

    let device_name = device
        .name()
        .unwrap_or_else(|_| "Unknown input device".to_string());

    let supported_config = device.default_input_config().map_err(io::Error::other)?;
    let sample_format = supported_config.sample_format();
    let stream_config: cpal::StreamConfig = supported_config.into();

    let stream = build_stream_for_format(
        &device,
        &stream_config,
        sample_format,
        rms_bits,
    ).map_err(io::Error::other)?;

    stream.play().map_err(io::Error::other)?;

    let status = format!(
        "Listening on {device_name} ({} ch @ {} Hz, {:?}) || Gain bits: {}",
        stream_config.channels,
        stream_config.sample_rate.0,
        sample_format,
        gain_bits.load(Ordering::Relaxed),
    );

    Ok((stream, status))
}

fn build_stream_for_format(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    sample_format: SampleFormat,
    rms_bits: Arc<AtomicU32>,
) -> Result<Stream, cpal::BuildStreamError> {
    match sample_format {
        SampleFormat::F32 => build_input_stream::<f32, _>(
            device,
            config,
            rms_bits,
            |sample| sample,
        ),
        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
    }
}

fn build_input_stream<T, F>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    rms_bits: Arc<AtomicU32>,
    convert_sample: F,
) -> Result<Stream, cpal::BuildStreamError>
where
    T: cpal::SizedSample + Copy + Send + 'static,
    F: Fn(T) -> f32 + Send + 'static,
{
    let channels = usize::from(config.channels.max(1));

    device.build_input_stream(
        config,
        move |data: &[T], _| update_rms(data, channels, &rms_bits, &convert_sample),
        |error| eprintln!("Audio input stream error: {error}"),
        None,
    )
}

fn update_rms<T, F>(
    data: &[T],
    channels: usize,
    rms_bits: &Arc<AtomicU32>,
    convert_sample: &F,
) where
    T: Copy,
    F: Fn(T) -> f32,
{
    if data.is_empty() {
        return;
    }

    let mut sum_squares = 0.0f32;
    let mut frame_count = 0usize;

    for frame in data.chunks(channels) {
        if frame.is_empty() {
            continue;
        }

        let mono_sample = frame
            .iter()
            .copied()
            .map(convert_sample)
            .sum::<f32>()
            / frame.len() as f32;

        sum_squares += mono_sample * mono_sample;
        frame_count += 1;
    }

    if frame_count == 0 {
        return;
    }

    let rms = (sum_squares / frame_count as f32).sqrt().clamp(0.0, 1.0);
    let previous = f32::from_bits(rms_bits.load(Ordering::Relaxed));
    let smoothed = if rms > previous {
        rms
    } else {
        previous * 0.9 + rms * 0.1
    };

    rms_bits.store(smoothed.to_bits(), Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_rms_for_mono_input() {
        let rms_bits = Arc::new(AtomicU32::new(0.0f32.to_bits()));
        let data = [1.0f32, -1.0, 1.0, -1.0];

        update_rms(&data, 1, &rms_bits, &|sample| sample);

        let rms = f32::from_bits(rms_bits.load(Ordering::Relaxed));
        assert!((rms - 1.0).abs() < 1.0e-6);
    }

    #[test]
    fn mixes_channels_before_calculating_rms() {
        let rms_bits = Arc::new(AtomicU32::new(0.0f32.to_bits()));
        let stereo_data = [1.0f32, 0.0, 1.0, 0.0];

        update_rms(&stereo_data, 2, &rms_bits, &|sample| sample);

        let rms = f32::from_bits(rms_bits.load(Ordering::Relaxed));
        assert!((rms - 0.5).abs() < 1.0e-6);
    }

    #[test]
    fn applies_smoothing_when_signal_drops() {
        let rms_bits = Arc::new(AtomicU32::new(1.0f32.to_bits()));
        let silent_data = [0.0f32; 8];

        update_rms(&silent_data, 1, &rms_bits, &|sample| sample);

        let rms = f32::from_bits(rms_bits.load(Ordering::Relaxed));
        assert!((rms - 0.9).abs() < 1.0e-6);
    }
}


