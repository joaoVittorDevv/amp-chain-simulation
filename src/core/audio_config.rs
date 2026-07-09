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

/// Choose the best configs among `ranges`, sorted by preference (best first).
/// Returns empty when none carries a format in `allowed`.
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
) -> Vec<PickedConfig> {
    let mut candidates: Vec<_> = ranges
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
            let max_block = max_block_from(range.buffer_size());
            let config = (*range).try_with_sample_rate(SampleRate(rate))?;
            Some((key, PickedConfig { config, max_block }))
        })
        .collect();

    // Sort by rank (best first)
    candidates.sort_by_key(|(key, _)| *key);

    // Extract just the configs
    candidates.into_iter().map(|(_, cfg)| cfg).collect()
}

/// The best configs among `ranges` that can actually run at `rate`.
///
/// Ranges that do not span `rate` are discarded rather than clamped, so every
/// returned config carries exactly `rate`. This is what lets full duplex commit
/// to one rate for both directions.
pub fn pick_at_rate(
    ranges: &[SupportedStreamConfigRange],
    rate: SampleRate,
    default_channels: u16,
    allowed: &[SampleFormat],
) -> Vec<PickedConfig> {
    let spanning: Vec<_> = ranges
        .iter()
        .filter(|r| spans(r, rate.0))
        .cloned()
        .collect();
    pick_from_ranges(&spanning, rate, default_channels, allowed)
}

/// Every config range `device` advertises for `dir`, or empty if it cannot say.
pub fn supported_ranges(device: &Device, dir: StreamDirection) -> Vec<SupportedStreamConfigRange> {
    match dir {
        StreamDirection::Input => device
            .supported_input_configs()
            .map(Iterator::collect)
            .unwrap_or_default(),
        StreamDirection::Output => device
            .supported_output_configs()
            .map(Iterator::collect)
            .unwrap_or_default(),
    }
}

fn default_config(
    device: &Device,
    dir: StreamDirection,
) -> Result<SupportedStreamConfig, cpal::DefaultStreamConfigError> {
    match dir {
        StreamDirection::Input => device.default_input_config(),
        StreamDirection::Output => device.default_output_config(),
    }
}

/// Negotiate stream configs for `device`, sorted by preference (best first).
///
/// Tries enumeration first. Falls back to `default_*_config()` only when the
/// device cannot enumerate its supported configs, enumerates none, or
/// enumerates none we can build.
pub fn pick_config(
    device: &Device,
    dir: StreamDirection,
) -> Result<Vec<PickedConfig>, cpal::DefaultStreamConfigError> {
    let ranges = supported_ranges(device, dir);

    if !ranges.is_empty() {
        let (default_sr, default_ch) = match default_config(device, dir) {
            Ok(def) => (def.sample_rate(), def.channels()),
            // No default to rank against; assume the CD-era stereo norm.
            Err(_) => (SampleRate(48_000), 2),
        };

        let picked = pick_from_ranges(&ranges, default_sr, default_ch, dir.formats());
        if !picked.is_empty() {
            return Ok(picked);
        }
    }

    let default = default_config(device, dir)?;
    Ok(vec![PickedConfig {
        max_block: max_block_from(default.buffer_size()),
        config: default,
    }])
}

/// Why a full-duplex negotiation could not settle on a shared sample rate.
#[derive(Debug)]
pub enum FullDuplexError {
    /// A device reported neither an enumeration nor a default config.
    Device(cpal::DefaultStreamConfigError),
    /// Both devices are usable on their own, but share no sample rate. The
    /// payloads are the inclusive `(min, max)` rate intervals each side offers
    /// in a format the corresponding callback can handle.
    NoCommonSampleRate {
        input: Vec<(u32, u32)>,
        output: Vec<(u32, u32)>,
    },
}

impl From<cpal::DefaultStreamConfigError> for FullDuplexError {
    fn from(err: cpal::DefaultStreamConfigError) -> Self {
        FullDuplexError::Device(err)
    }
}

