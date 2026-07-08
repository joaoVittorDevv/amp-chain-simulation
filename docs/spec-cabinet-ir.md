# Especificação: Emulação de Caixa com Impulse Response

**Status:** Especificação final — resultado de debate entre 3 agentes (2 propostas + reconciliação)
**Data:** 2026-07-06
**Escopo:** Adicionar biblioteca gerenciada e persistida de impulse responses de caixa ao estágio 4 (cabinet convolution) do plugin, com armazenamento byte-exato, troca em runtime, bypass e UI — em ambos os targets (plugin e standalone).

---

## 0. Princípio Norteador

Separar três responsabilidades que o design atual (path absoluto hardcoded) confunde:

1. **A biblioteca** (quais IRs existem) → compartilhada, global, persistida em SQLite.
2. **A seleção** (qual IR está ativo) → por projeto DAW no plugin; global no standalone.
3. **O objeto de áudio** (um convolver construído) → vive apenas na thread de áudio, recebe apenas objetos completamente prontos.

---

## 1. Armazenamento e Modelo de Dados

### Backend: SQLite via `rusqlite` (feature `bundled`)

SQLite com `bundled` compila seu próprio C — sem dependência de sistema, crucial para distribuição de plugin. WAL mode com `busy_timeout` de 200ms para acesso concorrente entre plugin + standalone.

### Schema

```sql
CREATE TABLE cabinet_irs (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    name         TEXT NOT NULL,              -- label editável, padrão = stem do filename
    filename     TEXT NOT NULL,              -- nome original do arquivo (proveniência)
    content_hash TEXT NOT NULL UNIQUE,       -- BLAKE3 hex — identidade estável cross-sessão
    sample_rate  INTEGER NOT NULL,           -- sample rate nativo do WAV
    channels     INTEGER NOT NULL,           -- 1 (mono) ou 2 (stereo)
    num_frames   INTEGER NOT NULL,           -- comprimento do IR em frames
    bit_depth    INTEGER NOT NULL,           -- 16 ou 32 (para display)
    byte_size    INTEGER NOT NULL,           -- tamanho do BLOB (sanity check)
    ir_data      BLOB NOT NULL,              -- bytes EXATOS do arquivo WAV original
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    last_used_at TEXT
);

CREATE TABLE cabinet_state (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

`cabinet_state` armazena:
- `selected_hash` → content hash do IR ativo (string vazia = nenhum)
- `schema_version` → "1" (para migrações futuras)

### Localização

`dirs::data_dir() / "distortion" / "cabinet_irs.db"`

| Plataforma | Caminho |
|---|---|
| Linux | `~/.local/share/distortion/cabinet_irs.db` |
| macOS | `~/Library/Application Support/distortion/cabinet_irs.db` |
| Windows | `%LOCALAPPDATA%/distortion/cabinet_irs.db` |

### Por que content hash como identidade (não UUID row id)

1. **Dedup automático.** `content_hash UNIQUE` impede IRs duplicados — reimportar o mesmo arquivo é um no-op que apenas re-seleciona a entrada existente.
2. **Persistência auto-curável.** Persistido por hash (`cab_active_hash`), se o usuário move o projeto para outra máquina que já tem o mesmo IR (mesmo com filename diferente), o hash resolve corretamente. Row IDs não são portáveis entre DBs.
3. **Integridade é efeito colateral.** O hash serve como verificação de integridade: ao carregar, re-hash do BLOB e compara com `content_hash`. Mismatch → recusa carga.
4. **Sem dependência `uuid`.** Um crate a menos.

### Por que BLAKE3 (não SHA256)

- ~10x mais rápido em CPUs modernas para hashing de multi-megabytes.
- Output de 256 bits é mais que suficiente para content addressing de IRs (colisões são irrelevantes para esta aplicação).
- Crate menor que `sha2` + `uuid` combinados.

---

## 2. Integridade do Arquivo IR

**Garantia: os bytes exatos que o usuário selecionou são preservados sem modificação.**

### Caminho de importação

```
1. rfd file dialog → path
2. std::fs::read(path) → Vec<u8> (bytes brutos)
3. blake3(Vec<u8>) → content_hash
4. hound::WavReader (read-only) → extrair metadados (sample_rate, channels, num_frames, bit_depth)
5. INSERT INTO cabinet_irs (..., ir_data = Vec<u8>, content_hash = hash)
6. hound::WavReader → decodificar samples → f32 normalizado → construir CabinetRuntime
```

**Nunca re-encodamos.** `hound` é usado apenas em modo leitura para extrair metadados e decodificar samples para o convolver — nunca toca os bytes armazenados.

### Verificação na carga

```
1. SELECT ir_data, content_hash FROM cabinet_irs WHERE content_hash = ?
2. blake3(ir_data) == content_hash ?
   SIM → construir CabinetRuntime
   NÃO → recusar, logar erro "IR corrompido"
