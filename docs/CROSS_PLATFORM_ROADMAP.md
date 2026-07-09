# Plano de Robustez Cross-Platform — Windows & macOS

> **Desenvolvimento 100% em Linux.** O plano cobre adaptações para que o build
> e a execução standalone funcionem corretamente em Windows e macOS, com o
> código desenvolvido e testado primariamente em Linux.

---

## Índice de Bloqueadores e Lacunas

| # | Item | SO afetado | Gravidade | Esforço |
|---|---|---|---|---|
| B1 | `build.rs` — comandos Linux-only (`which`, `$HOME`) | Windows, macOS | 🔴 Bloqueador | 4h |
| B2 | Mojo neural — SDK inexistente no Windows | Windows | 🔴 Bloqueador | 1-3d |
| B3 | `build.rs` — output Mojo hardcoded `.so` | Windows, macOS | 🔴 Bloqueador | 2h |
| B4 | `wrapper.h` — `unsigned long` = 32-bit no MSVC x64 | Windows | 🔴 Bloqueador | 1h |
| B5 | Feature ASIO ausente no `Cargo.toml` | Windows | 🔴 Bloqueador | 2h |
| B6 | Gate F32-only rejeita entrada ASIO real (I32) | Windows | 🔴 Bloqueador | 2-4h |
| B7 | Bug I16 — ring buffer drena a meia velocidade | Windows, macOS, Linux | 🟠 Bug ativo | 30min |
| B8 | Buffer do slider descarta frames silenciosamente | Windows, macOS, Linux | 🟠 Bug ativo | 1h |
| L1 | Sem hot-plug / erro de stream não propagado para UI | Todos | 🟡 Robusteza | 4h |
| L2 | Sem `SupportedStreamConfigRange` | Todos | 🟡 Robusteza | 4h |
| L3 | Sample rate input/output divergente sem reconciliação | Todos | 🟡 Robusteza | 3h |
| L4 | Sem resampling entre input/output | Todos | 🟡 Robusteza | 4h |
| L5 | Slider "Latência Física" não controla buffer do driver | Todos | 🟡 UX | 2h |
| L6 | Dois `Device` ASIO independentes (risco duplex) | Windows | 🟡 Robusteza | 3h |
| L7 | Drivers ASIO 24-bit desaparecem da lista sem aviso | Windows | 🟢 Baixa | 1h |
| L8 | Sem CI/CD multi-plataforma | Todos | 🟢 Infra | 4h |
| L9 | Plugin reporta latência zero ao host DAW | Todos | 🟢 Baixa | 1h |
| M1 | `build.rs` — `-rpath` específico de ELF (Linux) | macOS | 🟡 Médio | 1h |
| M2 | Mojo → `.dylib` no macOS (extensão diferente) | macOS | 🟡 Médio | 1h |

---

## Fase 1 — Fundação: Compilar em todas as plataformas

### B1. Refatorar `build.rs` — `find_mojo_path()` cross-platform

**Ficheiro:** `build.rs`, função `find_mojo_path()` (linhas 5-27)

**Problema:**
- Linha 8: `Command::new("which").arg("mojo")` — `which` não existe no Windows
- Linha 17: `env::var("HOME")` — Windows usa `%USERPROFILE%`, macOS também usa `$HOME`

**Solução:**
```rust
fn which_like(binary: &str) -> Option<PathBuf> {
    // Windows: where.exe; Unix: which
    #[cfg(target_os = "windows")]
    let cmd = { let out = Command::new("where").arg(binary).output().ok()?; out };
    #[cfg(not(target_os = "windows"))]
    let cmd = { let out = Command::new("which").arg(binary).output().ok()?; out };

    if cmd.status.success() {
        let path = String::from_utf8_lossy(&cmd.stdout).trim().to_string();
        return Some(PathBuf::from(path));
    }
    None
}

fn find_mojo_path() -> Option<PathBuf> {
    if let Some(p) = which_like("mojo") { return Some(p); }

    let home = if cfg!(target_os = "windows") {
        env::var("USERPROFILE").ok().unwrap_or_default()
    } else {
        env::var("HOME").ok().unwrap_or_default()
    };

    #[cfg(target_os = "windows")]
    let common_paths = vec![
        format!("{}\\modular\\bin\\mojo.exe", home),
        format!("{}\\modular\\pkg\\packages.modular.com_mojo\\bin\\mojo.exe", home),
    ];
    #[cfg(not(target_os = "windows"))]
    let common_paths = vec![
        "./.venv/bin/mojo".to_string(),
        format!("{}/.modular/pkg/packages.modular.com_mojo/bin/mojo", home),
        format!("{}/.modular/bin/mojo", home),
    ];

    for path in common_paths {
        let pb = PathBuf::from(&path);
        if pb.exists() { return Some(pb); }
    }
    None
}
```

