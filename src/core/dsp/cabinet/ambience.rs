pub struct RoomAmbience {
    delay_buffer: Vec<f32>,
    write_idx: usize,
    sample_rate: f32,
    delay_ms: f32,
    mix: f32,
}

impl Default for RoomAmbience {
    fn default() -> Self {
        Self::new()
    }
}

impl RoomAmbience {
    pub fn new() -> Self {
        Self {
            delay_buffer: Vec::new(),
            write_idx: 0,
            sample_rate: 44100.0,
            delay_ms: 10.0,
            mix: 0.0,
        }
    }

    /// Pre-allocates memory for the absolute worst-case scenario.
    /// This strictly avoids allocations in the audio thread.
    pub fn initialize(&mut self, max_sample_rate: f32) {
        // e.g., max 100ms early reflection delay, at max_sample_rate (192kHz)
        let max_delay_samples = (max_sample_rate * 0.10).ceil() as usize;
        self.delay_buffer = vec![0.0; max_delay_samples];
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix;
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        if self.delay_buffer.is_empty() {
            return sample;
        }

        let delay_samples = ((self.delay_ms / 1000.0) * self.sample_rate) as usize;
        let delay_samples = delay_samples.clamp(1, self.delay_buffer.len());

        let read_idx = if self.write_idx >= delay_samples {
            self.write_idx - delay_samples
        } else {
            self.delay_buffer.len() - (delay_samples - self.write_idx)
        };

        let delayed_sample = self.delay_buffer[read_idx];

        // Write current sample
        self.delay_buffer[self.write_idx] = sample;
        self.write_idx = (self.write_idx + 1) % self.delay_buffer.len();

        // Very simple dry/wet mix
        sample * (1.0 - self.mix) + delayed_sample * self.mix
    }
}
