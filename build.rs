use std::env;
use std::path::PathBuf;
use std::process::Command;

fn find_mojo_path() -> Option<PathBuf> {
    // 1. Tenta no PATH
    if let Ok(output) = Command::new("which").arg("mojo").output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Some(PathBuf::from(path_str));
        }
    }

    // 2. Tenta caminhos padrão do Modular e venv local
    let home = env::var("HOME").ok().unwrap_or_default();
    let common_paths = vec![
        "./.venv/bin/mojo".to_string(),
        format!("{}/.modular/pkg/packages.modular.com_mojo/bin/mojo", home),
        format!("{}/.modular/bin/mojo", home),
    ];

    for path in common_paths {
        let pb = PathBuf::from(path);
        if pb.exists() {
            return Some(pb);
        }
    }
    None
}

fn pre_build_check() {
    // Validação Faust
    let faust_exists = Command::new("faust").arg("--version").output().is_ok();
    if !faust_exists {
        panic!("\n\n❌ ERRO: Transpilador Faust não encontrado.\nPor favor, instale o Faust (https://faust.grame.fr/) para continuar.\n\n");
    }

    // Validação Mojo (Busca Inteligente)
    let mojo_bin = find_mojo_path().expect("\n\n❌ ERRO: Compilador Mojo não encontrado.\nCertifique-se de que o Mojo está instalado e acessível (https://www.modular.com/mojo).\n\n");
    
    // Configura o link path do Mojo dinamicamente
    if let Some(mojo_dir) = mojo_bin.parent() {
        if let Some(lib_dir) = mojo_dir.parent().map(|p| p.join("lib")) {
            if lib_dir.exists() {
                println!("cargo:rustc-link-search=native={}", lib_dir.display());
            }
        }
    }
}

fn main() {
    pre_build_check();

    println!("cargo:rerun-if-changed=dsp/wrapper.cpp");
    println!("cargo:rerun-if-changed=dsp/wrapper.h");
    println!("cargo:rerun-if-changed=dsp/main.dsp");
    println!("cargo:rerun-if-changed=neural/main.mojo");

    let dsp_dir = PathBuf::from("dsp");
    let main_dsp = dsp_dir.join("main.dsp");
    let wrapper_h = dsp_dir.join("wrapper.h");

    // Rebuild automático do Faust se .dsp existir e for alterado
    if main_dsp.exists() {
        let hpp_file = dsp_dir.join("FaustModule.hpp");
        let should_rebuild = !hpp_file.exists() || 
            std::fs::metadata(&main_dsp).unwrap().modified().unwrap() > 
            std::fs::metadata(&hpp_file).unwrap().modified().unwrap();

        if should_rebuild {
            println!("cargo:warning=Recompilando Faust (.dsp -> .hpp)...");
            let status = Command::new("faust")
                .args(&["-lang", "cpp", "-vec", "-I", "faust-ddsp", "-i", "dsp/main.dsp", "-o", "dsp/FaustModule.hpp"])
                .status()
                .expect("Falha ao executar o compilador Faust.");
            
            if !status.success() {
                panic!("Erro na transpilação do arquivo Faust (main.dsp).");
            }
        }
    }

    // Rebuild automático do Mojo se .mojo existir e for alterado
    let main_mojo = PathBuf::from("neural/main.mojo");
    let lib_so = PathBuf::from("neural/libneural.so");
    if main_mojo.exists() {
        let should_rebuild = !lib_so.exists() || 
            std::fs::metadata(&main_mojo).unwrap().modified().unwrap() > 
            std::fs::metadata(&lib_so).unwrap().modified().unwrap();

        if should_rebuild {
            println!("cargo:warning=Recompilando Mojo (.mojo -> .so)...");
            let mojo_bin = find_mojo_path().unwrap();
            let status = Command::new(mojo_bin)
                .args(&["build", "--emit", "shared-lib", "neural/main.mojo", "-o", "neural/libneural.so"])
                .status()
                .expect("Falha ao executar o compilador Mojo.");
            
            if !status.success() {
                panic!("Erro na compilação do arquivo Mojo (main.mojo).");
            }
        }
    }

    // 1. Compila o Wrapper C++
    cc::Build::new()
        .cpp(true)
        .file(dsp_dir.join("wrapper.cpp"))
        .opt_level(3)
        .compile("faust_dsp");

    // 2. Roda o Bindgen
    let bindings = bindgen::Builder::default()
        .header(wrapper_h.to_str().unwrap())
        .allowlist_function("faust_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Não foi possível gerar os bindings do Faust.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings_faust.rs"))
        .expect("Não foi possível escrever os bindings.");

    // 3. Linking do Mojo
    println!("cargo:rustc-link-search=native={}/neural", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-lib=dylib=neural");
}