**Esforço:** ~4h

---

### B2. Portar tanh neural do Mojo para Rust (fallback Windows)

**Ficheiro afetado:** `src/bridge/mojo.rs`, novo ficheiro `src/bridge/neural_rust.rs`

**Problema:** O Mojo SDK não tem versão Windows. O ficheiro `neural/main.mojo`
contém ~27 linhas de lógica — uma aproximação polinomial de tanh. É trivial
portar para Rust puro.

**Código Mojo atual** (`neural/main.mojo`):
```mojo
var data = UnsafePointer[Float32, MutAnyOrigin](unsafe_from_address=address)
for var i in range(size):
    var x = data[i] * drive
    if x > 4.0: x = 4.0
    elif x < -4.0: x = -4.0
    var x2 = x * x
    var saturated = x * (27.0 + x2) / (27.0 + 9.0 * x2)
    data[i] = saturated * output_gain
```

**Solução — Ficheiro novo `src/bridge/neural_rust.rs`:**
```rust
/// Processador neural em Rust puro — fallback cross-platform quando
/// o Mojo não está disponível (ex: Windows, ou build sem Mojo SDK).
pub struct RustNeuralProcessor {
    drive: f32,
    output_gain: f32,
    is_ready: bool,
}

impl ExternalProcessor for RustNeuralProcessor {
    fn init(&mut self, _sample_rate: f32) { self.is_ready = true; }

    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        if !self.is_ready { return; }
        unsafe {
            let data = std::slice::from_raw_parts_mut(buffer, length);
            for sample in data.iter_mut() {
                let mut x = *sample * self.drive;
                x = x.clamp(-4.0, 4.0);
                let x2 = x * x;
                let saturated = x * (27.0 + x2) / (27.0 + 9.0 * x2);
                *sample = saturated * self.output_gain;
            }
        }
    }
    // ... get_param, set_param, etc.
}
```

**Estratégia de seleção (`src/bridge/mojo.rs`):**
```rust
#[cfg(target_os = "linux")]
pub type NeuralProcessor = MojoProcessor;  // Mojo via FFI

#[cfg(not(target_os = "linux"))]
pub type NeuralProcessor = RustNeuralProcessor;  // Rust puro
```

Isto mantém o Mojo no Linux (desempenho nativo) e usa Rust puro como fallback
universal em Windows e macOS. A função `mojo_init`/`mojo_process_block` no
Linux continua a ser ligada via `extern "C"` com o `.so` compilado pelo
`build.rs`. No Windows/macOS, a type alias aponta para a implementação Rust
que não depende de FFI externa.

**Esforço:** 1-3 dias (inclui testes de equivalência numérica)

---

### B3. `build.rs` — output Mojo condicional por SO

**Ficheiro:** `build.rs`, linhas 96-98 e 142

**Problema:**
- Linha 98: `-o neural/libneural.so` — hardcoded `.so`
  - Windows: `neural.dll`
  - macOS: `libneural.dylib`
- Linha 142: `cargo:rustc-link-lib=dylib=neural` — funciona mas a keyword
  `dylib` não cobre `.dll` corretamente no Windows

**Solução:**
```rust
let ext = if cfg!(target_os = "windows") {
    "dll"
} else if cfg!(target_os = "macos") {
    "dylib"
} else {
    "so"
};
let lib_name = if cfg!(target_os = "windows") {
    "neural.dll"
} else if cfg!(target_os = "macos") {
    "libneural.dylib"
} else {
    "libneural.so"
};
// ...
.args(&["-o", &format!("neural/{}", lib_name)])

// Linking
println!("cargo:rustc-link-lib=neural");  // sem keyword dylib — funciona em todos
```

Adicionalmente, gatear o build Mojo para só correr em Linux (já que o fallback
Rust cobre Windows/macOS):
```rust
#[cfg(target_os = "linux")]
fn build_mojo() { /* ... */ }

#[cfg(not(target_os = "linux"))]
fn build_mojo() { /* no-op: usa Rust fallback */ }
```

**Esforço:** ~2h

---

