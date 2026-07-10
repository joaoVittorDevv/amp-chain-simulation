use std::path::PathBuf;
use std::process::Command;

use build_support::{compile_faust, compile_mojo, find_faust, find_mojo, header_state, HeaderState};

// ── Verb dispatch ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum Verb {
    CheckEnv,
    PreBuild,
    Build,
    Run,
    Clean,
    Bundle,
    /// Unknown verb: delegate to nih_plug_xtask::main_with_args.
    Delegate,
}

fn parse_verb(arg: &str) -> Verb {
    match arg {
        "check-env" => Verb::CheckEnv,
        "pre-build" => Verb::PreBuild,
        "build" => Verb::Build,
        "run" => Verb::Run,
        "clean" => Verb::Clean,
        "bundle" => Verb::Bundle,
        _ => Verb::Delegate,
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() -> nih_plug_xtask::Result<()> {
    nih_plug_xtask::chdir_workspace_root()?;

    let mut args: Vec<String> = std::env::args().skip(1).collect();

    let verb = args.first().map(|s| parse_verb(s)).unwrap_or(Verb::Delegate);

    match verb {
        Verb::CheckEnv => {
            args.remove(0);
            cmd_check_env();
            Ok(())
        }
        Verb::PreBuild => {
            args.remove(0);
            cmd_pre_build();
            Ok(())
        }
        Verb::Build => {
            args.remove(0);
            cmd_pre_build();
            // Forward any remaining args (e.g. `--features asio`) to cargo.
            let mut cargo_args: Vec<&str> = vec!["build", "--release"];
            cargo_args.extend(args.iter().map(String::as_str));
            run_cargo(&cargo_args);
            Ok(())
        }
        Verb::Run => {
            args.remove(0);
            cmd_pre_build();
            // Forward any remaining args (e.g. `--features asio`) to cargo.
            cmd_run(&args);
            Ok(())
        }
        Verb::Clean => {
            args.remove(0);
            cmd_clean();
            Ok(())
        }
        Verb::Bundle => {
            std::env::set_var("DISTORTION_FORCE_RUST_NEURAL", "1");
            nih_plug_xtask::main_with_args("cargo xtask", args)
        }
        Verb::Delegate => {
            // Includes `bundle-universal` and any unknown verb.
            nih_plug_xtask::main_with_args("cargo xtask", args)
        }
    }
}

// ── check-env ────────────────────────────────────────────────────────────────

fn cmd_check_env() {
    let r = build_support::report();

    match &r.faust_version {
        Some(v) => println!("[OK]  Faust: {}", v),
        None => eprintln!("[WARN] Faust not found in PATH"),
    }

    match &r.mojo_version {
        Some(v) => println!("[OK]  Mojo: {}", v),
        None => println!("[INFO] Mojo not found — will use Rust neural backend"),
    }

    println!(
        "[INFO] Neural backend: {}",
        r.neural_backend
    );

    if r.dsp_writable {
        println!("[OK]  dsp/ is writable");
    } else {
        eprintln!("[ERR]  dsp/ is not writable");
    }

    if r.neural_writable {
        println!("[OK]  neural/ is writable");
    } else {
        eprintln!("[ERR]  neural/ is not writable");
    }
}

// ── pre-build ────────────────────────────────────────────────────────────────

fn cmd_pre_build() {
    let dsp_dir = PathBuf::from("dsp");
    let include_dir = PathBuf::from("faust-ddsp");

    compile_dsp_if_needed(
        &dsp_dir.join("main.dsp"),
        &dsp_dir.join("FaustModule.hpp"),
        "mydsp",
        &include_dir,
    );
    compile_dsp_if_needed(
        &dsp_dir.join("mlc_zero_v.dsp"),
        &dsp_dir.join("MlcZeroVModule.hpp"),
        "mlczerov",
        &include_dir,
    );
    compile_mojo_if_needed();
}

fn compile_dsp_if_needed(dsp: &std::path::Path, hpp: &std::path::Path, class_name: &str, include_dir: &std::path::Path) {
    if !dsp.exists() {
        return;
    }
    if find_faust().is_none() {
        match header_state(dsp, hpp) {
            HeaderState::Fresh => {
                println!("[INFO] Faust not found; using versioned {} (up to date)", hpp.display());
            }
            HeaderState::Stale | HeaderState::Missing => {
                eprintln!(
                    "[WARN] {} is stale/missing and Faust is not installed. Install from https://faust.grame.fr/",
                    hpp.display()
                );
            }
        }
        return;
    }
    if header_state(dsp, hpp) != HeaderState::Fresh {
        println!("[INFO] Compiling {} -> {}", dsp.display(), hpp.display());
        if let Err(e) = compile_faust(dsp, hpp, class_name, include_dir) {
            eprintln!("[ERR] Faust compilation failed:\n{}", e);
            std::process::exit(1);
        }
    }
}

fn compile_mojo_if_needed() {
    let src = PathBuf::from("neural/main.mojo");
    if !src.exists() {
        return;
    }
    let Some(bin) = find_mojo() else { return };

    let target_os = std::env::consts::OS;
    let lib_name = build_support::neural_lib_filename(target_os);
    let out = PathBuf::from(format!("neural/{}", lib_name));

    let needs_rebuild = !out.exists()
        || std::fs::metadata(&src)
            .and_then(|m| m.modified())
            .ok()
            .zip(std::fs::metadata(&out).and_then(|m| m.modified()).ok())
            .map(|(s, o)| s > o)
            .unwrap_or(true);

    if needs_rebuild {
        println!("[INFO] Compiling neural/main.mojo -> neural/{}", lib_name);
        if let Err(e) = compile_mojo(&bin, &src, &out) {
            eprintln!("[ERR] Mojo compilation failed:\n{}", e);
            std::process::exit(1);
        }
    }
}

// ── run ──────────────────────────────────────────────────────────────────────

fn cmd_run(extra: &[String]) {
    let neural_dir = std::fs::canonicalize("neural")
        .unwrap_or_else(|_| PathBuf::from("neural"))
        .to_string_lossy()
        .into_owned();

    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--release", "--bin", "standalone"]);
    cmd.args(extra);

    #[cfg(target_os = "linux")]
    {
        let existing = std::env::var("LD_LIBRARY_PATH").unwrap_or_default();
        let new_val = if existing.is_empty() {
            neural_dir.clone()
        } else {
            format!("{}:{}", neural_dir, existing)
        };
        cmd.env("LD_LIBRARY_PATH", new_val);
    }
    #[cfg(target_os = "macos")]
    {
        let existing = std::env::var("DYLD_LIBRARY_PATH").unwrap_or_default();
        let new_val = if existing.is_empty() {
            neural_dir.clone()
        } else {
            format!("{}:{}", neural_dir, existing)
        };
        cmd.env("DYLD_LIBRARY_PATH", new_val);
    }
    #[cfg(target_os = "windows")]
    {
        let existing = std::env::var("PATH").unwrap_or_default();
        let new_val = format!("{};{}", neural_dir, existing);
        cmd.env("PATH", new_val);
    }

    let status = cmd.status().expect("Failed to run cargo");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

// ── clean ─────────────────────────────────────────────────────────────────────

fn cmd_clean() {
    // Remove generated Faust headers
    for entry in std::fs::read_dir("dsp").unwrap_or_else(|_| {
        std::fs::read_dir(".").unwrap()
    }) {
        if let Ok(e) = entry {
            let path = e.path();
            if path.extension().and_then(|s| s.to_str()) == Some("hpp") {
                println!("[CLEAN] Removing {}", path.display());
                let _ = std::fs::remove_file(&path);
            }
        }
    }

    // Remove neural artifacts
    for name in &["libneural.so", "libneural.dylib", "neural.dll"] {
        let path = PathBuf::from(format!("neural/{}", name));
        if path.exists() {
            println!("[CLEAN] Removing {}", path.display());
            let _ = std::fs::remove_file(&path);
        }
    }

    run_cargo(&["clean"]);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn run_cargo(args: &[&str]) {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(&cargo)
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("Failed to run cargo {}: {}", args.join(" "), e));
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verb_parser_maps_known_verbs() {
        assert_eq!(parse_verb("check-env"), Verb::CheckEnv);
        assert_eq!(parse_verb("pre-build"), Verb::PreBuild);
        assert_eq!(parse_verb("build"), Verb::Build);
        assert_eq!(parse_verb("run"), Verb::Run);
        assert_eq!(parse_verb("clean"), Verb::Clean);
        assert_eq!(parse_verb("bundle"), Verb::Bundle);
    }

    #[test]
    fn verb_parser_delegates_unknown() {
        assert_eq!(parse_verb("bundle-universal"), Verb::Delegate);
        assert_eq!(parse_verb("list-plugins"), Verb::Delegate);
        assert_eq!(parse_verb(""), Verb::Delegate);
        assert_eq!(parse_verb("unknown-verb"), Verb::Delegate);
    }
}