```

### Round-trip: exportação

Ação "Export IR…" escreve o BLOB direto em disco com `std::fs::write`, produzindo arquivo byte-idêntico ao original importado.

### Guarda de tamanho

Rejeitar imports acima de 10 MB (IRs de caixa típicos são < 1 MB). Exigir parse bem-sucedido com `hound` antes do INSERT.

---

## 3. Integração no Pipeline de Áudio

### Posição no pipeline (mantida)

```
Estágio 3: Mojo neural drive (tanh)
Estágio 4: Cabinet IR convolution  ← AQUI
Estágio 5: Master gain
```

### Protocolo de handoff: `ArcSwap`

Usamos `arc-swap` (já é dependência no `Cargo.toml`: `arc-swap = "1.7.1"`) para troca atômica do convolver entre threads:

```rust
struct CabinetEngine {
    active: arc_swap::ArcSwapOption<CabinetRuntime>,
}

struct CabinetRuntime {
    convolver_l: FFTConvolver<f32>,
    convolver_r: FFTConvolver<f32>,
    ir_hash: String,
}
```

- **UI thread:** constrói `CabinetRuntime` (aloca, inicializa FFTs, resampleia se necessário) → `engine.active.store(Arc::new(Some(runtime)))`. O `Arc` antigo é dropado naturalmente quando refcount chega a zero — na thread de UI, nunca na de áudio.
- **Audio thread:** `let handle = engine.active.load();` — empréstimo atômico de `Arc`. Se `Some`, usa o runtime. Zero blocking, zero alocação.

**Por que `ArcSwap` e não filas `rtrb`:** `rtrb` é para streaming de dados, não handoff de ownership. Filas introduzem capacity concerns e backpressure desnecessários. `ArcSwap` é a abstração correta para "um writer ocasional troca, muitos readers leem atomicamente sem bloquear."

### Estratégia de rampa na troca de IR

**5ms mute-ramp obrigatório** na troca de IR. Trocar `FFTConvolver` no meio de um buffer causa descontinuidade (buffers internos de overlap-add zerados). Mute de ~240 samples a 48kHz é imperceptível e elimina o glitch completamente.

**10ms crossfade wet↔dry no bypass.** Quando `cabinet_bypass` transiciona, crossfade sobre 10ms para evitar clique.

### Resampling com `rubato`

- IRs são armazenados no DB em sample rate **nativo** (preserva dado original).
- Resampling acontece em `initialize()` ao construir `CabinetRuntime` — usa `rubato` (já é dependência).
- Tratamento mono/stereo: IR mono → mesmo convolver L+R. IR stereo → canal 0 → left, canal 1 → right.

### Troca de sample rate

Em `initialize()`, reconstruir o `CabinetRuntime` para o novo engine rate.

---

## 4. Modelo de Parâmetros

### NIH-plug params (automatáveis)

```rust
// Em BaseIOParams:

#[id = "cab_bypass"]
pub cabinet_bypass: BoolParam,
// default: false
// crossfade 10ms wet↔dry na transição

#[id = "cab_level"]
pub cabinet_level: FloatParam,
// range: -24.0..+12.0 dB, default: 0.0 dB
// SmoothingStyle::Logarithmic(50ms)
// aplicado APÓS a convolução, antes do mix

#[id = "cab_mix"]
pub cabinet_mix: FloatParam,
// range: 0.0..1.0, default: 1.0 (100% wet)
// SmoothingStyle::Linear(50ms)
// blend wet/dry ao redor do convolver
// dry path = sinal que entrou no estágio (pré-convolver)
// 0.0 = 100% dry, 1.0 = 100% wet
```

**Por que `cabinet_mix` é obrigatório (não opcional):** Dry/wet mix para cabinet IR é controle de primeira classe em amp modeling. Adicionar depois quebra presets existentes. Custa ~10 linhas no `process()`.

### Seleção de IR (NÃO automatizável)

A seleção de IR **não** é um parâmetro automatizável. IRs não são estáveis entre máquinas/sessões; trocar IR aloca e reconstrói FFTs (não é seguro em tempo real).

```rust
#[persist = "cab_active_hash"]
pub cab_active_hash: String,  // vazio = nenhum IR selecionado
```

Este campo:
- Não tem `#[id]` — não é exposto ao host como parâmetro.
- É persistido pelo NIH-plug por projeto DAW (plugin target).
- É lido em `initialize()` para resolver o hash contra o DB e construir o `CabinetRuntime`.

