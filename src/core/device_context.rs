//! Descriptor for one enumerated audio device, including devices the current
//! host cannot actually open.
//!
//! Device enumeration used to drop any device whose default config could not be
//! queried. That silently hid real hardware — most visibly an ASIO interface
//! locked to 24-bit, a format cpal's ASIO backend does not expose, which simply
//! vanished from the picker with no explanation (CROSS-16).
//!
//! [`DeviceContext::from_config_result`] instead keeps the device and records
//! *why* it is unusable, so the UI can list it greyed out with the reason.

use cpal::{SampleFormat, SupportedStreamConfig};
use std::fmt;

/// Which half of the duplex stream a device is being considered for.
///
/// The two directions accept different sample formats, so the same device can
/// be usable as an input and unusable as an output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Input,
    Output,
}

impl Direction {
    /// Sample formats routing can actually build a stream for.
    ///
    /// These mirror the `match config.sample_format()` arms in the standalone's
    /// audio worker. Both matches end in an explicit `StreamConfigNotSupported`,
    /// so any format missing here fails when the stream is built — which is why
    /// a device offering only such a format must be marked unusable up front
    /// rather than enabled in the picker and rejected later.
    pub const fn accepted_formats(self) -> &'static [SampleFormat] {
        match self {
            Direction::Input => &[SampleFormat::F32, SampleFormat::I32, SampleFormat::I16],
            Direction::Output => &[SampleFormat::F32, SampleFormat::I16],
        }
    }

    pub const fn is_input(self) -> bool {
        matches!(self, Direction::Input)
    }
}

/// One row in the input or output device picker.
///
/// `usable == false` means the device was enumerated by the host but cannot be
/// opened with the config we need; `unusable_reason` then carries a
/// human-readable explanation. `channels` and `sample_rate` are always `0` in
/// that case — either no config was negotiated, or one was but names a sample
/// format we cannot stream, and reporting its channel count would invite the
/// picker to offer channels no stream will deliver.
#[derive(Clone, Debug, PartialEq)]
pub struct DeviceContext {
    /// Display name, possibly beautified (Linux ALSA names are rewritten).
    pub name: String,
    /// Name as reported by cpal; the key used to re-find the device later.
    pub raw_name: String,
    /// Position in the host enumeration this device was discovered at. Two
    /// devices can share `raw_name`; the index tells them apart (CROSS-25).
    pub enum_index: usize,
    pub channels: u16,
    pub sample_rate: u32,
    /// Sample formats the host reports for this device. May be non-empty even
    /// when the device is unusable, and empty even when it is usable (a host
    /// can fail to enumerate ranges while still serving a default config).
    pub supported_formats: Vec<SampleFormat>,
    pub usable: bool,
    /// `Some` iff `!usable`.
    pub unusable_reason: Option<String>,
}

impl DeviceContext {
    /// Build a descriptor from the host's answer for this device's config.
    ///
    /// An `Err` config yields an unusable entry that still carries both names,
    /// so the caller can list the device rather than skip it. A device that
    /// negotiates a config but reports zero channels is also unusable: opening
    /// a stream on it cannot succeed, and the channel pickers would be empty.
    ///
    /// A config that negotiates cleanly but lands on a sample format
    /// `direction` cannot stream is unusable too. The host is happy to describe
    /// such a device — it is our routing that has no arm for the format — so
    /// nothing short of checking the format catches it before the stream build
    /// fails.
    pub fn from_config_result<E: fmt::Display>(
        name: impl Into<String>,
        raw_name: impl Into<String>,
        enum_index: usize,
        direction: Direction,
        config: Result<SupportedStreamConfig, E>,
        supported_formats: Vec<SampleFormat>,
    ) -> Self {
        let name = name.into();
        let raw_name = raw_name.into();

        let unusable_reason = match &config {
            Ok(cfg) if cfg.channels() == 0 => Some(format!(
                "Dispositivo não expõe canais utilizáveis.{}",
                format_supported(&supported_formats)
            )),
            Ok(cfg) if !direction.accepted_formats().contains(&cfg.sample_format()) => {
                Some(format!(
                    "Formato {:?} não suportado.{}",
                    cfg.sample_format(),
                    format_supported(&supported_formats)
                ))
            }
            Ok(_) => None,
            Err(err) => Some(format!(
                "Formato não suportado por este host: {err}.{}",
                format_supported(&supported_formats)
            )),
        };

        // An unusable device never carries negotiated values: the pickers size
        // their channel controls from `channels`, and a non-zero count here
        // would offer channels no stream is ever going to deliver.
        let (channels, sample_rate) = match (&config, &unusable_reason) {
            (Ok(cfg), None) => (cfg.channels(), cfg.sample_rate().0),
            _ => (0, 0),
        };

        Self {
            name,
            raw_name,
            enum_index,
            channels,
            sample_rate,
            supported_formats,
            usable: unusable_reason.is_none(),
            unusable_reason,
        }
    }
}

