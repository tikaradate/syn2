mod wave;
use wave::generate_sine_wave;
use wave::generate_square_wave;
use wave::generate_harmonics;

pub struct Phoneme {
    pub f1: f32,
    pub f2: f32,
}

pub fn generate_phoneme_wave<F>(
    formant1: f32,
    formant2: f32,
    pitch: f32,
    duration: f32,
    sample_rate: u32,
    wave_generator: F,
) -> Vec<f32>
where
    F: Fn(f32, f32, u32) -> Vec<f32>,
{
    let wave1 = wave_generator(formant1, duration, sample_rate);
    let wave2 = wave_generator(formant2, duration, sample_rate);
    let pitch_wave = wave_generator(pitch, duration, sample_rate);

    wave1
        .iter()
        .zip(wave2.iter())
        .zip(pitch_wave.iter())
        .map(|((&w1, &w2), &p)| (w1 + w2 + p) / 3.0)
        .collect()
}

pub fn generate_phoneme(
    formant1: f32,
    formant2: f32,
    pitch: f32,
    duration: f32,
    sample_rate: u32,
) -> Vec<f32> {
    generate_phoneme_wave(formant1, formant2, pitch, duration, sample_rate, generate_sine_wave)
}

pub fn generate_square_phoneme(
    formant1: f32,
    formant2: f32,
    pitch: f32,
    duration: f32,
    sample_rate: u32,
) -> Vec<f32> {
    generate_phoneme_wave(formant1, formant2, pitch, duration, sample_rate, generate_square_wave)
}

pub fn generate_phoneme_with_harmonics(
    formant1: f32,
    formant2: f32,
    pitch: f32,
    duration: f32,
    sample_rate: u32,
    num_harmonics: usize,
) -> Vec<f32> {
    let formant1_wave = generate_harmonics(formant1, duration, sample_rate, num_harmonics);
    let formant2_wave = generate_harmonics(formant2, duration, sample_rate, num_harmonics);
    let pitch_wave = generate_harmonics(pitch, duration, sample_rate, num_harmonics);

    formant1_wave
        .iter()
        .zip(formant2_wave.iter())
        .zip(pitch_wave.iter())
        .map(|((&f1, &f2), &p)| (f1 + f2 + p) / 3.0)
        .collect()
}