### B4. Corrigir `wrapper.h` — `f_size_t` para 64-bit em MSVC

**Ficheiro:** `dsp/wrapper.h`, linha 5

**Problema:** `typedef unsigned long f_size_t;`
- Linux LP64: `unsigned long` = 8 bytes ✅
- Windows LLP64 (MSVC x64): `unsigned long` = 4 bytes ❌
- macOS LP64: `unsigned long` = 8 bytes ✅

O Rust espera `usize` (sempre 64-bit em targets x86_64), mas o MSVC passa
32-bit → corrupção de memória subtil.

**Solução:**
```c
#include <stdint.h>
typedef uint64_t f_size_t;
```

Ou, mais idiomático:
```c
#include <stddef.h>
typedef size_t f_size_t;
```

`size_t` é sempre 64-bit em targets 64-bit, em qualquer compilador.

**Verificação:** Confirmar que `bindgen` gera `usize` no lado Rust (já o faz
com `size_t`). Testar com `cargo test` em Linux após a mudança.

**Esforço:** ~1h

---

### B5. Ativar feature ASIO no CPAL

**Ficheiro:** `Cargo.toml`, linha 22

**Estado atual:**
```toml
cpal = "0.15.2"
```

**Necessário:**
```toml
[target.'cfg(target_os = "windows")'.dependencies]
cpal = { version = "0.15.2", features = ["asio"] }

[target.'cfg(not(target_os = "windows"))'.dependencies]
cpal = "0.15.2"
```

Isto ativa ASIO só no Windows, evitando puxar `asio-sys` + `num-traits` +
dependência no ASIO SDK em Linux/macOS.

**Pré-requisitos de build no Windows:**
- ASIO SDK da Steinberg (gratuito, download em steinberg.net)
- Variável de ambiente `ASIO_DIR` apontando para o SDK
- LLVM/libclang instalado (para `asio-sys` bindgen)

**Esforço:** ~2h (configuração) + documentação no README

---

### B6. Aceitar I32/I16 na entrada + conversão para F32

**Ficheiro:** `src/bin/standalone.rs`, linhas 774-775

**Problema:** O match de `config.sample_format()` só tem braço `F32`; qualquer
outro formato devolve `StreamConfigNotSupported`. No backend ASIO, CPAL reporta
`ASIOSTInt32LSB/MSB → I32`, que é o formato nativo da esmagadora maioria das
interfaces (Focusrite, RME, Behringer).

**Solução — Adicionar braços I32 e I16 no input:**
```rust
let stream_res = match config.sample_format() {
    cpal::SampleFormat::F32 => { /* ... código existente ... */ },
    cpal::SampleFormat::I32 => device.build_input_stream(
        &strict_config,
        move |data: &[i32], _: &_| {
            let num_frames = data.len() / (channels as usize);
            let max_len = num_frames.min(buffer_size as usize);
            for (i, frame) in data.chunks(channels as usize)
                .enumerate().take(max_len)
            {
                let l_sample = frame.get(l_idx).copied().unwrap_or(0);
                let r_sample = frame.get(r_idx).copied().unwrap_or(0);
                // Converter I32 → F32 (normalizado para [-1.0, 1.0])
                let l_f = (l_sample as f64 / i32::MAX as f64) as f32;
                let r_f = (r_sample as f64 / i32::MAX as f64) as f32;
                buf_l[i] = l_f;
                buf_r[i] = r_f;
            }
            // ... pipeline DSP igual ao ramo F32 ...
        },
        |err| eprintln!("Input error: {:?}", err),
        None,
    ),
    cpal::SampleFormat::I16 => device.build_input_stream(
        &strict_config,
        move |data: &[i16], _: &_| {
            // ... análogo ao I32, com normalização i16::MAX ...
        },
        |err| eprintln!("Input error: {:?}", err),
        None,
    ),
    _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
};
```

**Nota:** A conversão I32/I16 → F32 é feita uma vez na entrada, antes do
pipeline DSP. O pipeline inteiro (Faust, Neural, Cabinet) continua a operar
em F32 — apenas a fronteira com o driver é alargada.

**Esforço:** ~2-4h (inclui refatoração para evitar duplicação de ~100 linhas
de pipeline DSP nos 3 braços; extrair o pipeline para uma closure/função
genérica)

---

## Fase 2 — Bugs Ativos (afetam todas as plataformas)

### B7. Corrigir bug I16 — ring buffer a meia velocidade

**Ficheiro:** `src/bin/standalone.rs`, linhas 1241-1242

