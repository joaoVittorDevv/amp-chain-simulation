use std::path::{Path, PathBuf};
use std::process::Command;

// ── Platform helpers ──────────────────────────────────────────────────────────

/// Returns the path of `bin` by invoking `where.exe` (Windows) or `which` (Unix).
pub fn which_like(bin: &str) -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let cmd = "where.exe";
    #[cfg(not(target_os = "windows"))]
    let cmd = "which";

    Command::new(cmd)
        .arg(bin)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            // `where.exe` may return multiple lines; take the first.
            let raw = String::from_utf8_lossy(&o.stdout);
            PathBuf::from(raw.lines().next().unwrap_or("").trim())
        })
}

/// Returns the user's home directory via `%USERPROFILE%` (Windows) or `$HOME` (Unix).
pub fn home_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}

// ── Toolchain discovery ───────────────────────────────────────────────────────

/// Returns true if `path` refers to a file the current user can execute.
///
/// On Unix, `Path::exists()` alone can match a file that lacks the execute
/// bit (e.g. a toolchain archive extracted without preserving permissions),
/// which would panic downstream when the caller tries to run it. This checks
/// the user-execute permission bit explicitly.
///
/// On Windows, executability is determined by file extension rather than
/// POSIX permission bits, so plain existence is sufficient.
pub fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(path)
            .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        path.is_file()
    }
}

/// Standard install locations for `faust` on the host OS, tried in order when
/// the binary is absent from `PATH`.
///
/// Empty on platforms with no conventional prefix, which reduces `find_faust`
/// to its PATH lookup.
fn faust_fallback_paths() -> &'static [&'static str] {
    #[cfg(target_os = "linux")]
    {
        &["/usr/bin/faust", "/usr/local/bin/faust"]
    }
    #[cfg(target_os = "macos")]
    {
        // Apple Silicon Homebrew prefix, then the Intel one.
        &["/opt/homebrew/bin/faust", "/usr/local/bin/faust"]
    }
    #[cfg(target_os = "windows")]
    {
        &[r"C:\Program Files\Faust\bin\faust.exe"]
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        &[]
    }
}

/// Locates the `faust` binary:
///   1. PATH via `which_like`
///   2. The host OS's standard install prefixes (see `faust_fallback_paths`)
pub fn find_faust() -> Option<PathBuf> {
    if let Some(p) = which_like("faust").filter(|p| is_executable(p)) {
        return Some(p);
    }

    for candidate in faust_fallback_paths() {
        let path = Path::new(candidate);
        if is_executable(path) {
            return Some(path.to_path_buf());
        }
    }

    None
}

/// Locates the `mojo` binary:
///   1. PATH via `which_like`
///   2. `./.venv/bin/mojo` (local venv)
///   3. `~/.modular/bin/mojo`
///   4. `~/.modular/pkg/packages.modular.com_mojo/bin/mojo`
pub fn find_mojo() -> Option<PathBuf> {
    if let Some(p) = which_like("mojo").filter(|p| is_executable(p)) {
        return Some(p);
    }

    let venv = PathBuf::from("./.venv/bin/mojo");
    if is_executable(&venv) {
        return Some(venv);
    }

    if let Some(home) = home_dir() {
        let candidates = [
            home.join(".modular/bin/mojo"),
            home.join(".modular/pkg/packages.modular.com_mojo/bin/mojo"),
        ];
        for c in &candidates {
            if is_executable(c) {
                return Some(c.clone());
            }
        }
    }

    None
}

// ── Artefact naming ───────────────────────────────────────────────────────────

/// Returns the neural library filename for the given target OS string
/// (as produced by `$CARGO_CFG_TARGET_OS`).
/// Does **not** use `cfg!()` so this function is cross-compilable.
pub fn neural_lib_filename(target_os: &str) -> &'static str {
    match target_os {
        "windows" => "neural.dll",
        "macos" => "libneural.dylib",
        _ => "libneural.so",
    }
}

