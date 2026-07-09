//! Stable device identity (CROSS-25).
//!
//! Two identical interfaces (a common case on Windows: two Scarlett 2i2) report
//! the same name. Matching a selection by name alone always yields the first of
//! the pair, so picking the second in the UI silently opens the first.
//!
//! The fix is to remember the device's position in the host enumeration
//! alongside its name. The index disambiguates; the name still validates, since
//! the enumeration can be reordered between the refresh that produced the index
//! and the apply that consumes it (hotplug). When the index no longer names the
//! expected device, we fall back to the old name search.
//!
//! The lookup is generic over the name accessor so it can be exercised without a
//! real `cpal::Host`; `cpal::Device` cannot be constructed synthetically.

/// Pick the device the user selected out of a fresh host enumeration.
///
/// Prefers `enum_index`, accepting it only if the device there still reports
/// `raw_name`. Otherwise searches by name, which restores the pre-`enum_index`
/// behaviour for devices that moved.
///
/// `name_of` returns `None` for devices whose name could not be read; those
/// never match.
pub fn resolve_device<D, F>(
    devices: impl IntoIterator<Item = D>,
    enum_index: usize,
    raw_name: &str,
    name_of: F,
) -> Option<D>
where
    F: Fn(&D) -> Option<String>,
{
    let mut devices: Vec<D> = devices.into_iter().collect();

    let matches_name = |d: &D| name_of(d).as_deref() == Some(raw_name);

    if devices.get(enum_index).is_some_and(matches_name) {
        return Some(devices.swap_remove(enum_index));
    }

    let found = devices.iter().position(matches_name)?;
    Some(devices.swap_remove(found))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Stand-in for `cpal::Device`: a name plus an identity we can assert on.
    #[derive(Debug, PartialEq)]
    struct FakeDevice {
        name: &'static str,
        serial: u32,
    }

    fn named(d: &FakeDevice) -> Option<String> {
        Some(d.name.to_string())
    }

    fn two_scarletts() -> Vec<FakeDevice> {
        vec![
            FakeDevice {
                name: "Built-in Audio",
                serial: 0,
            },
            FakeDevice {
                name: "Scarlett 2i2",
                serial: 1,
            },
            FakeDevice {
                name: "Scarlett 2i2",
                serial: 2,
            },
        ]
    }

    #[test]
    fn device_identity_index_disambiguates_duplicate_names() {
        // The user picked the *second* Scarlett (enumeration index 2).
        let picked = resolve_device(two_scarletts(), 2, "Scarlett 2i2", named).unwrap();
        assert_eq!(picked.serial, 2, "index must win over first-name-match");

        // ...and the first is still reachable at its own index.
        let picked = resolve_device(two_scarletts(), 1, "Scarlett 2i2", named).unwrap();
        assert_eq!(picked.serial, 1);
    }

    #[test]
    fn device_identity_falls_back_to_name_when_index_moved() {
        // A device was unplugged, so the Scarlett slid from index 2 to index 1.
        let devices = vec![
            FakeDevice {
                name: "Built-in Audio",
                serial: 0,
            },
            FakeDevice {
                name: "Scarlett 2i2",
                serial: 2,
            },
        ];
        let picked = resolve_device(devices, 2, "Scarlett 2i2", named).unwrap();
        assert_eq!(picked.serial, 2, "stale index must fall back to name search");
    }

    #[test]
    fn device_identity_falls_back_when_index_names_another_device() {
        // Index 2 exists but now holds a different device.
        let devices = vec![
            FakeDevice {
                name: "Scarlett 2i2",
                serial: 1,
            },
            FakeDevice {
                name: "Built-in Audio",
                serial: 0,
            },
            FakeDevice {
                name: "HDMI Output",
                serial: 3,
            },
        ];
        let picked = resolve_device(devices, 2, "Scarlett 2i2", named).unwrap();
        assert_eq!(picked.serial, 1);
    }

    #[test]
    fn device_identity_out_of_bounds_index_is_not_a_panic() {
        let picked = resolve_device(two_scarletts(), 99, "Built-in Audio", named).unwrap();
        assert_eq!(picked.serial, 0);
    }

    #[test]
    fn device_identity_returns_none_when_device_is_gone() {
        assert!(resolve_device(two_scarletts(), 1, "Focusrite 18i20", named).is_none());
    }

    #[test]
    fn device_identity_unreadable_name_never_matches() {
        // `cpal::Device::name()` is fallible; such a device must not be selected
        // even when it sits exactly at `enum_index`.
        let devices = two_scarletts();
        let picked = resolve_device(devices, 1, "Scarlett 2i2", |_| None);
        assert!(picked.is_none());
    }

    #[test]
    fn device_identity_preserves_non_ascii_names() {
        // No Unicode handling is needed: `cpal::Device::name()` returns `String`
        // (UTF-8 by construction). This pins that the lookup is byte-transparent.
        let devices = vec![
            FakeDevice {
                name: "Saída de Áudio",
                serial: 7,
            },
            FakeDevice {
                name: "オーディオ出力",
                serial: 8,
            },
        ];
        let picked = resolve_device(devices, 1, "オーディオ出力", named).unwrap();
        assert_eq!(picked.serial, 8);
    }
}
