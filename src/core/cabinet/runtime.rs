use super::types::{CabinetError, CabinetRuntime};
use audioadapter_buffers::direct::SequentialSliceOfVecs;
use fft_convolver::FFTConvolver;
use rubato::{Fft, FixedSync, Resampler};
use std::io::Cursor;

impl CabinetRuntime {
    /// Build a runtime convolver pair from raw WAV bytes.
    ///
    /// Decodes the WAV (i16 or f32), resamples to `engine_sample_rate` if the
    /// IR's native rate differs, fans a mono IR out to both channels (stereo
    /// maps channel 0 → L, 1 → R), and initializes two `FFTConvolver`s sized
    /// for `max_block_size`.
    ///
    /// This performs allocations and FFT initialization — it must be called
    /// off the audio thread.
    pub fn build(
        wav_bytes: &[u8],
        engine_sample_rate: f32,
        max_block_size: usize,
    ) -> Result<CabinetRuntime, CabinetError> {
        let ir_hash = blake3::hash(wav_bytes).to_hex().to_string();

        // --- Decode WAV to planar per-channel f32 ---
        let (mut channels, native_rate) = decode_wav_planar(wav_bytes)?;

        if channels.is_empty() || channels[0].is_empty() {
            return Err(CabinetError::WavDecode("empty IR".to_string()));
        }

        // --- Resample if the native rate differs from the engine rate ---
        let engine_rate = engine_sample_rate.round() as usize;
        if native_rate as usize != engine_rate && engine_rate > 0 {
            channels = resample_planar(&channels, native_rate as usize, engine_rate)?;
        }

        // --- Map channels to L / R ---
        let ir_l = channels[0].clone();
        let ir_r = if channels.len() >= 2 {
            channels[1].clone()
        } else {
            channels[0].clone()
        };

        let num_frames = ir_l.len();

        let mut convolver_l = FFTConvolver::default();
        let mut convolver_r = FFTConvolver::default();

        convolver_l
            .init(max_block_size, &ir_l)
            .map_err(|e| CabinetError::WavDecode(format!("convolver L init failed: {:?}", e)))?;
        convolver_r
            .init(max_block_size, &ir_r)
            .map_err(|e| CabinetError::WavDecode(format!("convolver R init failed: {:?}", e)))?;

        Ok(CabinetRuntime {
            convolver_l,
            convolver_r,
            ir_hash,
            num_frames,
        })
    }
}

/// Decode WAV bytes into planar per-channel f32 samples plus the native rate.
fn decode_wav_planar(wav_bytes: &[u8]) -> Result<(Vec<Vec<f32>>, u32), CabinetError> {
    let mut reader = hound::WavReader::new(Cursor::new(wav_bytes))
        .map_err(|e| CabinetError::WavDecode(e.to_string()))?;
    let spec = reader.spec();
    let channels = spec.channels.max(1) as usize;
    let sample_rate = spec.sample_rate;

    // Decode to a flat interleaved f32 vector, normalizing integer PCM.
    let interleaved: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader
            .samples::<f32>()
            .filter_map(Result::ok)
            .collect(),
        hound::SampleFormat::Int => {
            let max = (1i64 << (spec.bits_per_sample - 1)) as f32;
            reader
                .samples::<i32>()
                .filter_map(Result::ok)
                .map(|s| s as f32 / max)
                .collect()
        }
    };

    // De-interleave into planar channels.
    let frames = interleaved.len() / channels;
    let mut planar: Vec<Vec<f32>> = vec![Vec::with_capacity(frames); channels];
    for (i, sample) in interleaved.into_iter().enumerate() {
        planar[i % channels].push(sample);
    }

    Ok((planar, sample_rate))
}

/// Resample planar per-channel audio from `from_rate` to `to_rate` using rubato's
/// synchronous FFT resampler. Runs offline (whole clip) — not on the audio thread.
fn resample_planar(
    channels_in: &[Vec<f32>],
    from_rate: usize,
    to_rate: usize,
) -> Result<Vec<Vec<f32>>, CabinetError> {
    let nbr_channels = channels_in.len();
    let input_len = channels_in[0].len();

    let mut resampler = Fft::<f32>::new(from_rate, to_rate, 1024, 1, nbr_channels, FixedSync::Both)
        .map_err(|e| CabinetError::Resample(e.to_string()))?;

    let needed = resampler.process_all_needed_output_len(input_len);
    let mut out: Vec<Vec<f32>> = (0..nbr_channels).map(|_| vec![0.0f32; needed]).collect();

    let input_adapter = SequentialSliceOfVecs::new(channels_in, nbr_channels, input_len)
        .map_err(|e| CabinetError::Resample(e.to_string()))?;
    let mut output_adapter = SequentialSliceOfVecs::new_mut(&mut out, nbr_channels, needed)
        .map_err(|e| CabinetError::Resample(e.to_string()))?;

    let (_in_len, out_len) = resampler
        .process_all_into_buffer(&input_adapter, &mut output_adapter, input_len, None)
        .map_err(|e| CabinetError::Resample(e.to_string()))?;

    for ch in out.iter_mut() {
        ch.truncate(out_len);
    }

    Ok(out)
}
