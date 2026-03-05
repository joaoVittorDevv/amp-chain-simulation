#!/usr/bin/env python3
import os
import sys
import subprocess
import re
import uuid


def is_protected_repo():
    # Segurança: Bloqueia execução se o diretório tiver o nome da base original
    current_dir = os.path.basename(os.path.abspath(".")).lower()
    protected_names = ["baseio", "baseio-plug", "channelstrip", "channel_strip"]
    return current_dir in protected_names


def format_snake_case(name):
    s1 = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    s2 = re.sub("([a-z0-9])([A-Z])", r"\1_\2", s1).lower()
    return re.sub(r"[^a-z0-9]+", "_", s2).strip("_")


def run_command(cmd, shell=False):
    print(f"Executando: {' '.join(cmd) if isinstance(cmd, list) else cmd}")
    subprocess.run(cmd, shell=shell, check=True)


def main():
    print("==================================================")
    print("🚀 BaseIO - Bootstrap de Novo Plugin")
    print("==================================================")

    if is_protected_repo():
        print(
            "\n⚠️ ERRO FATAL: Você está executando este script dentro do repositório baseIO original ou com remote configurado para o baseIO."
        )
        print("Para proteger o template raiz, a operação foi abortada.")
        print("CLONE o repositório em uma nova pasta primeiro e rode a partir dali.")
        sys.exit(1)

    print("\nVamos configurar o seu novo Plugin!")
    plugin_name = input(
        "1. Nome do Plugin (ex: Vintage EQ, My Super Compressor): "
    ).strip()
    if not plugin_name:
        print("O nome não pode ser vazio.")
        sys.exit(1)

    vendor_name = input("2. Nome do Vendor/Criador (ex: Seu Nome ou Empresa): ").strip()
    vendor_id = input(
        f"3. Vendor ID Base (ex: com.{format_snake_case(vendor_name)}): "
    ).strip()

    print("\nProcessando Identificadores...")

    package_name = format_snake_case(plugin_name)
    clap_id = f"{vendor_id}.{package_name.replace('_', '-')}"

    # Geração Segura do VST3 ID
    u = uuid.uuid4().bytes
    vst3_array_str = "[" + ", ".join([f"0x{b:02X}" for b in u]) + "]"

    print(f"📦 Package (Cargo): {package_name}")
    print(f"🔌 CLAP ID:         {clap_id}")
    print(f"🎸 VST3 ID Array:   {vst3_array_str}")

    print("\nAplicando Substituições [Search & Replace]...")

    # 1. Cargo.toml
    with open("Cargo.toml", "r", encoding="utf-8") as f:
        cargo_content = f.read()
    cargo_content = cargo_content.replace(
        'name = "base_io"', f'name = "{package_name}"'
    )
    with open("Cargo.toml", "w", encoding="utf-8") as f:
        f.write(cargo_content)

    # 2. bundler.toml
    with open("bundler.toml", "r", encoding="utf-8") as f:
        bundler_content = f.read()
    bundler_content = bundler_content.replace("[base_io]", f"[{package_name}]")
    bundler_content = bundler_content.replace(
        'name = "BaseIO"', f'name = "{plugin_name}"'
    )
    with open("bundler.toml", "w", encoding="utf-8") as f:
        f.write(bundler_content)

    # 3. Makefile
    if os.path.exists("Makefile"):
        with open("Makefile", "r", encoding="utf-8") as f:
            makefile_content = f.read()
        makefile_content = makefile_content.replace(
            "bundle base_io", f"bundle {package_name}"
        )
        with open("Makefile", "w", encoding="utf-8") as f:
            f.write(makefile_content)

    # 4. src/lib.rs
    lib_path = os.path.join("src", "lib.rs")
    with open(lib_path, "r", encoding="utf-8") as f:
        lib_content = f.read()

    lib_content = lib_content.replace(
        'pub const APP_NAME: &str = "BaseIO";',
        f'pub const APP_NAME: &str = "{plugin_name}";',
    )
    lib_content = lib_content.replace(
        'pub const APP_ID: &str = "base_io";',
        f'pub const APP_ID: &str = "{package_name}";',
    )
    lib_content = lib_content.replace(
        'pub const VENDOR: &str = "jao";', f'pub const VENDOR: &str = "{vendor_name}";'
    )
    lib_content = lib_content.replace(
        'pub const CLAP_ID: &str = "com.jao.base-io";',
        f'pub const CLAP_ID: &str = "{clap_id}";',
    )
    lib_content = lib_content.replace(
        'pub const VST3_ID: [u8; 16] = *b"BaseIOTemplate26";',
        f"pub const VST3_ID: [u8; 16] = {vst3_array_str};",
    )

    with open(lib_path, "w", encoding="utf-8") as f:
        f.write(lib_content)

    # 5. src/bin/standalone.rs
    standalone_path = os.path.join("src", "bin", "standalone.rs")
    if os.path.exists(standalone_path):
        with open(standalone_path, "r", encoding="utf-8") as f:
            std_content = f.read()
        std_content = std_content.replace("use base_io::", f"use {package_name}::")
        with open(standalone_path, "w", encoding="utf-8") as f:
            f.write(std_content)

    print("✅ Código Base Atualizado.")

    print("\n🧹 Resetando ambiente Git...")
    # Tentativa de deletar .git e refazer
    os.system("rm -rf .git")
    run_command(["git", "init"])
    run_command(["git", "add", "."])
    # Tenta commitar (se houver usuário git global configurado no sistema de destino)
    os.system('git commit -m "Iniciando projeto a partir do boilerplate BaseIO"')

    print(
        "\n🛠️ Rodando o primeiro Cargo Build para validar (Isso pode demorar um pouco)..."
    )
    try:
        run_command(["cargo", "build", "--bin", "standalone"])
        print("\n✅ Build validado com sucesso! O seu projeto está saudável.")
    except subprocess.CalledProcessError:
        print("\n⚠️ O Build encontrou problemas! Verifique os logs do cargo acima.")

    print("\n💣 Auto-destruição iniciada...")
    script_path = os.path.abspath(__file__)
    os.remove(script_path)

    current_dir = os.path.basename(os.path.abspath("."))
    print(
        f"\n🎉 Tudo pronto! Bem-vindo ao seu novo diretório de desenvolvimento: {current_dir}"
    )
    print(f"🎵 Plugin: {plugin_name} inicializado com IDs únicos.")


if __name__ == "__main__":
    main()