---

## 5. UI Design

### Signal Chain

Adicionar terceiro nó "CABINET" ao visualizador de chain:

```
PARAM EQ  →  NEURAL AMP  →  CABINET  →  OUTPUT
```

`ActivePanel` ganha variante `Cabinet`. Layout de 4 nós: `n * total_span / 4` para n = 1, 2, 3.

### Painel Cabinet (`src/core/ui/cabinet_panel.rs`)

Novo módulo seguindo o padrão de injeção de closures da UI existente. Layout:

```
┌────────────────────────────────────────────────┐
│ CABINET                     [bypass toggle]    │
│                                                │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │  LEVEL   │  │   MIX    │  │   (vazio) │     │
│  │  -2.3dB  │  │   85%    │  │           │     │
│  └──────────┘  └──────────┘  └──────────┘     │
│                                                │
│  IR Ativo: Marshall 4x12 Greenback             │
│  [Sample rate: 48000 Hz | Canais: 1 | 2048 fr] │
│                                                │
│  ┌──────────────────────────────────────────┐  │
│  │ IR Library                          [3]  │  │
│  │  ● Marshall 4x12 Greenback  48k mono    │  │
│  │    Mesa Recto 4x12          44k stereo   │  │
│  │    Fender Twin 2x12         48k mono    │  │
│  └──────────────────────────────────────────┘  │
│                                                │
│  [Load IR...]  [Delete]  [Rename]  [Export]   │
└────────────────────────────────────────────────┘
```

### Comportamento do IR Browser

- Lista scrollable de IRs armazenados (nome, sample rate, canais).
- Linha ativa destacada.
- Clique para selecionar → dispara `select_ir(hash)` → reconstrói `CabinetRuntime` → `ArcSwap::store`.
- Estado vazio: "Nenhum IR de caixa carregado" com botão "Load IR…" proeminente.

### Fluxo de importação

1. Usuário clica "Load IR…".
2. `rfd::FileDialog::new().add_filter("WAV", &["wav"]).pick_file()`.
3. Arquivo lido, hash calculado, parseado com `hound`.
4. Se hash já existe no DB → apenas re-seleciona (sem duplicar).
5. Se novo → INSERT no DB, constrói `CabinetRuntime`, `ArcSwap::store`.
6. UI atualiza lista e seleção.

### Tratamento de erros na UI

- WAV inválido: toast/mensagem de erro não-bloqueante.
- Falha de escrita no DB: erro + mantém IR atual ativo.
- Falha de decode/resample: não troca o IR de runtime.
- IR selecionado desapareceu do DB: fallback para pass-through + aviso.

### Closures injetadas (padrão existente)

O painel recebe closures do caller (plugin ou standalone):

```rust
get_ir_list() -> Vec<IrMeta>
get_active_hash() -> String
select_ir(hash: String)
import_ir(path: PathBuf)
delete_ir(hash: String)
rename_ir(hash: String, new_name: String)
export_ir(hash: String, dest: PathBuf)
get_bypass() -> bool
set_bypass(v: bool)
get_level() -> f32
set_level(v: f32)
get_mix() -> f32
set_mix(v: f32)
```

---

## 6. Paridade Standalone

### Módulo compartilhado: `src/core/cabinet/`

```
src/core/cabinet/
├── mod.rs              # re-exports
├── library.rs          # CabinetLibrary — SQLite, CRUD de IRs, cache de metadados
├── engine.rs           # CabinetEngine — ArcSwap<CabinetRuntime>, swap/rampa
├── runtime.rs          # CabinetRuntime — FFTConvolver L+R, hash, metadados
└── types.rs            # IrMeta, CabinetBuildRequest, resultados
```

### Plugin target (`src/lib.rs`)

- `BaseIO` ganha campo `cabinet_engine: CabinetEngine`.
- `initialize()`: abre/cria DB em `dirs::data_dir()`, resolve `cab_active_hash` contra DB, constrói `CabinetRuntime`, faz `ArcSwap::store`.
- `process()`: `let runtime = self.cabinet_engine.active.load()` → se `Some`, faz convolução com bypass/mix/level.
- `create_egui_editor()`: injeta closures que usam `CabinetLibrary` para CRUD + `ParamSetter` para bypass/level/mix.
- Persistência da seleção: `#[persist = "cab_active_hash"]` no `BaseIOParams` (NIH-plug gerencia por projeto).

