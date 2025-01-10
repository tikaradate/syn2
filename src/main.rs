use hound;
use std::f32::consts::PI;

struct Phoneme {
    f1: f32,
    f2: f32,
}

fn generate_sine_wave(frequency: f32, duration: f32, sample_rate: u32) -> Vec<f32> {
    let sample_count = (duration * sample_rate as f32) as usize;
    (0..sample_count)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            (2.0 * PI * frequency * t).sin()
        })
        .collect()
}
 
fn generate_phoneme(
    formant1: f32,
    formant2: f32,
    pitch: f32,
    duration: f32,
    sample_rate: u32,
) -> Vec<f32> {
    let wave1 = generate_sine_wave(formant1, duration, sample_rate);
    let wave2 = generate_sine_wave(formant2, duration, sample_rate);
    let pitch_wave = generate_sine_wave(pitch, duration, sample_rate);

    wave1
        .iter()
        .zip(wave2.iter())
        .zip(pitch_wave.iter())
        .map(|((&w1, &w2), &p)| (w1 + w2 + p) / 3.0)
        .collect()
}

fn write_wav_file(filename: &str, samples: &[f32], sample_rate: u32) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    for &sample in samples {
        let amplitude = (sample * i16::MAX as f32) as i16;
        writer.write_sample(amplitude).unwrap();
    }
}

fn main() {
    let sample_rate = 44100;
    let duration = 1.0;
    let pitch = 440.0;
    let phoneme_map = vec![
        ("a", Phoneme { f1: 850.0, f2: 1300.0 }),
        ("i", Phoneme { f1: 415.0, f2: 2700.0 }),
        ("u", Phoneme { f1: 570.0, f2: 1430.0 }),
        ("e", Phoneme { f1: 670.0, f2: 2275.0 }),
        ("o", Phoneme { f1: 625.0, f2: 1090.0 }),
    ];

    for (symbol, phoneme) in phoneme_map{
        let waveform = generate_phoneme(
            phoneme.f1,
            phoneme.f2,
            pitch,
            duration,
            sample_rate,
        );
        write_wav_file(&("phonemes/".to_owned() + &symbol.to_owned() + "_phoneme.wav"), &waveform, sample_rate);
    }
}