**Problema:** O ramo de output I16 faz **um** `pop()` por frame, mas o ramo F32
faz **dois** (L + R). O produtor faz `push` de L *e* R por frame (linha 1154-1157).
Resultado: o stream I16 drena o ring buffer a metade da velocidade → áudio uma
oitava abaixo + buffer cresce indefinidamente.

**Código atual (bug):**
```rust
cpal::SampleFormat::I16 => device.build_output_stream(
    &strict_config,
    move |data: &mut [i16], _: &_| {
        for frame in data.chunks_mut(channels as usize) {
            let l_sample = match pt_consumer.as_mut() {
                Some(pc) => pc.pop().unwrap_or(0.0),
                None => 0.0,
            };
            let pcm = (l_sample * i16::MAX as f32) as i16;
            if let Some(l) = frame.get_mut(l_idx) { *l = pcm; }
            if let Some(r) = frame.get_mut(r_idx) { *r = pcm; }
        }
    },
```

**Correção:**
```rust
cpal::SampleFormat::I16 => device.build_output_stream(
    &strict_config,
    move |data: &mut [i16], _: &_| {
        for frame in data.chunks_mut(channels as usize) {
            let (l_sample, r_sample) = match pt_consumer.as_mut() {
                Some(pc) => (pc.pop().unwrap_or(0.0), pc.pop().unwrap_or(0.0)),
                None => (0.0, 0.0),
            };
            if let Some(l) = frame.get_mut(l_idx) {
                *l = (l_sample * i16::MAX as f32) as i16;
            }
            if let Some(r) = frame.get_mut(r_idx) {
                *r = (r_sample * i16::MAX as f32) as i16;
            }
        }
    },
```

**Esforço:** ~30min

---

### B8. Corrigir truncamento de buffer — frames descartados

**Ficheiro:** `src/bin/standalone.rs`, linhas 891-893

**Problema:**
```rust
let num_frames = data.len() / (channels as usize);
let max_len = num_frames.min(buffer_size as usize);
```

O `buffer_size` vem do slider (mínimo 32, linha 1383). Se o driver WASAPI
entrega ~480 frames, `max_len = min(480, 32) = 32` → 93% das amostras são
**silenciosamente descartadas**. O `take(max_len)` trunca o iterador.

**Solução:** O buffer interno deve ser dimensionado para caber o bloco real
do driver, nunca truncado:
```rust
let num_frames = data.len() / (channels as usize);
// Garantir que o buffer interno é grande o suficiente
ensure_buffer_capacity(num_frames);
let max_len = num_frames;  // processar TODOS os frames
```

E o ring buffer deve ser redimensionado com base no **tamanho real observado**
do callback, não no slider:
```rust
let actual_buffer = num_frames.max(64);  // mínimo razoável
let (p, c) = RingBuffer::new((actual_buffer * 8).max(2048) as usize);
```

O slider pode continuar a existir como "buffer de segurança interno" (para
casos de jitter), mas o processamento nunca deve truncar frames.

**Esforço:** ~1h

---

## Fase 3 — Robustez Cross-Platform

### L1. Hot-plug e propagação de erros para UI

**Ficheiro:** `src/bin/standalone.rs`

**Problemas:**
1. `refresh_devices()` só é chamado no arranque (linha 1414) e na mudança de
   host (linha 1623). Se o dispositivo for desligado, o áudio morre em silêncio.
2. `error_callback` dos streams só faz `eprintln!` (linhas 1160, 1234) — nunca
   chega à UI.
3. Não há botão de refresh manual na UI.

**Solução:**
1. Adicionar botão "Refresh Devices" na UI (linha ~1615, junto ao ComboBox
   de Driver)
2. Adicionar `AudioCommand::StreamError(String)` ao enum e propagar via
   `tx_event` no `error_callback`
3. Exibir banner de erro na UI quando `AudioEvent::StreamError` é recebido
4. Opcional: timer de 2s para re-scan automático de dispositivos

**Esforço:** ~4h

---

### L2. Usar `SupportedStreamConfigRange` para negociação

**Ficheiro:** `src/bin/standalone.rs`

**Estado atual:** Usa exclusivamente `default_input_config()` /
`default_output_config()`. Não conhece os limites reais do driver.