### Standalone target (`src/bin/standalone.rs`)

- `StandaloneState` ganha: `cabinet_bypass: bool`, `cabinet_level: f32`, `cabinet_mix: f32`, `cab_active_hash: String`.
- `StandaloneApp` ganha `cabinet_engine: CabinetEngine` + `cabinet_library: Arc<Mutex<CabinetLibrary>>`.
- Inicialização: abre DB, resolve `cab_active_hash`, constrói runtime.
- Audio callback: snapshot dos campos cabinet do `StandaloneState`, aplica no `CabinetEngine`.
- UI: mesmo painel `cabinet_panel.rs`, com closures que mutam `StandaloneState` via `Arc<Mutex<>>`.
- Persistência: serializa `cabinet_active_hash` (e opcionalmente bypass/level/mix) em `dirs::config_dir() / "distortion" / "standalone.json"` via `serde_json`.

**Lógica duplicada apenas na binding de estado.** Toda lógica de DB, runtime, convolução e UI é compartilhada em `src/core/cabinet/`.

---

## 7. Thread Safety

| Operação | Thread | Mecanismo |
|---|---|---|
| Leitura/escrita SQLite | UI apenas | Audio thread **nunca** toca o DB |
| File dialog (`rfd`) | UI | Bloqueante na UI thread (raro, iniciado pelo usuário) |
| Import (read file, hash, parse WAV, DB insert) | UI ou worker thread | Spawned thread para não travar UI; ao terminar, `ArcSwap::store` |
| Build CabinetRuntime (decode WAV, resample rubato, init FFTConvolver) | UI/worker | Fora da thread de áudio; alocações e FFT init são seguros aqui |
| Convolver handoff | UI/worker → áudio | `ArcSwap::store` (writer), `ArcSwap::load` (reader) — atômico, sem bloqueio |
| Drop do convolver antigo | UI/worker | `Arc` refcount chega a zero na thread que fez `store` — nunca na thread de áudio |
| Params (bypass, level, mix) | Ambas | NIH-plug atomic-backed params (já thread-safe) |
| Cache de metadados para UI | UI | `Arc<Mutex<CabinetLibrary>>` na thread de UI; áudio nunca lê |

**Regra de ouro:** Audio thread faz apenas DSP. Nunca abre arquivos, nunca acessa SQLite, nunca aloca, nunca faz lock. Apenas `ArcSwap::load()` + `FFTConvolver::process()` + aritmética de bypass/mix/level.

---

## 8. Impacto no Build e Dependências

### Novas dependências

| Crate | Justificativa |
|---|---|
| `rusqlite` (feature `bundled`) | Core do requisito de database; `bundled` = sem dependência de sistema |
| `blake3` | Hashing de conteúdo para identidade + dedup + verificação de integridade |
| `dirs` | Cross-platform data dir (minúsculo, sem deps transitórios) |
| `serde` + `serde_json` | Serialização do `standalone.json` (provavelmente já é dep transitória via nih_plug) |

### Dependências reutilizadas (já existentes)

| Crate | Uso |
|---|---|
| `arc-swap` (1.7.1) | Handoff atômico do convolver |
| `fft-convolver` (0.3) | Convolução particionada |
| `hound` (3.5) | Decode de WAV |
| `rfd` (0.14) | File dialog nativo |
| `rubato` (1.0) | Resampling de IR |
| `egui_knob` (0.2) | Knobs no painel cabinet |

### `build.rs`

**Inalterado.** `rusqlite bundled` compila seu próprio SQLite independentemente do pipeline Faust/Mojo. Sem novas etapas de build.

### Impacto no binário

- SQLite bundled: ~1.2 MB adicional no binário.
- BLAKE3: ~50 KB.
- IRs em BLOB: ~10-100 KB por IR típico. Com 20 IRs carregados: ~2 MB no DB em disco. Não afeta o binário.

---

## 9. Caminho de Migração

### Estado atual

- `cabinet_ir.wav` em `neural/drive/` carregado por **path absoluto hardcoded** (`/home/jao/VSCode/distortion/meu-novo-plugin/neural/drive/cabinet_ir.wav`).
- Código de loading duplicado em `lib.rs` (plugin) e `standalone.rs`.
- Falha silenciosa se arquivo não encontrado.

### Migração

1. **Embed do IR default no binário:**
   ```rust
   const DEFAULT_CABINET_IR: &[u8] = include_bytes!("../neural/drive/cabinet_ir.wav");
   ```