/// Trailing clause naming the formats the host did report, to distinguish "this
/// host understands nothing this device offers" from "the device offers formats
/// we could not negotiate a default from".
fn format_supported(formats: &[SampleFormat]) -> String {
    if formats.is_empty() {
        " Nenhum formato de amostra compatível foi reportado.".to_string()
    } else {
        let list: Vec<String> = formats.iter().map(|f| format!("{f:?}")).collect();
        format!(" Formatos reportados: {}.", list.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpal::{SampleRate, SupportedBufferSize};

    fn ok_config(channels: u16) -> Result<SupportedStreamConfig, String> {
        ok_config_with(channels, SampleFormat::F32)
    }

    fn ok_config_with(
        channels: u16,
        format: SampleFormat,
    ) -> Result<SupportedStreamConfig, String> {
        Ok(SupportedStreamConfig::new(
            channels,
            SampleRate(48_000),
            SupportedBufferSize::Range { min: 64, max: 4096 },
            format,
        ))
    }

    #[test]
    fn err_config_keeps_device_visible_and_marks_it_unusable() {
        let dev = DeviceContext::from_config_result(
            "ASIO Interface",
            "raw asio name",
            0,
            Direction::Input,
            Err::<SupportedStreamConfig, _>("the requested stream type is not supported"),
            vec![],
        );

        assert!(!dev.usable);
        assert_eq!(dev.name, "ASIO Interface");
        assert_eq!(dev.raw_name, "raw asio name");
        assert_eq!(dev.channels, 0);
        assert_eq!(dev.sample_rate, 0);
    }

    #[test]
    fn err_config_reason_quotes_the_host_error() {
        let dev = DeviceContext::from_config_result(
            "Focusrite",
            "Focusrite",
            0,
            Direction::Input,
            Err::<SupportedStreamConfig, _>("stream type not supported"),
            vec![],
        );

        let reason = dev.unusable_reason.expect("unusable device needs a reason");
        assert!(reason.contains("stream type not supported"), "{reason}");
        assert!(reason.contains("Nenhum formato"), "{reason}");
    }

    /// The CROSS-16 case: an ASIO device pinned to 24-bit. cpal's ASIO backend
    /// exposes no matching `SampleFormat`, so the default config errors, yet the
    /// device must remain listed with its formats surfaced in the reason.
    #[test]
    fn unusable_device_still_reports_the_formats_the_host_saw() {
        let dev = DeviceContext::from_config_result(
            "24-bit only",
            "24-bit only",
            0,
            Direction::Input,
            Err::<SupportedStreamConfig, _>("StreamTypeNotSupported"),
            vec![SampleFormat::I16],
        );

        assert!(!dev.usable);
        assert_eq!(dev.supported_formats, vec![SampleFormat::I16]);
        let reason = dev.unusable_reason.expect("unusable device needs a reason");
        assert!(reason.contains("I16"), "{reason}");
    }

    #[test]
    fn ok_config_is_usable_and_carries_negotiated_values() {
        let dev = DeviceContext::from_config_result(
            "Built-in",
            "Built-in",
            0,
            Direction::Input,
            ok_config(2),
            vec![SampleFormat::F32],
        );

        assert!(dev.usable);
        assert_eq!(dev.unusable_reason, None);
        assert_eq!(dev.channels, 2);
        assert_eq!(dev.sample_rate, 48_000);
    }

    #[test]
    fn zero_channel_device_is_unusable_despite_an_ok_config() {
        let dev = DeviceContext::from_config_result(
            "Phantom",
            "Phantom",
            0,
            Direction::Input,
            ok_config(0),
            vec![],
        );

        assert!(!dev.usable);
        assert!(dev.unusable_reason.is_some());
    }

    /// I32 is an input-only arm in the router, so the *same* negotiated config
    /// must come out usable on the way in and unusable on the way out.
    #[test]
    fn i32_is_usable_as_input_but_not_as_output() {
        let as_input = DeviceContext::from_config_result(
            "Interface",
            "Interface",
            0,
            Direction::Input,
            ok_config_with(2, SampleFormat::I32),
            vec![SampleFormat::I32],
        );
        let as_output = DeviceContext::from_config_result(
            "Interface",
            "Interface",
            0,
            Direction::Output,
            ok_config_with(2, SampleFormat::I32),
            vec![SampleFormat::I32],
        );

        assert!(as_input.usable);
        assert!(!as_output.usable);
        let reason = as_output.unusable_reason.expect("needs a reason");
        assert!(reason.contains("Formato I32 não suportado"), "{reason}");
    }

    /// A format neither direction handles: the host negotiates it happily, so
    /// only the format check stands between it and a failed stream build.
    #[test]
    fn unhandled_format_is_unusable_in_both_directions() {
        for direction in [Direction::Input, Direction::Output] {
            let dev = DeviceContext::from_config_result(
                "U16 device",
                "U16 device",
                0,
                direction,
                ok_config_with(2, SampleFormat::U16),
                vec![SampleFormat::U16],
            );

            assert!(!dev.usable, "{direction:?} accepted U16");
            let reason = dev.unusable_reason.expect("needs a reason");
            assert!(reason.contains("U16"), "{reason}");
        }
    }

    /// An unusable device must not advertise negotiated values, or the picker
    /// will draw channel controls for a stream that cannot open.
    #[test]
    fn unsupported_format_device_reports_no_negotiated_values() {
        let dev = DeviceContext::from_config_result(
            "I32 out",
            "I32 out",
            0,
            Direction::Output,
            ok_config_with(8, SampleFormat::I32),
            vec![SampleFormat::I32],
        );

        assert!(!dev.usable);
        assert_eq!(dev.channels, 0);
        assert_eq!(dev.sample_rate, 0);
    }

    /// Zero channels outranks the format check: a device that is broken in both
    /// ways should say so in the terms the user can act on.
    #[test]
    fn zero_channels_is_reported_before_an_unsupported_format() {
        let dev = DeviceContext::from_config_result(
            "Broken",
            "Broken",
            0,
            Direction::Output,
            ok_config_with(0, SampleFormat::I32),
            vec![],
        );

        let reason = dev.unusable_reason.expect("needs a reason");
        assert!(reason.contains("canais utilizáveis"), "{reason}");
    }
}