// ── Header freshness ──────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub enum HeaderState {
    /// The generated header exists and is at least as new as the source.
    Fresh,
    /// The source is newer than the generated header — must recompile.
    Stale,
    /// The generated header does not exist.
    Missing,
}

/// Compares the modification times of a Faust `.dsp` source and its generated
/// `.hpp` header.
///
/// Returns:
/// - `Missing` if `hpp` does not exist.
/// - `Stale`   if `dsp` exists and is strictly newer than `hpp`.
/// - `Fresh`   otherwise (hpp is current).
pub fn header_state(dsp: &Path, hpp: &Path) -> HeaderState {
    let hpp_mtime = match std::fs::metadata(hpp).and_then(|m| m.modified()) {
        Ok(t) => t,
        Err(_) => return HeaderState::Missing,
    };

    let dsp_mtime = match std::fs::metadata(dsp).and_then(|m| m.modified()) {
        Ok(t) => t,
        Err(_) => return HeaderState::Fresh, // dsp gone — hpp is effectively current
    };

    if dsp_mtime > hpp_mtime {
        HeaderState::Stale
    } else {
        HeaderState::Fresh
    }
}

// ── Compilation ───────────────────────────────────────────────────────────────

/// Invokes the Faust transpiler to compile `dsp` → `hpp`.
///
/// `class_name` is the C++ class name passed to `-cn`.
/// `include_dir` is the path passed to `-I` (e.g. `faust-ddsp`).
///
/// Returns `Err(stderr)` if Faust fails or is not found.
pub fn compile_faust(
    dsp: &Path,
    hpp: &Path,
    class_name: &str,
    include_dir: &Path,
) -> Result<(), String> {
    let faust_bin = find_faust()
        .ok_or_else(|| "Faust transpiler not found in PATH".to_string())?;

    let out = Command::new(&faust_bin)
        .args([
            "-lang", "cpp",
            "-cn", class_name,
            "-vec",
            "-I", include_dir.to_str().unwrap_or("faust-ddsp"),
            "-i", dsp.to_str().unwrap_or_default(),
            "-o", hpp.to_str().unwrap_or_default(),
        ])
        .output()
        .map_err(|e| format!("Failed to spawn faust: {e}"))?;

    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).into_owned())
    }
}

/// Invokes the Mojo compiler to build `src` → `out` as a shared library.
///
/// Returns `Err(stderr)` if compilation fails.
pub fn compile_mojo(bin: &Path, src: &Path, out: &Path) -> Result<(), String> {
    let result = Command::new(bin)
        .args([
            "build",
            "--emit", "shared-lib",
            src.to_str().unwrap_or_default(),
            "-o", out.to_str().unwrap_or_default(),
        ])
        .output()
        .map_err(|e| format!("Failed to spawn mojo: {e}"))?;

    if result.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&result.stderr).into_owned())
    }
}

// ── Environment report ────────────────────────────────────────────────────────

pub struct EnvReport {
    pub faust_version: Option<String>,
    pub mojo_version: Option<String>,
    /// "mojo" when the Mojo toolchain is present, "rust" otherwise.
    pub neural_backend: &'static str,
    pub dsp_writable: bool,
    pub neural_writable: bool,
}

fn probe_version(bin: &Path, args: &[&str]) -> Option<String> {
    Command::new(bin)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            let raw = String::from_utf8_lossy(&o.stdout);
            raw.lines().next().unwrap_or("").trim().to_string()
        })
}

fn dir_writable(path: &str) -> bool {
    let p = Path::new(path);
    if !p.exists() {
        return std::fs::create_dir_all(p).is_ok();
    }
    // Try creating a temp file to verify write permission.
    let tmp = p.join(".write_probe");
    let ok = std::fs::write(&tmp, b"").is_ok();
    let _ = std::fs::remove_file(&tmp);
    ok
}

