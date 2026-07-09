use std::env;
use std::path::PathBuf;

use build_support::{compile_faust, compile_mojo, find_mojo, header_state, HeaderState};

fn main() {
    // Allow forced Rust neural backend (used by xtask bundle, CROSS-24).
    println!("cargo:rerun-if-env-changed=DISTORTION_FORCE_RUST_NEURAL");

    println!("cargo:rerun-if-changed=dsp/wrapper.cpp");
    println!("cargo:rerun-if-changed=dsp/wrapper.h");
    println!("cargo:rerun-if-changed=dsp/main.dsp");
    println!("cargo:rerun-if-changed=dsp/mlc_zero_v.dsp");
    println!("cargo:rerun-if-changed=dsp/mlc_zero_v_wrapper.cpp");
    println!("cargo:rerun-if-changed=dsp/mlc_zero_v_wrapper.h");
    println!("cargo:rerun-if-changed=neural/main.mojo");

    let dsp_dir = PathBuf::from("dsp");
    let include_dir = PathBuf::from("faust-ddsp");

    // Declared unconditionally so `#[cfg(have_mojo)]` never triggers the
    // `unexpected_cfgs` lint, regardless of whether the cfg is emitted below.
    println!("cargo::rustc-check-cfg=cfg(have_mojo)");

    // ── Faust: main DSP ───────────────────────────────────────────────────────
    let main_dsp = dsp_dir.join("main.dsp");
    let main_hpp = dsp_dir.join("FaustModule.hpp");
    compile_dsp(&main_dsp, &main_hpp, "mydsp", &include_dir);

    // ── Faust: MLC ZERO V ────────────────────────────────────────────────────
    let mlc_dsp = dsp_dir.join("mlc_zero_v.dsp");
    let mlc_hpp = dsp_dir.join("MlcZeroVModule.hpp");
    compile_dsp(&mlc_dsp, &mlc_hpp, "mlczerov", &include_dir);

    // ── Mojo: neural shared library ───────────────────────────────────────────
    let mojo_bin = find_mojo();
    let force_rust = env::var("DISTORTION_FORCE_RUST_NEURAL").is_ok();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let lib_name = build_support::neural_lib_filename(&target_os);
    let neural_out = PathBuf::from(format!("neural/{}", lib_name));

    if !force_rust {
        if let Some(ref bin) = mojo_bin {
            let src = PathBuf::from("neural/main.mojo");

            let needs_rebuild = !neural_out.exists()
                || std::fs::metadata(&src)
                    .and_then(|m| m.modified())
                    .ok()
                    .zip(
                        std::fs::metadata(&neural_out)
                            .and_then(|m| m.modified())
                            .ok(),
                    )
                    .map(|(s, o)| s > o)
                    .unwrap_or(true);

            if needs_rebuild {
                println!("cargo:warning=Recompilando Mojo (.mojo -> {})...", lib_name);
                if let Err(stderr) = compile_mojo(bin, &src, &neural_out) {
                    panic!("Erro na compilação do Mojo (main.mojo):\n{}", stderr);
                }
            }
        }
    }

    // `have_mojo` requires the toolchain to have been found, not forced off,
    // AND the artifact to actually exist on disk — never inferred from intent.
    let have_mojo = !force_rust && mojo_bin.is_some() && neural_out.exists();

    // ── C++ wrapper compilation ───────────────────────────────────────────────
    cc::Build::new()
        .cpp(true)
        .file(dsp_dir.join("wrapper.cpp"))
        .opt_level(3)
        .warnings(false)
        .compile("faust_dsp");

    cc::Build::new()
        .cpp(true)
        .file(dsp_dir.join("mlc_zero_v_wrapper.cpp"))
        .opt_level(3)
        .warnings(false)
        .compile("mlc_zero_v_dsp");

    // ── Bindgen ───────────────────────────────────────────────────────────────
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header(dsp_dir.join("wrapper.h").to_str().unwrap())
        .allowlist_function("faust_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Não foi possível gerar os bindings do Faust.");
    bindings
        .write_to_file(out_path.join("bindings_faust.rs"))
        .expect("Não foi possível escrever os bindings.");

    let mlc_bindings = bindgen::Builder::default()
        .header(dsp_dir.join("mlc_zero_v_wrapper.h").to_str().unwrap())
        .allowlist_function("mlc_zero_v_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Não foi possível gerar os bindings do MLC ZERO V.");
    mlc_bindings
        .write_to_file(out_path.join("bindings_mlc_zero_v.rs"))
        .expect("Não foi possível escrever os bindings do MLC ZERO V.");

    // ── Neural library linking ────────────────────────────────────────────────
    let neural_dir = format!("{}/neural", env::var("CARGO_MANIFEST_DIR").unwrap());

    if have_mojo {
        println!("cargo::rustc-cfg=have_mojo");
        println!("cargo:rustc-link-search=native={}", neural_dir);
        println!("cargo:rustc-link-lib=neural");

        if target_os == "linux" || target_os == "macos" {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", neural_dir);
        }
    }
}

/// Transpile a Faust `.dsp` to `.hpp`, respecting the freshness check.
///
/// - If Faust is absent and the hpp is `Fresh`: emit a warning and continue.
/// - If Faust is absent and the hpp is `Stale`: panic (stale header would compile wrong DSP).
/// - If Faust is absent and the hpp is `Missing`: panic with install instructions.
/// - If Faust is present and the hpp is not `Fresh`: compile.
fn compile_dsp(
    dsp: &std::path::Path,
    hpp: &std::path::Path,
    class_name: &str,
    include_dir: &std::path::Path,
) {
    if !dsp.exists() {
        return;
    }

    let state = header_state(dsp, hpp);

    if build_support::find_faust().is_none() {
        match state {
            HeaderState::Fresh => {
                println!(
                    "cargo:warning=Faust não encontrado; usando {} versionado (atualizado).",
                    hpp.display()
                );
                return;
            }
            HeaderState::Stale => {
                panic!(
                    "\n\nERRO: {} está desatualizado face a {} e o Faust não está instalado.\n\
                     Instale o Faust (https://faust.grame.fr/) e rode novamente.\n\n",
                    hpp.display(),
                    dsp.display()
                );
            }
            HeaderState::Missing => {
                panic!(
                    "\n\nERRO: {} não encontrado e Faust não está instalado.\n\
                     Instale o Faust (https://faust.grame.fr/) e rode novamente.\n\n",
                    hpp.display()
                );
            }
        }
    }

    // Faust is available — only recompile when needed.
    if state != HeaderState::Fresh {
        println!("cargo:warning=Recompilando Faust ({} -> {})...", dsp.display(), hpp.display());
        if let Err(e) = compile_faust(dsp, hpp, class_name, include_dir) {
            panic!("Erro na transpilação do Faust ({}):\n{}", dsp.display(), e);
        }
    }
}
