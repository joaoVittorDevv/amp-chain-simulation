//! Stream configuration negotiation from a device's advertised capabilities.
//!
//! `cpal`'s `default_input_config()` / `default_output_config()` report *one*
//! configuration the driver considers default — on ASIO and WASAPI that is
//! frequently an integer format even when the device also advertises `F32`.
//! Enumerating `supported_*_configs()` instead lets us pick the format that
//! costs the least conversion, and lets `max_block` be derived from the range
//! the driver actually promises rather than from a guess (CROSS-11).

use cpal::traits::DeviceTrait;
use cpal::{
    Device, SampleFormat, SampleRate, SupportedBufferSize, SupportedStreamConfig,
    SupportedStreamConfigRange,
};

/// Block size assumed when the platform reports `SupportedBufferSize::Unknown`
/// (some ALSA and PulseAudio paths do).
pub const FALLBACK_MAX_BLOCK: usize = 8192;

/// Upper bound on a driver-reported block size. Some backends advertise a
/// `max` in the millions; `StandalonePipeline::new` would allocate scratch
/// buffers for it. Exceeding this is harmless at runtime because the callback
/// chunks any oversized block over `max_block` (T14).
const MAX_BLOCK_CEILING: usize = 1 << 16;

/// Input formats the callback can deinterleave, in order of preference.
/// Mirrors the `match config.sample_format()` arms of the input stream.
pub const INPUT_FORMATS: [SampleFormat; 3] =
    [SampleFormat::F32, SampleFormat::I32, SampleFormat::I16];

/// Output formats the callback can write, in order of preference. There is no
/// `I32` arm on the output stream, so negotiating `I32` here would hand
/// `build_output_stream` a format it rejects — on a device that advertises
/// `I32` but not `F32` that would turn a working `I16` stream into an error.
pub const OUTPUT_FORMATS: [SampleFormat; 2] = [SampleFormat::F32, SampleFormat::I16];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamDirection {
    Input,
    Output,
}

impl StreamDirection {
    /// Formats the corresponding callback can handle, most preferred first.
    pub fn formats(self) -> &'static [SampleFormat] {
        match self {
            StreamDirection::Input => &INPUT_FORMATS,
            StreamDirection::Output => &OUTPUT_FORMATS,
        }
    }
}

/// A negotiated configuration plus the largest block it can deliver.
#[derive(Clone, Debug)]
pub struct PickedConfig {
    pub config: SupportedStreamConfig,
    pub max_block: usize,
}

/// Largest block a driver-reported buffer range can deliver.
pub fn max_block_from(buffer_size: &SupportedBufferSize) -> usize {
    match *buffer_size {
        SupportedBufferSize::Range { max, .. } if max > 0 => (max as usize).min(MAX_BLOCK_CEILING),
        _ => FALLBACK_MAX_BLOCK,
    }
}

/// Choose the best config among `ranges`, or `None` when none carries a format
/// in `allowed`.
///
/// Ranked by, in order: distance from `default_sample_rate` (resampling is the
/// most expensive concession), distance from `default_channels` (a channel
/// count the device does not natively run is the likeliest to fail to open),
/// then position in `allowed`. Ties resolve to the first range enumerated, so
/// the result is stable for a given device.
pub fn pick_from_ranges(
    ranges: &[SupportedStreamConfigRange],
    default_sample_rate: SampleRate,
    default_channels: u16,
    allowed: &[SampleFormat],
) -> Option<PickedConfig> {
    ranges
        .iter()
        .filter_map(|range| {
            let rank = allowed.iter().position(|f| *f == range.sample_format())?;
            let (min, max) = (range.min_sample_rate().0, range.max_sample_rate().0);
            if min > max {
                return None;
            }
            let rate = default_sample_rate.0.clamp(min, max);
            let key = (
                rate.abs_diff(default_sample_rate.0),
                range.channels().abs_diff(default_channels),
                rank,
            );
            Some((key, range, SampleRate(rate)))
        })
        .min_by_key(|(key, _, _)| *key)
        .and_then(|(_, range, rate)| {
            let max_block = max_block_from(range.buffer_size());
            Some(PickedConfig {
                config: (*range).try_with_sample_rate(rate)?,
                max_block,
            })
        })
}

