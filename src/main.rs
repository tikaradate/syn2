use std::env;
use syn2::read_raw_wav;
use syn2::write_wav_pcm16_mono;



fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("usage: wavtool <path.wav>");
    //     return;
    // }
    // let file_path = &args[1];

    // match read_raw_wav(file_path) {
    //     Ok(wav) => {
    //         println!("fmt: {:?}", wav.format());
    //         println!("data bytes: {}", wav.data_bytes().len())
    //     }
    //     Err(e) => eprintln!("error: {e}"),
    // }
   
    // sine wave
    // let sample_rate = 44100.0;
    // let duration = 1.0;
    // let freq = 440.0;
    // let num_samples: u32 = (sample_rate*duration) as u32;
    // let mut samples = Vec::<i16>::new();
    // for i in 0..num_samples {
    //     let time = i as f32 / sample_rate;
    //     let value = (2.0 * std::f32::consts::PI  * freq * time).sin();
    //     samples.push((value * 32767.0) as i16);
    // }
    // write_wav_pcm16_mono("./sounds/sine.wav", &samples, sample_rate as u32);

    // glottal
    // let sample_rate = 44100.0;
    // let duration = 1.0;
    // let f0 = 100.0;
    // let period = (sample_rate/f0) as u32;
    // let num_samples: u32 = (sample_rate*duration) as u32;
    // let mut samples = Vec::<i16>::new();
    // for i in 0..num_samples {
    //     let mut value = 0.0;
    //     if i % period == 0 {
    //         value = 1.0;
    //     }
    //     samples.push((value * 32767.0) as i16);
    // }
    // write_wav_pcm16_mono("./sounds/glottal.wav", &samples, sample_rate as u32);
}
