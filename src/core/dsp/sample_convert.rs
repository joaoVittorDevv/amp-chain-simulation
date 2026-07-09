//! Bit-depth conversions shared by every standalone audio input/output arm
//! (F32/I32/I16), so the DSP pipeline always sees f32 samples regardless of
//! the native format negotiated with ASIO/WASAPI/CoreAudio/ALSA.

/// Converts a 32-bit signed PCM sample to f32 in `[-1.0, 1.0]`.
///
/// Divides by `i32::MAX` (not `abs(i32::MIN)`): using `i32::MIN`'s magnitude
/// would let `i32::MIN` map to exactly `-1.0` but let `i32::MAX` overshoot
/// past `1.0`, which is worse for a signal that's supposed to live in
/// `[-1.0, 1.0]` since most downstream DSP does not expect out-of-range
/// input at full scale. The intermediate f64 division avoids precision loss
/// at the extremes of the i32 range that a plain f32 division would incur.
#[inline(always)]
pub fn i32_to_f32(sample: i32) -> f32 {
    (sample as f64 / i32::MAX as f64) as f32
}

/// Converts a 16-bit signed PCM sample to f32 in `[-1.0, 1.0]`.
#[inline(always)]
pub fn i16_to_f32(sample: i16) -> f32 {
    (sample as f64 / i16::MAX as f64) as f32
}

/// Converts an f32 sample to 32-bit signed PCM.
///
/// Clamps to `[-1.0, 1.0]` before scaling. Scaling first and clamping after
/// would let an out-of-range f32 (e.g. from a hot gain stage) overflow past
/// `i32::MAX`/`i32::MIN` during the cast, which wraps around instead of
/// saturating and produces a full-scale polarity-inverted glitch.
#[inline(always)]
pub fn f32_to_i32(sample: f32) -> i32 {
    (sample.clamp(-1.0, 1.0) as f64 * i32::MAX as f64) as i32
}

/// Converts an f32 sample to 16-bit signed PCM. See [`f32_to_i32`] for why
/// the clamp must happen before scaling.
#[inline(always)]
pub fn f32_to_i16(sample: f32) -> i16 {
    (sample.clamp(-1.0, 1.0) as f64 * i16::MAX as f64) as i16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_convert_i32_round_trip_extremes() {
        assert!((i32_to_f32(i32::MAX) - 1.0).abs() < 1e-6);
        assert!((i32_to_f32(i32::MIN) - (-1.0)).abs() < 1e-3);
        assert_eq!(i32_to_f32(0), 0.0);
    }

    #[test]
    fn sample_convert_i16_round_trip_extremes() {
        assert!((i16_to_f32(i16::MAX) - 1.0).abs() < 1e-4);
        assert!((i16_to_f32(i16::MIN) - (-1.0)).abs() < 1e-3);
        assert_eq!(i16_to_f32(0), 0.0);
    }

    #[test]
    fn sample_convert_f32_to_i32_clamps_before_scaling() {
        // Out-of-range input must saturate, never wrap around to the
        // opposite polarity via i32 overflow.
        assert_eq!(f32_to_i32(2.0), i32::MAX);
        assert_eq!(f32_to_i32(-2.0), -i32::MAX);
        assert_eq!(f32_to_i32(1.0), i32::MAX);
        assert_eq!(f32_to_i32(0.0), 0);
    }

    #[test]
    fn sample_convert_f32_to_i16_clamps_before_scaling() {
        assert_eq!(f32_to_i16(2.0), i16::MAX);
        assert_eq!(f32_to_i16(-2.0), -i16::MAX);
        assert_eq!(f32_to_i16(1.0), i16::MAX);
        assert_eq!(f32_to_i16(0.0), 0);
    }

    #[test]
    fn sample_convert_i32_f32_round_trip_preserves_sign() {
        for &v in &[-0.75_f32, -0.25, 0.1, 0.5, 0.99] {
            let round_tripped = i32_to_f32(f32_to_i32(v));
            assert!((round_tripped - v).abs() < 1e-6, "v={v} rt={round_tripped}");
        }
    }

    #[test]
    fn sample_convert_i16_f32_round_trip_within_quantization() {
        for &v in &[-0.75_f32, -0.25, 0.1, 0.5, 0.99] {
            let round_tripped = i16_to_f32(f32_to_i16(v));
            assert!((round_tripped - v).abs() < 5e-5, "v={v} rt={round_tripped}");
        }
    }
}