/// Gathers information equivalent to what `scripts/check_env.sh` used to print,
/// plus the effective neural backend.
pub fn report() -> EnvReport {
    let faust_version = find_faust()
        .and_then(|b| probe_version(&b, &["--version"]));

    let mojo_bin = find_mojo();
    let mojo_version = mojo_bin
        .as_ref()
        .and_then(|b| probe_version(b, &["--version"]));

    let neural_backend = if mojo_bin.is_some() { "mojo" } else { "rust" };

    EnvReport {
        faust_version,
        mojo_version,
        neural_backend,
        dsp_writable: dir_writable("dsp"),
        neural_writable: dir_writable("neural"),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    fn write_tmp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(name);
        std::fs::write(&p, b"x").unwrap();
        p
    }

    fn set_mtime(path: &Path, t: SystemTime) {
        filetime::set_file_mtime(path, filetime::FileTime::from_system_time(t)).unwrap();
    }

    #[test]
    fn neural_lib_filename_all_platforms() {
        assert_eq!(neural_lib_filename("linux"), "libneural.so");
        assert_eq!(neural_lib_filename("macos"), "libneural.dylib");
        assert_eq!(neural_lib_filename("windows"), "neural.dll");
        // unknown OS falls back to .so
        assert_eq!(neural_lib_filename("freebsd"), "libneural.so");
    }

    #[test]
    fn which_like_finds_cargo() {
        assert!(which_like("cargo").is_some(), "cargo must be in PATH");
    }

    #[test]
    fn header_state_missing_when_hpp_absent() {
        let dsp = write_tmp("bs_test_missing.dsp");
        let hpp = std::env::temp_dir().join("bs_test_missing_nonexistent.hpp");
        let _ = std::fs::remove_file(&hpp);
        assert_eq!(header_state(&dsp, &hpp), HeaderState::Missing);
        let _ = std::fs::remove_file(&dsp);
    }

    #[test]
    fn header_state_stale_when_dsp_newer() {
        let now = SystemTime::now();
        let dsp = write_tmp("bs_test_stale.dsp");
        let hpp = write_tmp("bs_test_stale.hpp");
        set_mtime(&hpp, now - Duration::from_secs(10));
        set_mtime(&dsp, now);
        assert_eq!(header_state(&dsp, &hpp), HeaderState::Stale);
        let _ = std::fs::remove_file(&dsp);
        let _ = std::fs::remove_file(&hpp);
    }

    #[test]
    fn header_state_fresh_when_hpp_newer() {
        let now = SystemTime::now();
        let dsp = write_tmp("bs_test_fresh.dsp");
        let hpp = write_tmp("bs_test_fresh.hpp");
        set_mtime(&dsp, now - Duration::from_secs(10));
        set_mtime(&hpp, now);
        assert_eq!(header_state(&dsp, &hpp), HeaderState::Fresh);
        let _ = std::fs::remove_file(&dsp);
        let _ = std::fs::remove_file(&hpp);
    }

    #[test]
    fn home_dir_returns_some() {
        assert!(home_dir().is_some(), "HOME / USERPROFILE must be set");
    }

    #[test]
    fn is_executable_false_for_missing_path() {
        let p = std::env::temp_dir().join("bs_test_does_not_exist_xyz");
        let _ = std::fs::remove_file(&p);
        assert!(!is_executable(&p));
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    fn faust_fallback_paths_are_absolute() {
        let paths = faust_fallback_paths();
        assert!(!paths.is_empty(), "supported platforms must list a prefix");
        for p in paths {
            assert!(
                Path::new(p).is_absolute(),
                "fallback {p} must be absolute — it is probed without a PATH lookup"
            );
        }
    }

    #[test]
    #[cfg(unix)]
    fn is_executable_respects_permission_bits() {
        use std::os::unix::fs::PermissionsExt;

        let p = write_tmp("bs_test_exec_perm.sh");
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(!is_executable(&p), "file without exec bit must not be executable");

        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
        assert!(is_executable(&p), "file with exec bit must be executable");

        let _ = std::fs::remove_file(&p);
    }
}
