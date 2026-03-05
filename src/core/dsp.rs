use num_complex::Complex;
use realfft::RealFftPlanner;
use rtrb::Consumer;
use std::sync::Arc;

pub const FFT_SIZE: usize = 2048;

pub struct AnalyzerDsp {
    pub fft_plan: Arc<dyn realfft::RealToComplex<f32>>,
    pub in_buffer: Vec<f32>,
    pub out_buffer: Vec<Complex<f32>>,
    pub spectrum: Vec<f32>,
    pub audio_history: Vec<f32>,
}

impl Default for AnalyzerDsp {
    fn default() -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft_plan = planner.plan_fft_forward(FFT_SIZE);
        Self {
            in_buffer: fft_plan.make_input_vec(),
            out_buffer: fft_plan.make_output_vec(),
            fft_plan,
            spectrum: vec![-100.0; FFT_SIZE / 2],
            audio_history: Vec::with_capacity(FFT_SIZE * 2),
        }
    }
}

impl AnalyzerDsp {
    pub fn process_consumer(&mut self, cons: &mut Consumer<f32>) {
        let mut new_samples = false;
        
        while let Ok(s) = cons.pop() {
            self.audio_history.push(s);
            new_samples = true;
        }
        
        if self.audio_history.len() > FFT_SIZE {
            let excess = self.audio_history.len() - FFT_SIZE;
            self.audio_history.drain(0..excess);
        }

        if new_samples && self.audio_history.len() == FFT_SIZE {
            self.in_buffer.copy_from_slice(&self.audio_history);
            
            for (i, v) in self.in_buffer.iter_mut().enumerate() {
                let x = 2.0 * std::f32::consts::PI * i as f32 / (FFT_SIZE - 1) as f32;
                let window = 0.35875 - 0.48829 * x.cos() + 0.14128 * (2.0 * x).cos() - 0.01168 * (3.0 * x).cos();
                *v *= window;
            }
            
            let _ = self.fft_plan.process(&mut self.in_buffer, &mut self.out_buffer);
            
            let sample_rate = 48000.0;
            for (i, c) in self.out_buffer.iter().take(FFT_SIZE / 2).enumerate() {
                let mag = c.norm() / (FFT_SIZE as f32 / 2.0);
                let mut db = 20.0 * mag.log10().max(-100.0);
                
                let freq = i as f32 * sample_rate / FFT_SIZE as f32;
                if freq > 0.0 {
                    let tilt = 3.0 * (freq / 1000.0).log2();
                    db += tilt;
                }
                
                db = db.clamp(-100.0, 20.0);
                
                let prev = self.spectrum[i];
                self.spectrum[i] = prev * 0.85 + db * 0.15;
            }
        }
    }
}
