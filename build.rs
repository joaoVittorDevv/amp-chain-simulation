use std::env;
use std::path::PathBuf;

fn main() {
    // Diz para o cargo re-rodar esse script se os arquivos mudarem
    println!("cargo:rerun-if-changed=dsp/main.dsp");
    println!("cargo:rerun-if-changed=dsp/FaustModule.hpp");
    println!("cargo:rerun-if-changed=neural/main.mojo");

    let dsp_dir = PathBuf::from("dsp");
    let hpp_file = dsp_dir.join("FaustModule.hpp");

    // Compila o código C++ gerado pelo Faust, se existir
    if hpp_file.exists() {
        println!("cargo:warning=Header Faust encontrado. Se precisar compilar C++, configure o cc::Build no build.rs");
        // Exemplo simplificado de como compilar C++ gerado caso crie um wrapper:
        /*
        cc::Build::new()
            .cpp(true)
            .file("dsp/wrapper.cpp") // Precisaria de um wrapper.cpp para interagir com o hpp
            .compile("faust_dsp");
        */
    }

    // Configurando linking do Mojo
    let neural_dir = PathBuf::from("neural");
    let neural_abs_path = env::current_dir().unwrap().join(&neural_dir);

    // Instruir o Rust a buscar bibliotecas dinâmicas na pasta neural
    println!("cargo:rustc-link-search=native={}", neural_abs_path.display());

    // Fazer link explícito com a libneural gerada (libneural.so, etc.)
    if neural_abs_path.join("libneural.so").exists() || neural_abs_path.join("libneural.dylib").exists() {
        println!("cargo:rustc-link-lib=dylib=neural");
    } else {
        println!("cargo:warning=Biblioteca Mojo (libneural) não encontrada. Pule o linking se estiver rodando o make inicial.");
    }
}