**Solução:**
```rust
// Enumerar TODAS as configs suportadas
let supported_configs: Vec<SupportedStreamConfig> = device
    .supported_input_configs()?
    .flat_map(|range| range.try_into_supported_config())
    .collect();

// Filtrar por F32 (ou I32 com conversão)
// Escolher a melhor: sample rate mais alta, buffer size razoável
let best_config = supported_configs.iter()
    .filter(|c| c.sample_format() == SampleFormat::F32
                || c.sample_format() == SampleFormat::I32)
    .max_by_key(|c| c.max_sample_rate());
```

**Esforço:** ~4h

---

### L3. Negociação de sample rate comum input/output

**Ficheiro:** `src/bin/standalone.rs`, linhas 1494-1506

**Estado atual:** Apenas avisa se input e output têm sample rates diferentes.
Prossegue com ambos — o que causa drift ou glitches.

**Solução:**
1. Se input e output têm sample rates diferentes, procurar uma sample rate
   comum nos `supported_configs()` de ambos
2. Se não houver comum, escolher a do output como master e usar resampling
   (ver L4) no input
3. Inicializar Faust e Mojo com a sample rate efetiva (a do output)

**Esforço:** ~3h

---

### L4. Resampling entre input/output (via `rubato`)

**Ficheiro:** `src/bin/standalone.rs`

**Estado atual:** `rubato` já é dependência (`Cargo.toml:23`), mas só é usado
para reamostragem offline de IRs de cabinet (`src/core/cabinet/runtime.rs`).
Não há resampling em tempo real.

**Solução:** Inserir `rubato::FftFixedInOut` (ou `SincFixedInOut`) entre o
callback de input e o pipeline DSP quando as sample rates divergem.

```rust
use rubato::{Resampler, SincFixedInOut, InterpolationType, WindowFunction};

if input_sr != output_sr {
    let resampler = SincFixedInOut::new(
        output_sr / input_sr,  // ratio
        1.0,                    // max_relative_error
        InterpolationType::Linear,
        WindowFunction::BlackmanHarris2,
        num_frames,             // chunk_size
        2,                      // n_channels (L+R)
        1,                      // sub_chunks
    )?;
    // Aplicar no callback de input antes do pipeline DSP
}
```

**Esforço:** ~4h

---

### L5. Slider "Latência Física" — refletir buffer real ou remover

**Ficheiro:** `src/bin/standalone.rs`

**Estado atual:** O slider `buffer_power` (linha 1383, range 5-15 → 32-32768)
não controla o buffer do driver — `BufferSize::Default` é sempre usado
(linhas 766, 1205). O slider só dimensiona ring buffers internos.

**Opção A (recomendada):** Remover o slider como controlo de latência e
substituí-lo por:
- Label informativo: "Driver buffer: 256 samples @ 48kHz = 5.3ms"
- Internamente, o buffer_size é derivado de `num_frames` real do callback

**Opção B:** Usar `BufferSize::Fixed(n)` e passar o valor do slider ao CPAL,
mas isto só funciona em backends que suportam (WASAPI sim, ASIO normalmente
não — o driver ASIO impõe o seu próprio buffer size).

**Esforço:** ~2h

---

### L6. ASIO duplex — unificar Device handle

**Ficheiro:** `src/bin/standalone.rs`, linhas 761-764 e 1200-1203

**Problema:** Input e output são construídos como dois `Device` independentes,
obtidos de `input_devices()` e `output_devices()`. No backend ASIO, cada
`Device` carrega o seu próprio estado de driver e o driver ASIO real é
tipicamente instância única duplex. Carregar duas vezes pode falhar.

**Solução:** Quando host é ASIO, obter um único `Device` que sirva para input
e output (usar o mesmo nome de dispositivo). Construir ambos os streams a
partir do mesmo `Device`.

**Esforço:** ~3h (requer teste em máquina Windows com interface ASIO real)

---

### L7. Drivers ASIO 24-bit — não os esconder da lista

**Ficheiro:** `src/bin/standalone.rs`, linha 677; cpal `host/asio/device.rs`

**Problema:** `ASIOSTInt24LSB` não consta de `convert_data_type` no CPAL →
`default_input_config()` devolve `Err` → dispositivo é saltado sem aviso.

**Solução:** Reportar dispositivos mesmo sem config válida, com indicação de
"formato não suportado". Alternativamente, contribuir com suporte a
`ASIOSTInt24LSB` no CPAL upstream.

**Esforço:** ~1h

---

### L8. CI/CD Multi-Plataforma

**Ficheiro novo:** `.github/workflows/ci.yml`

