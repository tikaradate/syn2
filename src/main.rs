use std::env;
use syn2::read_raw_wav;
use syn2::write_wav_pcm16_mono;
use std::f32::consts::PI;

struct Resonator {
    frequency: f32,
    bandwidth: f32,
    sample_rate: f32,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl Resonator {
    fn new(frequency: f32, bandwidth: f32, sample_rate: f32) -> Self {
        // Calculate R and θ
        // R = exp(-π × bandwidth / sample_rate)
        let r = (-PI * bandwidth / sample_rate).exp();
        // θ = 2π × frequency / sample_rate
        let theta= 2.0*PI * frequency / sample_rate;
        // Calculate coefficients
        // a1 = -2 × R × cos(θ)
        let a1 = -2.0 * r * theta.cos();
        // a2 = R²
        let a2 = r*r;
        // b0 = 1 - R
        let b0 = 1.0 - r;
        // b1 = 0
        let b1 = 0.0;
        // b2 = 0
        let b2 = 0.0;
        
        return Resonator {
            frequency,
            bandwidth,
            sample_rate,
            a1,
            a2,
            b0,
            b1,
            b2,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
    fn process(&mut self, x: f32) -> f32 {
        // Apply difference equation:
        // y = b0*x + b1*x1 + b2*x2 - a1*y1 - a2*y2
        let y = self.b0 * x + self.b1*self.x1 + self.b2*self.x2 - self.a1*self.y1 - self.a2*self.y2;
        
        // Shift state:
        // x2 = x1
        // x1 = x
        // y2 = y1
        // y1 = y
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;

        return y
    }
}



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

    // saw
    // let sample_rate = 44100.0;
    // let duration = 1.0;
    // let f0 = 100.0;
    // let period = (sample_rate/f0) as u32;
    // let num_samples: u32 = (sample_rate*duration) as u32;
    // let mut samples = Vec::<i16>::new();
    // for i in 0..num_samples {
    //     let value = (i % period) as f32 / period as f32; 
    //     samples.push(((2.0 * value - 1.0)* 32767.0) as i16);
    // }
    // write_wav_pcm16_mono("./sounds/saw.wav", &samples, sample_rate as u32);

    let sample_rate = 44100.0;
    let duration = 1.0;
    let f0 = 100.0;
    let period = (sample_rate/f0) as u32;
    let num_samples: u32 = (sample_rate*duration) as u32;
    let mut samples = Vec::<i16>::new();
    for i in 0..num_samples {
        let value = (i % period) as f32 / period as f32; 
        samples.push(((2.0 * value - 1.0)* 32767.0) as i16);
    }



    let mut r1 = Resonator::new(700.0, 130.0, 44100.0);   // F1
    let mut r2 = Resonator::new(1200.0, 70.0, 44100.0);   // F2
    let mut r3 = Resonator::new(2600.0, 160.0, 44100.0);  // F3

    let output: Vec<i16> = samples
        .iter()
        .map(|&s| {
            let x = s as f32 / 32767.0;  // back to -1..1
            let y = r3.process(r2.process(r1.process(x)));
            (y * 32767.0).clamp(-32767.0, 32767.0) as i16
        })
        .collect();
    
    write_wav_pcm16_mono("./sounds/a.wav", &output, sample_rate as u32);

}