2. **Primeira inicialização (seed):**
   - Ao abrir DB, se `SELECT COUNT(*) FROM cabinet_irs == 0`:
     - Calcular `blake3(DEFAULT_CABINET_IR)`.
     - `INSERT INTO cabinet_irs` com `ir_data = DEFAULT_CABINET_IR.to_vec()`, nome = "Default Cabinet".
     - Setar `cabinet_state.selected_hash` para o hash.
   - Construir `CabinetRuntime` com os bytes embedados.

3. **Remoção do path hardcoded:**
   - Remover `const IR_BASE: &str = "/home/jao/..."` de `lib.rs` e `standalone.rs`.
   - Remover loading com `hound::WavReader::open(p)` do código de init.
   - Substituir por `CabinetLibrary::get_ir_by_hash(hash)` → bytes → `CabinetEngine::build_runtime(bytes, sample_rate)`.

4. **Falha explícita (nunca silenciosa):**
   - Se DB não tem entradas e o `include_bytes!` falhar (não deve acontecer em build normal): cabinet stage vira pass-through com UI mostrando "No cabinet IR loaded".
   - Se IR selecionado não está no DB: pass-through + aviso na UI.

5. **Arquivo `neural/drive/cabinet_ir.wav` mantido no repositório** apenas como asset de seed. Cópia autoritativa passa a ser o DB após primeira execução.

---

## 10. Estimativa de Implementação

| Tarefa | Complexidade | Depende de |
|---|---|---|
| `src/core/cabinet/types.rs` — tipos compartilhados | Baixa | — |
| `src/core/cabinet/library.rs` — SQLite CRUD | Média | types.rs |
| `src/core/cabinet/runtime.rs` — CabinetRuntime + build | Média | types.rs |
| `src/core/cabinet/engine.rs` — ArcSwap handoff + ramp | Média | runtime.rs |
| `src/core/cabinet/mod.rs` — re-exports | Baixa | todos acima |
| `src/core/ui/cabinet_panel.rs` — painel UI | Alta | library.rs, types.rs |
| `src/core/ui/signal_chain.rs` — nó CABINET | Baixa | — |
| `src/lib.rs` — integração plugin | Alta | engine.rs, library.rs |
| `src/bin/standalone.rs` — integração standalone | Alta | engine.rs, library.rs |
| `src/core/state/plugin_params.rs` — novos params | Baixa | — |
| `Cargo.toml` — novas deps | Baixa | — |
| Migração: `include_bytes!` + seed | Média | library.rs |
| Testes | Média | — |

---

## A. Apêndice: Decisões Rejeitadas

Esta seção documenta alternativas consideradas e por que foram rejeitadas.

### JSON + arquivos soltos (rejeitado)
- Perde atomicidade (metadata e WAV podem dessincronizar).
- Arquivos órfãos/parciais.
- O requisito explicitamente pede database.

### UUID como identidade de IR (rejeitado)
- Sem dedup automático — precisa de check manual antes de INSERT.
- Row IDs não são portáveis entre DBs — presets quebram em migração/reset.
- Adiciona dependência `uuid` sem ganho sobre content hash.

### Filas `rtrb` para handoff (rejeitado)
- Abstração errada: `rtrb` é para streaming, não ownership handoff.
- Introduz capacity concerns e backpressure desnecessários.
- `ArcSwap` já é dependência, zero-cost na thread de áudio, semanticamente correto.

### `cabinet_mix` como opcional/deferível (rejeitado)
- Controle fundamental em amp modeling (dry/wet blend).
- Adicionar depois quebra presets existentes.
- Custo de implementação é trivial (~10 linhas no process).

### Rampa de mute na troca de IR como opcional (rejeitado)
- Trocar FFTConvolver sem mute causa glitch de áudio garantido.
- Não é "nice to have" — é correção. 5ms é imperceptível.

---

## B. Apêndice: Agentes Participantes

| Fase | Agent | Tarefa |
|---|---|---|
| Exploração | pi (`explore-signal-chain`) | Pipeline, FFTConvolver, fluxo de parâmetros |
| Exploração | codex (`explore-storage-state`) | Estado, parâmetros, persistência |
| Exploração | claude_code (`explore-ui-and-build`) | UI, build, dependências |
| Proposta | claude_code (`spec-proposal-A`) | Proposta de design (content-hash, rtrb) |
| Proposta | codex (`spec-proposal-B`) | Proposta de design (UUID, ArcSwap) |
| Debate | pi (`debate-reconcile`) | Comparação, argumentação, especificação final |