impl std::fmt::Display for FullDuplexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FullDuplexError::Device(err) => {
                write!(f, "device reported no usable configuration: {err}")
            }
            FullDuplexError::NoCommonSampleRate { input, output } => write!(
                f,
                "no sample rate is common to both devices (input offers {}; output offers {})",
                format_intervals(input),
                format_intervals(output),
            ),
        }
    }
}

impl std::error::Error for FullDuplexError {}

fn format_intervals(intervals: &[(u32, u32)]) -> String {
    if intervals.is_empty() {
        return "nothing".to_string();
    }
    intervals
        .iter()
        .map(|(min, max)| {
            if min == max {
                format!("{min} Hz")
            } else {
                format!("{min}-{max} Hz")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn spans(range: &SupportedStreamConfigRange, rate: u32) -> bool {
    let (min, max) = (range.min_sample_rate().0, range.max_sample_rate().0);
    min <= max && min <= rate && rate <= max
}

/// The inclusive sample-rate intervals `ranges` offers in a format we can use.
/// Inverted ranges are dropped, matching `pick_from_ranges`.
fn rate_intervals(
    ranges: &[SupportedStreamConfigRange],
    allowed: &[SampleFormat],
) -> Vec<(u32, u32)> {
    ranges
        .iter()
        .filter(|r| allowed.contains(&r.sample_format()))
        .map(|r| (r.min_sample_rate().0, r.max_sample_rate().0))
        .filter(|(min, max)| min <= max)
        .collect()
}

/// The sample rate both sides can run, closest to the input's default.
///
/// The input is the master clock: the pipeline is driven from the capture
/// callback, so a rate the input dislikes costs more than one the output
/// dislikes. Overlaps are searched pairwise because a device advertises its
/// rates as a set of intervals, not as one contiguous span.
fn best_common_rate(
    input: &[(u32, u32)],
    output: &[(u32, u32)],
    input_default: u32,
    output_default: u32,
) -> Option<u32> {
    let mut best: Option<((u32, u32, u32), u32)> = None;

    for &(in_min, in_max) in input {
        for &(out_min, out_max) in output {
            let low = in_min.max(out_min);
            let high = in_max.min(out_max);
            if low > high {
                continue;
            }
            // Both defaults are worth trying: the input's is the primary
            // preference, but when it falls outside the overlap the output's
            // clamped default is often a rate the hardware genuinely runs.
            for candidate in [
                input_default.clamp(low, high),
                output_default.clamp(low, high),
            ] {
                let key = (
                    candidate.abs_diff(input_default),
                    candidate.abs_diff(output_default),
                    candidate,
                );
                if best.as_ref().is_none_or(|(best_key, _)| key < *best_key) {
                    best = Some((key, candidate));
                }
            }
        }
    }

    best.map(|(_, rate)| rate)
}

/// One side of a full-duplex negotiation, reduced to what the search needs.
struct DuplexSide {
    ranges: Vec<SupportedStreamConfigRange>,
    intervals: Vec<(u32, u32)>,
    default_rate: u32,
    default_channels: u16,
    /// Set when the device could not enumerate anything usable, in which case
    /// its `default_*_config()` is the only config we may open it with.
    fallback: Option<PickedConfig>,
}

fn duplex_side(
    device: &Device,
    dir: StreamDirection,
) -> Result<DuplexSide, cpal::DefaultStreamConfigError> {
    let ranges = supported_ranges(device, dir);
    let intervals = rate_intervals(&ranges, dir.formats());

    if intervals.is_empty() {
        // Mirrors `pick_config`'s fallback: a device that enumerates nothing we
        // can build gets exactly one config, at exactly one rate.
        let default = default_config(device, dir)?;
        let rate = default.sample_rate().0;
        return Ok(DuplexSide {
            ranges: Vec::new(),
            intervals: vec![(rate, rate)],
            default_rate: rate,
            default_channels: default.channels(),
            fallback: Some(PickedConfig {
                max_block: max_block_from(default.buffer_size()),
                config: default,
            }),
        });
    }

    let (default_rate, default_channels) = match default_config(device, dir) {
        Ok(def) => (def.sample_rate().0, def.channels()),
        Err(_) => (48_000, 2),
    };

    Ok(DuplexSide {
        ranges,
        intervals,
        default_rate,
        default_channels,
        fallback: None,
    })
}

/// Negotiate full-duplex configs that share one sample rate.
///
/// The search runs over the devices' *raw* advertised rate intervals, not over
/// configs each device already narrowed to its own default. Two devices that
/// both span `44100-96000` but default to `48000` and `44100` respectively do
/// have common rates; collapsing each to its default first would hide that.
///
/// Returns `(input_configs, output_configs, common_sample_rate)`, both lists
/// sorted best-first and every config carrying `common_sample_rate`. Never
/// returns a rate mismatch: when no rate is common, this is an error, and the
/// caller must not open the streams.
pub fn pick_full_duplex(
    input_device: &Device,
    output_device: &Device,
) -> Result<(Vec<PickedConfig>, Vec<PickedConfig>, SampleRate), FullDuplexError> {
    let input = duplex_side(input_device, StreamDirection::Input)?;
    let output = duplex_side(output_device, StreamDirection::Output)?;

    let no_common = || FullDuplexError::NoCommonSampleRate {
        input: input.intervals.clone(),
        output: output.intervals.clone(),
    };

    let rate = best_common_rate(
        &input.intervals,
        &output.intervals,
        input.default_rate,
        output.default_rate,
    )
    .ok_or_else(no_common)?;
    let rate = SampleRate(rate);

    let input_configs = match input.fallback.clone() {
        Some(picked) => vec![picked],
        None => pick_at_rate(
            &input.ranges,
            rate,
            input.default_channels,
            StreamDirection::Input.formats(),
        ),
    };
    let output_configs = match output.fallback.clone() {
        Some(picked) => vec![picked],
        None => pick_at_rate(
            &output.ranges,
            rate,
            output.default_channels,
            StreamDirection::Output.formats(),
        ),
    };

    if input_configs.is_empty() || output_configs.is_empty() {
        return Err(no_common());
    }

    debug_assert!(input_configs
        .iter()
        .chain(&output_configs)
        .all(|cfg| cfg.config.sample_rate() == rate));

    Ok((input_configs, output_configs, rate))
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
        let picked = pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS);
        assert!(!picked.is_empty(), "expected configs");
        assert_eq!(picked[0].config.sample_format(), SampleFormat::F32);

        // Drop F32 and I32 must win over I16; drop it too and I16 remains.
        let picked = pick_from_ranges(&ranges[..2], SampleRate(48_000), 2, &INPUT_FORMATS);
        assert!(!picked.is_empty(), "expected configs");
        assert_eq!(picked[0].config.sample_format(), SampleFormat::I32);

        let picked = pick_from_ranges(&ranges[..1], SampleRate(48_000), 2, &INPUT_FORMATS);
        assert!(!picked.is_empty(), "expected configs");
        assert_eq!(picked[0].config.sample_format(), SampleFormat::I16);
    }

    #[test]
    fn pick_config_output_direction_never_negotiates_i32() {
        // The output callback has no I32 arm; I16 must win even though I32
        // outranks it on the input side.
        let ranges = [
            fixed(SampleFormat::I32, 48_000),
            fixed(SampleFormat::I16, 48_000),
        ];
        let picked = pick_from_ranges(&ranges, SampleRate(48_000), 2, &OUTPUT_FORMATS);
        assert!(!picked.is_empty(), "expected configs");
        assert_eq!(picked[0].config.sample_format(), SampleFormat::I16);
    }

    #[test]
    fn pick_config_takes_the_sample_rate_closest_to_the_default() {
        // Sample-rate distance outranks format: matching the default rate
        // avoids resampling, which costs more than an integer conversion.
        let ranges = [
            fixed(SampleFormat::F32, 44_100),
            fixed(SampleFormat::I16, 48_000),
        ];
        let picked = pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS);
        assert!(!picked.is_empty(), "expected configs");
        assert_eq!(picked[0].config.sample_rate(), SampleRate(48_000));
        assert_eq!(picked[0].config.sample_format(), SampleFormat::I16);
    }

    #[test]
    fn pick_config_clamps_the_default_rate_into_a_range() {
        let buffer = SupportedBufferSize::Range { min: 64, max: 512 };
        let ranges = [range(SampleFormat::F32, 88_200, 192_000, buffer)];
        let picked = pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS);
        assert!(!picked.is_empty(), "expected configs");
        assert_eq!(picked[0].config.sample_rate(), SampleRate(88_200));

        let ranges = [range(SampleFormat::F32, 44_100, 96_000, buffer)];
        let picked = pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS);
        assert!(!picked.is_empty(), "expected configs");
        assert_eq!(picked[0].config.sample_rate(), SampleRate(48_000));
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
        let picked = pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS);
        assert!(!picked.is_empty(), "expected configs");
        assert_eq!(picked[0].config.channels(), 2);
        assert_eq!(picked[0].config.sample_format(), SampleFormat::I16);
    }

    #[test]
    fn pick_config_derives_max_block_from_the_buffer_range() {
        let ranges = [range(
            SampleFormat::F32,
            48_000,
            48_000,
            SupportedBufferSize::Range { min: 64, max: 512 },
        )];
        let picked = pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS);
        assert!(!picked.is_empty(), "expected configs");
        assert_eq!(picked[0].max_block, 512);
    }

    #[test]
    fn pick_config_falls_back_to_8192_when_the_buffer_size_is_unknown() {
        let ranges = [range(
            SampleFormat::F32,
            48_000,
            48_000,
            SupportedBufferSize::Unknown,
        )];
        let picked = pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS);
        assert!(!picked.is_empty(), "expected configs");
        assert_eq!(picked[0].max_block, FALLBACK_MAX_BLOCK);

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
    fn pick_config_returns_empty_without_a_supported_format() {
        let ranges = [
            fixed(SampleFormat::U8, 48_000),
            fixed(SampleFormat::F64, 48_000),
        ];
        assert!(pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).is_empty());
        assert!(pick_from_ranges(&[], SampleRate(48_000), 2, &INPUT_FORMATS).is_empty());
    }

    #[test]
    fn pick_config_skips_an_inverted_sample_rate_range() {
        let buffer = SupportedBufferSize::Range { min: 64, max: 512 };
        let ranges = [range(SampleFormat::F32, 96_000, 44_100, buffer)];
        assert!(pick_from_ranges(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS).is_empty());
    }

    #[test]
    fn full_duplex_finds_a_rate_inside_two_overlapping_ranges() {
        // The regression: both devices span 44100-96000, but their defaults
        // disagree (48000 vs 44100). Collapsing each side to its own default
        // first would report no intersection; the overlap plainly has one.
        let input = [(44_100, 96_000)];
        let output = [(44_100, 96_000)];
        assert_eq!(
            best_common_rate(&input, &output, 48_000, 44_100),
            Some(48_000),
            "input is the master clock, so its default wins inside the overlap"
        );
    }

    #[test]
    fn full_duplex_clamps_the_input_default_into_the_overlap() {
        // Input prefers 44100 but the overlap starts at 48000: take 48000
        // rather than pretend either device can run 44100.
        let input = [(44_100, 96_000)];
        let output = [(48_000, 192_000)];
        assert_eq!(
            best_common_rate(&input, &output, 44_100, 48_000),
            Some(48_000)
        );

        // Symmetric: the overlap ends below the input's default.
        let input = [(44_100, 48_000)];
        let output = [(8_000, 48_000)];
        assert_eq!(
            best_common_rate(&input, &output, 96_000, 8_000),
            Some(48_000)
        );
    }

    #[test]
    fn full_duplex_searches_every_interval_pair() {
        // A device advertises rates as a set of intervals, not one span. The
        // only overlap here is the second input interval against the first
        // output interval.
        let input = [(8_000, 16_000), (88_200, 192_000)];
        let output = [(96_000, 96_000), (44_100, 48_000)];
        assert_eq!(
            best_common_rate(&input, &output, 48_000, 96_000),
            Some(96_000)
        );
    }

    #[test]
    fn full_duplex_lets_the_input_default_choose_between_two_overlaps() {
        // Two disjoint overlaps are both reachable; the input is the master
        // clock, so its default decides which one we commit to.
        let input = [(44_100, 44_100), (48_000, 48_000)];
        let output = [(44_100, 48_000)];
        assert_eq!(
            best_common_rate(&input, &output, 48_000, 44_100),
            Some(48_000)
        );
        assert_eq!(
            best_common_rate(&input, &output, 44_100, 48_000),
            Some(44_100)
        );
    }

    #[test]
    fn full_duplex_reports_no_rate_when_the_ranges_are_disjoint() {
        assert_eq!(
            best_common_rate(&[(44_100, 44_100)], &[(48_000, 48_000)], 44_100, 48_000),
            None
        );
        assert_eq!(
            best_common_rate(&[(8_000, 16_000)], &[(44_100, 192_000)], 16_000, 44_100),
            None
        );
        // A side with nothing usable can never intersect.
        assert_eq!(
            best_common_rate(&[], &[(48_000, 48_000)], 48_000, 48_000),
            None
        );
    }

    #[test]
    fn rate_intervals_keeps_only_ranges_the_callback_can_build() {
        let buffer = SupportedBufferSize::Range { min: 64, max: 512 };
        let ranges = [
            range(SampleFormat::F32, 44_100, 96_000, buffer),
            range(SampleFormat::U8, 8_000, 8_000, buffer), // format we cannot build
            range(SampleFormat::F32, 192_000, 44_100, buffer), // inverted
            range(SampleFormat::I32, 48_000, 48_000, buffer),
        ];
        assert_eq!(
            rate_intervals(&ranges, &INPUT_FORMATS),
            vec![(44_100, 96_000), (48_000, 48_000)]
        );
        // The output callback has no I32 arm, so that interval must not count.
        assert_eq!(
            rate_intervals(&ranges, &OUTPUT_FORMATS),
            vec![(44_100, 96_000)]
        );
    }

    #[test]
    fn pick_at_rate_drops_ranges_that_cannot_reach_the_rate() {
        let buffer = SupportedBufferSize::Range { min: 64, max: 512 };
        let ranges = [
            fixed(SampleFormat::F32, 44_100),
            range(SampleFormat::I16, 44_100, 96_000, buffer),
        ];
        // 48000 lies outside the F32 range, so the I16 range is the only one
        // that can honour it — no silent clamp back to 44100.
        let picked = pick_at_rate(&ranges, SampleRate(48_000), 2, &INPUT_FORMATS);
        assert_eq!(picked.len(), 1);
        assert_eq!(picked[0].config.sample_rate(), SampleRate(48_000));
        assert_eq!(picked[0].config.sample_format(), SampleFormat::I16);

        // At 44100 both qualify and the format preference decides.
        let picked = pick_at_rate(&ranges, SampleRate(44_100), 2, &INPUT_FORMATS);
        assert_eq!(picked.len(), 2);
        assert_eq!(picked[0].config.sample_format(), SampleFormat::F32);
        assert!(picked
            .iter()
            .all(|cfg| cfg.config.sample_rate() == SampleRate(44_100)));

        assert!(pick_at_rate(&ranges, SampleRate(192_000), 2, &INPUT_FORMATS).is_empty());
    }
}
