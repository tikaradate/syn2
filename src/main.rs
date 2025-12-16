use std::env;
use syn2::read_raw_wav;
use syn2::write_wav_pcm16_mono;
use std::f32::consts::PI;

struct Resonator {
    frequency: f32,
    bandwidth: f32,
    sample_rate: f32,
    theta: f32,
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
        let theta = 2.0*PI * frequency / sample_rate;
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
            theta,
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
        
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;

        return y
    }
    fn set_formant(&mut self, frequency: f32, bandwidth: f32) {
        let r = (-PI * bandwidth / self.sample_rate).exp();
        self.theta= 2.0*PI * frequency / self.sample_rate;
        self.a1 = -2.0 * r * self.theta.cos();
        self.a2 = r*r;
        self.b0 = 1.0 - r;
        self.b1 = 0.0;
        self.b2 = 0.0;
    }
}

struct LFSource {
    sample_rate: f32,
    f0: f32,
    t0: f32,
    tp: f32,
    te: f32,
    ta: f32,
    omega_g: f32,
    alpha: f32,
    epsilon: f32,
    phase: f32,
}

impl LFSource {
    fn new(sample_rate: f32, f0: f32) -> Self {
        return LFSource {
            sample_rate,
            f0,
            t0: 0.0,
            tp: 0.0,
            te: 0.0,
            ta: 0.0,
            omega_g: 0.0,
            alpha: 0.0,
            epsilon: 0.0,
            phase: 0.0,
        }
    }
    fn next_sample(&mut self) -> f32 {
        self.t0 = 1.0/self.f0;
        self.tp = 0.4 * self.t0;
        self.te = 0.57 * self.t0;
        self.ta = 0.03 * self.t0;

        let t = self.phase * self.t0;
        let mut e = 0.0;
        if t < self.te {
            self.omega_g = PI / self.tp;
            self.alpha = -self.omega_g / (self.omega_g*self.te).tan();
            e = (self.alpha*t).exp() * (self.omega_g*t).sin();
        } else {
            self.epsilon = 1.0 / self.ta;
            e = -(- self.epsilon * (t - self.te)).exp();
        }

        self.phase += 1.0 / (self.sample_rate * self.t0);
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        return e
    }
}

fn main() {
    let sample_rate = 44100.0;   
    let duration = 1.0;
    let f0 = 100.0;
    let period = (sample_rate/f0) as u32;
    let num_samples: u32 = (sample_rate*duration) as u32;
    let mut lf = LFSource::new(sample_rate, f0);
    let mut samples_f32: Vec<f32> = Vec::new();
    let mut r1 = Resonator::new(700.0, 100.0, sample_rate);
    let mut r2 = Resonator::new(1200.0, 100.0, sample_rate);
    for i in 0..num_samples {
        let progress = i as f32 / num_samples as f32;  // 0.0 → 1.0
    
        let f1 = 400.0 + (270.0 - 400.0) * progress;
        let f2 = 800.0 + (2300.0 - 800.0) * progress;
        
        r1.set_formant(f1, 100.0);
        r2.set_formant(f2, 100.0);
        
        let x = lf.next_sample();
        let y = r2.process(r1.process(x));

        samples_f32.push(y);
    }

    let max = samples_f32.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
    let samples: Vec<i16> = samples_f32
        .iter()
        .map(|&s| ((s / max) * 32767.0) as i16)
        .collect();

    write_wav_pcm16_mono("./sounds/oi.wav", &samples, sample_rate as u32);
}