/// Negotiate a stream config for `device`.
///
/// Falls back to `default_*_config()` when the device cannot enumerate its
/// supported configs, enumerates none, or enumerates none we can build.
pub fn pick_config(
    device: &Device,
    dir: StreamDirection,
) -> Result<PickedConfig, cpal::DefaultStreamConfigError> {
    let default = match dir {
        StreamDirection::Input => device.default_input_config()?,
        StreamDirection::Output => device.default_output_config()?,
    };

    let ranges: Vec<SupportedStreamConfigRange> = match dir {
        StreamDirection::Input => device
            .supported_input_configs()
            .map(Iterator::collect)
            .unwrap_or_default(),
        StreamDirection::Output => device
            .supported_output_configs()
            .map(Iterator::collect)
            .unwrap_or_default(),
    };

    let picked = pick_from_ranges(
        &ranges,
        default.sample_rate(),
        default.channels(),
        dir.formats(),
    );

    Ok(picked.unwrap_or_else(|| PickedConfig {
        max_block: max_block_from(default.buffer_size()),
        config: default,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range(
        format: SampleFormat,
        min_sr: u32,
        max_sr: u32,
        buffer: SupportedBufferSize,
    ) -> SupportedStreamConfigRange {
        SupportedStreamConfigRange::new(2, SampleRate(min_sr), SampleRate(max_sr), buffer, format)
    }

    fn fixed(format: SampleFormat, sr: u32) -> SupportedStreamConfigRange {
        range(
            format,
            sr,
            sr,
            SupportedBufferSize::Range { min: 64, max: 1024 },
        )
    }

    #[test]
    fn pick_config_prefers_f32_over_i32_over_i16_at_the_same_sample_rate() {
        // Enumerated worst-first so a first-match tie-break cannot pass by luck.
        let ranges = [
            fixed(SampleFormat::I16, 48_000),
            fixed(SampleFormat::I32, 48_000),
            fixed(SampleFormat::F32, 48_000),
        ];
        let picked =
            pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).expect("a config");
        assert_eq!(picked.config.sample_format(), SampleFormat::F32);

        // Drop F32 and I32 must win over I16; drop it too and I16 remains.
        let picked = pick_from_ranges(&ranges[..2], SampleRate(48_000), 2, &INPUT_FORMATS)
            .expect("a config");
        assert_eq!(picked.config.sample_format(), SampleFormat::I32);

        let picked = pick_from_ranges(&ranges[..1], SampleRate(48_000), 2, &INPUT_FORMATS)
            .expect("a config");
        assert_eq!(picked.config.sample_format(), SampleFormat::I16);
    }

    #[test]
    fn pick_config_output_direction_never_negotiates_i32() {
        // The output callback has no I32 arm; I16 must win even though I32
        // outranks it on the input side.
        let ranges = [
            fixed(SampleFormat::I32, 48_000),
            fixed(SampleFormat::I16, 48_000),
        ];
        let picked =
            pick_from_ranges(&ranges, SampleRate(48_000), 2, &OUTPUT_FORMATS).expect("a config");
        assert_eq!(picked.config.sample_format(), SampleFormat::I16);
    }

    #[test]
    fn pick_config_takes_the_sample_rate_closest_to_the_default() {
        // Sample-rate distance outranks format: matching the default rate
        // avoids resampling, which costs more than an integer conversion.
        let ranges = [
            fixed(SampleFormat::F32, 44_100),
            fixed(SampleFormat::I16, 48_000),
        ];
        let picked =
            pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).expect("a config");
        assert_eq!(picked.config.sample_rate(), SampleRate(48_000));
        assert_eq!(picked.config.sample_format(), SampleFormat::I16);
    }

    #[test]
    fn pick_config_clamps_the_default_rate_into_a_range() {
        let buffer = SupportedBufferSize::Range { min: 64, max: 512 };
        let ranges = [range(SampleFormat::F32, 88_200, 192_000, buffer)];
        let picked =
            pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).expect("a config");
        assert_eq!(picked.config.sample_rate(), SampleRate(88_200));

        let ranges = [range(SampleFormat::F32, 44_100, 96_000, buffer)];
        let picked =
            pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).expect("a config");
        assert_eq!(picked.config.sample_rate(), SampleRate(48_000));
    }

    #[test]
    fn pick_config_prefers_the_default_channel_count_over_the_better_format() {
        // An 8-channel F32 range on a device that runs 2 channels is likelier
        // to fail to open than a 2-channel I16 one.
        let buffer = SupportedBufferSize::Range { min: 64, max: 1024 };
        let ranges = [
            SupportedStreamConfigRange::new(
                8,
                SampleRate(48_000),
                SampleRate(48_000),
                buffer,
                SampleFormat::F32,
            ),
            SupportedStreamConfigRange::new(
                2,
                SampleRate(48_000),
                SampleRate(48_000),
                buffer,
                SampleFormat::I16,
            ),
        ];
        let picked =
            pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).expect("a config");
        assert_eq!(picked.config.channels(), 2);
        assert_eq!(picked.config.sample_format(), SampleFormat::I16);
    }

    #[test]
    fn pick_config_derives_max_block_from_the_buffer_range() {
        let ranges = [range(
            SampleFormat::F32,
            48_000,
            48_000,
            SupportedBufferSize::Range { min: 64, max: 512 },
        )];
        let picked =
            pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).expect("a config");
        assert_eq!(picked.max_block, 512);
    }

    #[test]
    fn pick_config_falls_back_to_8192_when_the_buffer_size_is_unknown() {
        let ranges = [range(
            SampleFormat::F32,
            48_000,
            48_000,
            SupportedBufferSize::Unknown,
        )];
        let picked =
            pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).expect("a config");
        assert_eq!(picked.max_block, FALLBACK_MAX_BLOCK);

        // A degenerate `max: 0` is as useless as `Unknown`.
        assert_eq!(
            max_block_from(&SupportedBufferSize::Range { min: 0, max: 0 }),
            FALLBACK_MAX_BLOCK
        );
    }

    #[test]
    fn pick_config_caps_an_absurd_driver_reported_block() {
        assert_eq!(
            max_block_from(&SupportedBufferSize::Range {
                min: 64,
                max: u32::MAX
            }),
            MAX_BLOCK_CEILING
        );
    }

    #[test]
    fn pick_config_returns_none_without_a_supported_format() {
        let ranges = [
            fixed(SampleFormat::U8, 48_000),
            fixed(SampleFormat::F64, 48_000),
        ];
        assert!(pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).is_none());
        assert!(pick_from_ranges(&[], SampleRate(48_000), 2, &INPUT_FORMATS).is_none());
    }

    #[test]
    fn pick_config_skips_an_inverted_sample_rate_range() {
        let buffer = SupportedBufferSize::Range { min: 64, max: 512 };
        let ranges = [range(SampleFormat::F32, 96_000, 44_100, buffer)];
        assert!(pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).is_none());
    }
}
