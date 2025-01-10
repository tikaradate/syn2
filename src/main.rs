mod phoneme;

use phoneme::Phoneme;
use phoneme::generate_phoneme;
use phoneme::generate_square_phoneme;
use phoneme::generate_phoneme_with_harmonics;

use hound;

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
        // let waveform = generate_phoneme(
        //     phoneme.f1,
        //     phoneme.f2,
        //     pitch,
        //     duration,
        //     sample_rate,
        // );
        // write_wav_file(&("phonemes/".to_owned() + &symbol.to_owned() + "_phoneme.wav"), &waveform, sample_rate);

        // let waveform = generate_square_phoneme(
        //     phoneme.f1,
        //     phoneme.f2,
        //     pitch,
        //     duration,
        //     sample_rate,
        // );
        // write_wav_file(&("phonemes/".to_owned() + &symbol.to_owned() + "_square_phoneme.wav"), &waveform, sample_rate);

        let waveform = generate_phoneme_with_harmonics(
            phoneme.f1,
            phoneme.f2,
            pitch,
            duration,
            sample_rate,
            5,
        );
        write_wav_file(&("sounds/".to_owned() + &symbol.to_owned() + "_phoneme.wav"), &waveform, sample_rate);
    }
}
