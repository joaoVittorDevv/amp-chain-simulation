//! CI-only validation for the native binaries produced by `cargo xtask bundle`.

#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::fs;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::path::{Path, PathBuf};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::process::Command;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[test]
#[ignore = "requires artifacts produced by `cargo xtask bundle distortion --release`"]
fn bundled_artifacts_do_not_link_libneural() {
    let target_dir = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target"));
    let bundled_dir = target_dir.join("bundled");
    let mut artifacts = Vec::new();

    collect_native_binaries(&bundled_dir, &mut artifacts);
    assert!(
        !artifacts.is_empty(),
        "no native bundle artifacts found under {}; run `cargo xtask bundle distortion --release` first",
        bundled_dir.display()
    );

    for artifact in artifacts {
        let output = dependency_tool(&artifact);
        assert!(
            output.status.success(),
            "dependency inspection failed for {}:\n{}",
            artifact.display(),
            String::from_utf8_lossy(&output.stderr)
        );

        let dependencies = String::from_utf8_lossy(&output.stdout);
        assert!(
            !dependencies.to_ascii_lowercase().contains("libneural"),
            "{} depends on libneural:\n{}",
            artifact.display(),
            dependencies
        );
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn collect_native_binaries(dir: &Path, artifacts: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", dir.display()));

    for entry in entries {
        let path = entry
            .unwrap_or_else(|error| panic!("failed to read entry in {}: {error}", dir.display()))
            .path();
        if path.is_dir() {
            collect_native_binaries(&path, artifacts);
        } else if is_native_binary(&path) {
            artifacts.push(path);
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn is_native_binary(path: &Path) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    let Some(magic) = bytes.get(..4) else {
        return false;
    };

    #[cfg(target_os = "linux")]
    return magic == b"\x7fELF";

    #[cfg(target_os = "macos")]
    return matches!(
        magic,
        [0xfe, 0xed, 0xfa, 0xce]
            | [0xfe, 0xed, 0xfa, 0xcf]
            | [0xce, 0xfa, 0xed, 0xfe]
            | [0xcf, 0xfa, 0xed, 0xfe]
            | [0xca, 0xfe, 0xba, 0xbe]
            | [0xbe, 0xba, 0xfe, 0xca]
    );
}

#[cfg(target_os = "linux")]
fn dependency_tool(artifact: &Path) -> std::process::Output {
    Command::new("ldd")
        .arg(artifact)
        .output()
        .unwrap_or_else(|error| panic!("failed to run ldd: {error}"))
}

#[cfg(target_os = "macos")]
fn dependency_tool(artifact: &Path) -> std::process::Output {
    Command::new("otool")
        .arg("-L")
        .arg(artifact)
        .output()
        .unwrap_or_else(|error| panic!("failed to run otool: {error}"))
}
