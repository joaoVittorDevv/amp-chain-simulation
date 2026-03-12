# 🎸 Distortion Plugin — Guia de Desenvolvimento de Contexto Zero

> **Este documento é o manual de operações do projeto.**  
> Todo agente ou desenvolvedor que assumir o projeto deve começar por aqui.

---

## Índice

1. [Stack Tecnológica e Arquitetura](#1-stack-tecnológica-e-arquitetura)
2. [Fluxos de Build (Dual-Target)](#2-fluxos-de-build-dual-target)
3. [Guia de Implementação](#3-guia-de-implementação)
4. [Ponte FFI Zero-Copy](#4-ponte-ffi-zero-copy)
5. [Troubleshooting](#5-troubleshooting)
6. [Instalação do Ambiente](#6-instalação-do-ambiente)

---

## 1. Stack Tecnológica e Arquitetura

O projeto combina três tecnologias em um único pipeline de processamento de áudio em tempo-real:

```
┌─────────────────────────────────────────────────────────┐
│                   HOST (DAW / Standalone)                │
└─────────────────────────┬───────────────────────────────┘
                          │ nih-plug (Rust)
          ┌───────────────▼────────────────┐
          │        Orquestrador de Estado  │
          │      src/bridge/mod.rs         │
          │  (Trait ExternalProcessor)     │
          └───────┬──────────────┬─────────┘
                  │ FFI (C ABI)  │ FFI (C ABI)
      ┌───────────▼──┐     ┌─────▼──────────────┐
      │  Faust (C++) │     │    Mojo (MAX SDK)   │
      │  dsp/main.dsp│     │  neural/main.mojo   │
      │  ──────────  │     │  ──────────────────  │
      │  DSP linear  │     │  Processamento neural│
      │  (EQ, dist.) │     │  alta performance    │
      └──────────────┘     └────────────────────-┘
```

### 1.1 Rust + `nih_plug` — Orquestrador

- **Papel**: Gerencia o ciclo de vida do plugin (parâmetros, janela de UI via `egui`, callbacks do host).
- **Ponto de entrada de áudio**: O buffer vindo do host é entregue como `*mut f32` ao método `process_block` de cada processador.
- **Trait central** (`src/bridge/mod.rs`):
  ```rust
  pub trait ExternalProcessor {
      fn init(&mut self, sample_rate: f32);
      /// Processa in-place: recebe ponteiro bruto + tamanho
      fn process_block(&mut self, buffer: *mut f32, length: usize);
  }
  ```
- **Dependências-chave** (`Cargo.toml`): `nih_plug` (git), `nih_plug_egui` (git), `tract-onnx`, `cpal`, `ringbuf`, `rtrb`.

### 1.2 Faust (`dsp/main.dsp`) — Backend DSP Matemático

- **Papel**: Define o processamento de sinal linear (distorção, EQ, filtros) usando a linguagem de domínio específico do Faust.
- **Integração**: O `build.rs` transpila `dsp/main.dsp → dsp/FaustModule.hpp` automaticamente via `faust -lang cpp`.
- **Bibliotecas Externas**: Suporte para `faust-ddsp` (Differentiable Digital Signal Processing) integrado. A biblioteca está localizada em `faust-ddsp/` e é incluída automaticamente no path de busca do compilador Faust.
- **Wrapper C**: O arquivo `dsp/wrapper.cpp` + `dsp/wrapper.h` expõem as funções do Faust como símbolos C puros para o Rust via `bindgen`.
- **API pública** (`dsp/wrapper.h`):
  ```c
  typedef void* FaustHandle;

  FaustHandle faust_create();
  void        faust_init(FaustHandle handle, float sample_rate);
  void        faust_process(FaustHandle handle, float* buffer, f_size_t length);
  void        faust_destroy(FaustHandle handle);
  ```
- **Compilação**: O `cc::Build` no `build.rs` compila `wrapper.cpp` como uma biblioteca estática `faust_dsp` linkada diretamente no binário Rust.

### 1.3 Mojo (`neural/main.mojo`) — Processamento Neural High-Performance

- **Papel**: Executa o processamento neural (inferência em tempo-real) via FFI com zero-copy de memória.
- **Artefato gerado**: `neural/libneural.so` (biblioteca dinâmica carregada em runtime pelo linker).
- **Declaração das funções no Rust** (`src/bridge/mojo.rs`):
  ```rust
  extern "C" {
      fn mojo_init(sample_rate: f32);
      fn mojo_process_block(address: usize, size: usize);
  }
  ```

---

## 2. Fluxos de Build (Dual-Target)

> **Regra de Ouro**: Toda alteração no processamento de áudio **deve ser validada em modo Standalone** antes de gerar o bundle final para DAW.

### 2.1 Standalone — Debug e Desenvolvimento Rápido

```bash
make run
```

**O que acontece internamente:**
1. `make check-env` → executa `scripts/check_env.sh` para validar as dependências do sistema.
2. `make build-faust` → transpila `dsp/main.dsp` para `dsp/FaustModule.hpp` se houver alterações.
3. `make build-mojo` → compila `neural/main.mojo` para `neural/libneural.so`.
4. `./scripts/run_standalone.sh` → inicia o host standalone com ALSA.

### 2.2 VST3 / CLAP — Distribuição para DAWs

```bash
make bundle
```

**O que acontece internamente:**
1. Executa o mesmo `pre-build` (check-env → faust → mojo).
2. `cargo xtask bundle distortion --release` → empacota o plugin nos formatos VST3 e CLAP.

### 2.3 Compilação Apenas (sem executar)

```bash
make build
# Equivale a: cargo build --release
```

### 2.4 Limpeza Total

```bash
make clean
# Remove: target/, dsp/*.hpp, dsp/*.cpp gerados, neural/*.so
```

### 2.5 Diagrama do Fluxo de Build

```
make run / make bundle
       │
       ▼
  check-env (scripts/check_env.sh)
       │
       ├──► build-faust
       │         └── faust -lang cpp -I faust-ddsp -i dsp/main.dsp -o dsp/FaustModule.hpp
       │
       ├──► build-mojo
       │         └── mojo build --emit shared-lib neural/main.mojo -o neural/libneural.so
       │
       └──► cargo build / cargo xtask bundle
                    └── build.rs:
                         ├── cc::Build  → compila wrapper.cpp → libfaust_dsp.a
                         ├── bindgen    → gera bindings_faust.rs
                         └── rustc-link-lib=dylib=neural → linka libneural.so
```

---

## 3. Guia de Implementação

### 3.1 Desenvolvimento em Mojo (`neural/main.mojo`)

#### ⚠️ Padrão Address Bypass (OBRIGATÓRIO)

O Mojo 0.26+ **não aceita `UnsafePointer` como parâmetro direto de funções `@export`** devido a restrições de parametricidade. A solução é receber o ponteiro como `Int` (endereço de memória) e reconstruí-lo internamente.

**Exemplo canônico do projeto:**

```mojo
from std.memory import UnsafePointer

@export
fn mojo_process_block(address: Int, size: Int):
    # Reconstrói o ponteiro mutável a partir do endereço recebido
    var data = UnsafePointer[Float32, MutAnyOrigin](unsafe_from_address=address)
    
    for var i in range(size):
        data[i] = data[i] * 0.5  # Processa in-place

@export
fn mojo_init(sample_rate: Float64):
    pass
```

#### Regras para arquivos de biblioteca Mojo

| ✅ Correto | ❌ Errado |
|---|---|
| `@export fn mojo_process_block(...)` | `@extern_c fn mojo_process_block(...)` |
| `fn mojo_process_block(address: Int, ...)` | `fn mojo_process_block(ptr: UnsafePointer[...], ...)` |
| Sem `fn main()` no arquivo | Ter `fn main()` em bibliotecas dinâmicas |
| `from std.memory import UnsafePointer` | `import UnsafePointer` sem qualificação |

> **Use sempre `@export`** em vez de `@extern_c` para gerar símbolos acessíveis por FFI em bibliotecas dinâmicas (`.so`).

### 3.2 Desenvolvimento em Faust (`dsp/main.dsp`)

#### Ciclo de Vida

O `build.rs` monitora `dsp/main.dsp` e regenera `dsp/FaustModule.hpp` automaticamente quando detecta modificações:

```rust
// build.rs — lógica de rebuild automático
println!("cargo:rerun-if-changed=dsp/main.dsp");

let should_rebuild = !hpp_file.exists() || 
    fs::metadata(&main_dsp).unwrap().modified().unwrap() > 
    fs::metadata(&hpp_file).unwrap().modified().unwrap();

if should_rebuild {
    Command::new("faust")
        .args(&["-lang", "cpp", "-i", "dsp/main.dsp", "-o", "dsp/FaustModule.hpp"])
        .status().expect("Falha ao executar o compilador Faust.");
}
```

#### Sincronização do Wrapper

Ao adicionar ou remover parâmetros no `dsp/main.dsp`, o **`dsp/wrapper.cpp` deve ser atualizado** para expor os novos parâmetros ao Rust. O `wrapper.h` define o contrato ABI:

```c
// dsp/wrapper.h — contrato C entre Faust e Rust
FaustHandle faust_create();
void        faust_init(FaustHandle handle, float sample_rate);
void        faust_process(FaustHandle handle, float* buffer, f_size_t length);
void        faust_destroy(FaustHandle handle);
```

O `bindgen` gera `bindings_faust.rs` a partir desse header e o inclui no Rust via:
```rust
include!(concat!(env!("OUT_DIR"), "/bindings_faust.rs"));
```

### 3.3 Uso da Biblioteca `faust-ddsp` (`diff.lib`)

A biblioteca `faust-ddsp` está configurada para uso global dentro do projeto. Você pode importá-la no seu arquivo `.dsp` de duas formas:

#### Importação Padrão
Como o diretório `faust-ddsp/` está no path de busca (`-I`), você pode simplesmente usar:
```faust
import("stdfaust.lib");
import("diff.lib");

process = diff.lp(1000, 0.7);
```

#### Importação via Variável (Recomendado)
Para maior clareza conforme o padrão do projeto:
```faust
import("stdfaust.lib");
diff = library("diff.lib");

process = diff.lp(1000, 0.7);
```

---

## 4. Ponte FFI Zero-Copy

O princípio central do projeto é **nunca alocar memória na Audio Thread**. O buffer de áudio entregue pelo host é reutilizado in-place em toda a cadeia.

### Como funciona na prática

```
Host (DAW/Standalone)
      │
      │  &mut Buffer<f32>  (nih-plug gerencia)
      ▼
MojoProcessor::process_block(buffer: *mut f32, length: usize)
      │
      │  Converte ponteiro para usize (sem cópia!)
      │  let ptr = buffer as usize;
      │
      ▼
extern "C" { fn mojo_process_block(address: usize, size: usize) }
      │
      │  FFI call — passa o ENDEREÇO, não os dados
      │
      ▼
Mojo: UnsafePointer[Float32, MutAnyOrigin](unsafe_from_address=address)
      │  Modifica os bytes no lugar, na mesma região de memória
      ▼
Host recebe de volta o buffer modificado
```

### Código Rust da ponte (`src/bridge/mojo.rs`)

```rust
impl ExternalProcessor for MojoProcessor {
    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        if !self.is_ready {
            return;
        }
        
        unsafe {
            // ZERO-COPY: o ponteiro é convertido para usize.
            // Nenhum dado de áudio é copiado ou alocado.
            let ptr = buffer as usize;
            mojo_process_block(ptr, length);
        }
    }
}
```

A mesma estratégia zero-copy vale para o Faust (`src/bridge/faust.rs`):

```rust
fn process_block(&mut self, buffer: *mut f32, length: usize) {
    unsafe {
        // Ponteiro bruto passado diretamente ao runtime C do Faust
        faust_process(self.handle, buffer, length as _);
    }
}
```

---

## 5. Troubleshooting

### 🔴 Linking Error: `-lneural not found`

**Sintoma**: `error: linking with 'cc' failed` com menção a `libneural`.

**Causa**: O arquivo `neural/libneural.so` não existe porque o build do Mojo falhou ou foi pulado.

**Solução**:
1. Verifique se o arquivo existe: `ls -la neural/libneural.so`
2. Recompile manualmente: `./.venv/bin/mojo build --emit shared-lib neural/main.mojo -o neural/libneural.so`
3. Verifique se o Mojo está instalado: `which mojo || ls .venv/bin/mojo`
4. Confirme que `LD_LIBRARY_PATH` inclui `$(PWD)/neural` (configurado automaticamente pelo Makefile).

---

### 🔴 Mojo Syntax Errors

**Sintoma**: Erros de compilação em `neural/main.mojo` durante `make build-mojo`.

**Causas comuns e correções**:

| Erro | Causa | Correção |
|---|---|---|
| `parametricity error` | `UnsafePointer` como parâmetro de `@export` | Use `address: Int` e reconstrua o ponteiro internamente |
| `fn main() not allowed` | `fn main()` presente em biblioteca dinâmica | Remova `fn main()` — arquivos `.so` não têm entrypoint |
| `unresolved import` | Import não qualificado | Use `from std.memory import UnsafePointer` |
| `@extern_c not found` | Uso de anotação errada | Substitua por `@export` |

---

### 🟡 ALSA Warnings no Standalone

**Sintoma**: Mensagens como `ALSA lib pcm.c: playback stream` ou `underrun` no terminal.

**Status**: **Normal e esperado.** Estas mensagens são geradas pelo driver ALSA no modo Standalone e não afetam o processamento de áudio. O plugin continua funcionando corretamente.

---

### 🔴 Faust não encontrado

**Sintoma**: O `build.rs` entra em `panic!` com a mensagem `❌ ERRO: Transpilador Faust não encontrado`.

**Solução**:
```bash
# Ubuntu/Debian
sudo apt install faust

# Ou via Homebrew (macOS/Linux)
brew install faust

# Verifique a instalação
faust --version
```

---

### 🛠️ Diagnóstico Geral — Ambiente

**A primeira ferramenta de diagnóstico é sempre:**

```bash
bash scripts/check_env.sh
```

Este script valida todas as dependências do sistema (Faust, Mojo, Cargo, Rust) antes de qualquer build.

---

## 6. Instalação do Ambiente

### Pré-requisitos

| Ferramenta | Uso | Como instalar |
|---|---|---|
| Rust + Cargo | Build principal | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Mojo (MAX SDK) | Processamento neural | `modular install mojo` |
| Faust | DSP transpilation | `sudo apt install faust` |
| LLVM / Clang | Bindgen dependency | `sudo apt install clang` |
| ALSA dev libs | Backend de áudio | `sudo apt install libasound2-dev` |

### Configuração do Projeto

```bash
# 1. Clone o repositório
git clone <url-do-repositorio>
cd meu-novo-plugin

# 2. Ative o ambiente virtual Python (necessário se Mojo estiver no .venv)
source .venv/bin/activate

# 3. Valide o ambiente
bash scripts/check_env.sh

# 4. Execute o pre-build (Faust + Mojo)
make pre-build

# 5. Inicie em modo Standalone para validação
make run

# 6. Gere o bundle para DAW (VST3/CLAP)
make bundle
```

### Variáveis de Ambiente

O `Makefile` configura as seguintes variáveis automaticamente:

```makefile
MOJO_HOME  ?= $(HOME)/.modular/pkg/packages.modular.com_mojo
LD_LIBRARY_PATH := $(MOJO_HOME)/lib:$(PWD)/neural:$(LD_LIBRARY_PATH)
```

Se o Mojo estiver instalado no `.venv` local, o `Makefile` e o `build.rs` detectam isso automaticamente via busca em:
1. `$PATH`
2. `./.venv/bin/mojo`
3. `~/.modular/pkg/packages.modular.com_mojo/bin/mojo`
4. `~/.modular/bin/mojo`

---

## Referências Rápidas

| Arquivo | Responsabilidade |
|---|---|
| `src/bridge/mod.rs` | Trait `ExternalProcessor` — contrato entre Rust e os backends |
| `src/bridge/mojo.rs` | Adapter Rust → Mojo (conversão `*mut f32` → `usize`) |
| `src/bridge/faust.rs` | Adapter Rust → Faust (bindings gerados por `bindgen`) |
| `dsp/main.dsp` | Lógica DSP em Faust (fonte da verdade do processamento linear) |
| `dsp/wrapper.cpp` | Wrapper C++ que expõe o Faust ao Rust |
| `dsp/wrapper.h` | Contrato ABI C para o `bindgen` |
| `faust-ddsp/` | Biblioteca DDSP para Faust (local e integrada ao build) |
| `neural/main.mojo` | Processamento neural em Mojo (fonte da verdade do neural) |
| `neural/libneural.so` | Artefato compilado do Mojo (gerado pelo build, não versionado) |
| `build.rs` | Orquestra: rebuild do Faust, rebuild do Mojo, cc::Build, bindgen, linking |
| `Makefile` | Interface de comandos: `run`, `bundle`, `build`, `clean`, `pre-build` |
| `scripts/check_env.sh` | Diagnóstico de dependências do ambiente |
