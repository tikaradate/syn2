use std::f32::consts::PI;

pub fn generate_sine_wave(frequency: f32, duration: f32, sample_rate: u32) -> Vec<f32> {
    let sample_count = (duration * sample_rate as f32) as usize;
    (0..sample_count)
        .map(|i| {
            let t = i as f32 / sample_rate as f32;
            (2.0 * PI * frequency * t).sin()
        })
        .collect()
}
 
pub fn generate_square_wave(frequency: f32, duration: f32, sample_rate: u32) -> Vec<f32> {
    generate_sine_wave(frequency, duration, sample_rate)
        .into_iter()
        .map(|sample| if sample > 0.0 {0.33} else {-0.33})
        .collect()
}

pub fn generate_harmonics(
    frequency: f32,    
    duration: f32,    
    sample_rate: u32,  
    num_harmonics: usize,
) -> Vec<f32> {
    let mut result_wave = vec![0.0; (sample_rate as f32 * duration) as usize];

    for n in 1..=num_harmonics {
        let harmonic_freq = frequency * n as f32;
        let harmonic_wave = generate_sine_wave(harmonic_freq, duration, sample_rate);

        let amplitude = 1.0 / n as f32;

        for (i, sample) in result_wave.iter_mut().enumerate() {
            *sample += harmonic_wave[i] * amplitude;
        }
    }

    let max_amplitude = result_wave.iter().cloned().fold(0.0, f32::max);
    if max_amplitude > 1.0 {
        result_wave.iter_mut().for_each(|sample| *sample /= max_amplitude);
    }

    result_wave
}