```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4

      - name: Install Faust
        if: matrix.os != 'windows-latest'
        run: sudo apt-get install -y faust || brew install faust

      - name: Install Mojo (Linux only)
        if: matrix.os == 'ubuntu-latest'
        run: |
          curl https://get.modular.com | sh -s -- --quick
          echo "$HOME/.modular/bin" >> $GITHUB_PATH

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build
        run: cargo build --release

      - name: Test
        run: cargo test --release
```

**Esforço:** ~4h (inclui iterações de debugging do CI)

---

### L9. Reportar latência real ao host DAW

**Ficheiro:** `src/lib.rs`, linha 998

**Estado atual:**
```rust
_context.set_latency_samples(0);
```

**Correção:** Reportar a latência real do pipeline DSP (FFT, convolução, etc.)
quando conhecida, ou pelo menos o buffer size do host:
```rust
let latency = buffer_config.max_buffer_size as u32;
_context.set_latency_samples(latency);
```

**Esforço:** ~1h

---

## Fase 4 — macOS Específico

### M1. `build.rs` — `-rpath` para macOS

**Ficheiro:** `build.rs`, linhas 140-141

**Estado atual:**
```rust
if env::var("CARGO_CFG_TARGET_OS").ok().as_deref() == Some("linux") {
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", neural_dir);
}
```

**Problema:** macOS também precisa de rpath para `.dylib`, mas a flag já é
suportada pelo linker da Apple (`ld64`). Só é necessário alargar o gate:

```rust
if cfg!(any(target_os = "linux", target_os = "macos")) {
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", neural_dir);
}
```

**Esforço:** ~1h

---

### M2. `neural/.gitignore` e extensões `.dylib`

**Ficheiro:** `neural/.gitignore` (se existir), `build.rs` linhas 96-98

**Problema:** macOS compila Mojo para `libneural.dylib`. O `.gitignore` deve
incluir ambas as extensões.

Além disso, o Mojo SDK **tem suporte oficial para macOS** (Apple Silicon e
Intel), portanto o build Mojo no macOS funciona sem fallback Rust — apenas a
extensão do ficheiro de saída difere.

**Esforço:** ~1h

---

## Resumo de Esforço Total

| Fase | Descrição | Esforço | Bloqueia |
|---|---|---|---|
| **Fase 1** | Compilar em Windows + macOS | 6-8 dias | Tudo |
| **Fase 2** | Bugs ativos (todas plataformas) | 1.5h | Áudio correto |
| **Fase 3** | Robustez cross-platform | ~26h (3-4 dias) | Experiência sólida |
| **Fase 4** | macOS específico | 2h | macOS |

| Plataforma | Estado após Fase 1 | Estado após Fase 3 |
|---|---|---|
| **Linux** | ✅ Já funciona | ✅ Robusto |
| **macOS** | ✅ Compila + CoreAudio OK | ✅ Robusto |
| **Windows** | ✅ Compila + WASAPI OK; ASIO parcial | ✅ Robusto (ASIO full) |

---

## Notas de Desenvolvimento

1. **Todo o desenvolvimento é feito em Linux.** As adaptações cross-platform
   são feitas com `#[cfg(target_os = "...")]` e testadas via CI no GitHub
   Actions.

2. **O Mojo continua a ser o backend neural primário no Linux.** O fallback
   Rust é ativado apenas em `#[cfg(not(target_os = "linux"))]`. Ambas as
   implementações partilham a mesma trait `ExternalProcessor`, portanto o
   resto do código é agnóstico.

3. **Testes:** Adicionar um teste de equivalência numérica entre
   `MojoProcessor` e `RustNeuralProcessor` para garantir que produzem output
   bit-identical (dentro de tolerância `f32`).

4. **CI:** A matriz multi-OS no GitHub Actions valida cada commit em Linux,
   macOS e Windows. O build Mojo só corre no Linux; macOS e Windows usam o
   fallback Rust (e o macOS pode opcionalmente usar Mojo também).

5. **Ordem de implementação sugerida:**
   - B4 (wrapper.h) — 1h, sem dependências
   - B1 (build.rs cross-platform) — 4h, base para tudo
   - B2 + B3 (Mojo fallback + extensões) — 1-3 dias, o maior bloco
   - B7 + B8 (bugs ativos) — 1.5h, afetam Linux também
   - B5 + B6 (ASIO) — 4-6h, Windows-specific
   - Fase 3 (robustez) — 3-4 dias
   - L8 (CI/CD) — 4h, valida tudo
