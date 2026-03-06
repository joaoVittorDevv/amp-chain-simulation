use arc_swap::ArcSwapOption;
use realfft::num_complex::Complex;
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};
use std::sync::Arc;

pub const BLOCK_SIZE: usize = 64;
pub const MAX_IR_LEN: usize = 4096;
pub const NUM_PARTITIONS: usize = MAX_IR_LEN / BLOCK_SIZE;
pub const FFT_SIZE: usize = BLOCK_SIZE * 2;

pub struct IrData {
    pub partitions: Vec<Vec<Complex<f32>>>,
    pub name: String,
}

impl IrData {
    pub fn new(samples: &[f32], name: String) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(FFT_SIZE);

        let mut partitions = Vec::new();
        
        let padded_len = samples.len().min(MAX_IR_LEN);
        let num_parts = (padded_len + BLOCK_SIZE - 1) / BLOCK_SIZE;

        for p in 0..num_parts {
            let start = p * BLOCK_SIZE;
            let end = (start + BLOCK_SIZE).min(padded_len);
            let mut time_block = vec![0.0; FFT_SIZE];
            
            // IR partitions are zero-padded to FFT_SIZE
            // We copy BLOCK_SIZE samples
            let chunk_len = end - start;
            let mut chunk = samples[start..end].to_vec();
            
            // Apply a very quick fade-out on the last chunk
            if p == num_parts - 1 && chunk_len > 0 {
                let fade_len = chunk_len.min(64);
                for i in 0..fade_len {
                    let factor = 1.0 - (i as f32 / fade_len as f32);
                    chunk[chunk_len - fade_len + i] *= factor;
                }
            }

            time_block[..chunk_len].copy_from_slice(&chunk);

            // Forward FFT
            let mut freq_block = fft.make_output_vec();
            fft.process(&mut time_block, &mut freq_block).unwrap();
            partitions.push(freq_block);
        }

        Self { partitions, name }
    }
}

pub struct IrConvolver {
    fft: Arc<dyn RealToComplex<f32>>,
    ifft: Arc<dyn ComplexToReal<f32>>,
    current_ir: Arc<ArcSwapOption<IrData>>,
    
    time_buf: [f32; FFT_SIZE],
    freq_history: Vec<Vec<Complex<f32>>>,
    freq_history_idx: usize,
    
    input_buffer: [f32; BLOCK_SIZE],
    output_buffer: [f32; BLOCK_SIZE],
    in_idx: usize,
    out_idx: usize,
}

impl IrConvolver {
    pub fn new(shared_ir: Arc<ArcSwapOption<IrData>>) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        Self {
            fft: planner.plan_fft_forward(FFT_SIZE),
            ifft: planner.plan_fft_inverse(FFT_SIZE),
            current_ir: shared_ir,
            time_buf: [0.0; FFT_SIZE],
            freq_history: vec![vec![Complex::new(0.0, 0.0); FFT_SIZE / 2 + 1]; NUM_PARTITIONS],
            freq_history_idx: 0,
            input_buffer: [0.0; BLOCK_SIZE],
            output_buffer: [0.0; BLOCK_SIZE],
            in_idx: 0,
            out_idx: BLOCK_SIZE, // forces initial block processing on first sample
        }
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        if self.out_idx >= BLOCK_SIZE {
            self.process_block();
            self.in_idx = 0;
            self.out_idx = 0;
        }

        self.input_buffer[self.in_idx] = sample;
        let out = self.output_buffer[self.out_idx];
        
        self.in_idx += 1;
        self.out_idx += 1;
        
        out
    }

    fn process_block(&mut self) {
        // Shift time_buf left by BLOCK_SIZE
        self.time_buf.copy_within(BLOCK_SIZE..FFT_SIZE, 0);
        // Copy new input block into the right half of time_buf
        self.time_buf[BLOCK_SIZE..FFT_SIZE].copy_from_slice(&self.input_buffer);

        if let Some(ir_guard) = self.current_ir.load_full() {
            // Forward FFT on time_buf
            let mut time_scratch = self.time_buf; // copy since fft alters input
            let mut freq_block = self.fft.make_output_vec();
            self.fft.process(&mut time_scratch, &mut freq_block).unwrap();

            // Store in circular history
            self.freq_history[self.freq_history_idx] = freq_block;

            // Multiply and accumulate
            let mut freq_accum = self.fft.make_output_vec();
            let p_len = ir_guard.partitions.len().min(NUM_PARTITIONS);
            for p in 0..p_len {
                let hist_idx = (self.freq_history_idx + NUM_PARTITIONS - p) % NUM_PARTITIONS;
                
                let h_part = &self.freq_history[hist_idx];
                let ir_part = &ir_guard.partitions[p];
                
                for (a, (&hp, &irp)) in freq_accum.iter_mut().zip(h_part.iter().zip(ir_part.iter())) {
                    *a += hp * irp;
                }
            }

            // Inverse FFT
            let mut time_out = [0.0; FFT_SIZE];
            self.ifft.process(&mut freq_accum, &mut time_out).unwrap();

            // The realfft IFFT is unscaled
            let scale = 1.0 / FFT_SIZE as f32;
            for i in BLOCK_SIZE..FFT_SIZE {
                self.output_buffer[i - BLOCK_SIZE] = time_out[i] * scale;
            }

            // Move history index forward
            self.freq_history_idx = (self.freq_history_idx + 1) % NUM_PARTITIONS;
        } else {
            // Bypass if no IR loaded
            self.output_buffer.copy_from_slice(&self.input_buffer);
        }
    }
}
