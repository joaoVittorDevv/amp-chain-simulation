# Cross-Platform Roadmap — Progresso

Última atualização: 2026-07-10

**Nota:** `dsp/MlcZeroVModule.hpp` regenerado de `mlc_zero_v.dsp` com o makeup gain correto (2.24 → estava stale com 2.5). Commit `76069e9`, merge `ed31bc4`.

---

## ✅ Phase 0 — Fundação de Build (T1–T5)

**Status:** Concluído e mergeado em `main`
**Commit:** `dd2d5e4`

| Task | Descrição | Status |
|------|-----------|--------|
| T1 | `build-support` crate com `find_faust()` PATH-only | ✅ |
| T2 | `build.rs` usa `build-support` | ✅ |
| T3 | `f_size_t` → `size_t` nos headers C | ✅ |
| T4 | `cargo xtask` unificado | ✅ |
| T5 | `Makefile` delegando para xtask | ✅ |

**Nota:** `find_faust()` original só usava PATH. Corrigido posteriormente
com fallback paths (branch `fix-find-faust-fallbacks`, ver Phase 3).

---

## ✅ Phase 1 — Backend Neural (T6–T10)

**Status:** Concluído e mergeado em `main`
**Commit:** `ef6dd6f`

| Task | Descrição | Status |
|------|-----------|--------|
| T6 | `RustNeuralProcessor` genérico | ✅ |
| T7 | Feature `have_mojo` + compile-time gate | ✅ |
| T8 | Type alias `NeuralProcessor` | ✅ |
| T9 | `VariantRegistry` registra `RustNeuralProcessor` | ✅ |
| T10 | Testes de equivalência Mojo vs Rust | ✅ |

---

## ✅ Phase 2 — Correção de Áudio (T11–T16)

**Status:** Concluído e mergeado em `main`
**Commits:** `ec4ac4e` → `6fc2787` (7 commits)

| Task | Descrição | Status |
|------|-----------|--------|
| T11 | `sample_convert` cross-format PCM | ✅ |
| T12 | Extrair `StandalonePipeline` do callback de input | ✅ |
| T13 | Chunking do callback de input (não truncar) | ✅ |
| T14 | Aceitar I32/I16 nativos além de F32 | ✅ |
| T15 | Testes de equivalência entre formatos | ✅ |
| T16 | Prova RT-safety (allocation-free) | ✅ |

**Arquivos novos:**
- `src/core/dsp/sample_convert.rs`
- `src/core/dsp/standalone_pipeline.rs`
- `tests/format_equivalence.rs` (217 linhas)
- `tests/pipeline_golden.rs` (285 linhas)
- `tests/rt_safety.rs` (196 linhas)

---

## ✅ Phase 3 — Windows/ASIO (T17–T20, T30)

**Status:** Concluído e mergeado em `main`
**Commits:** 5 merge commits + 1 squash

| Task | Descrição | Branch | Revisão |
|------|-----------|--------|---------|
| — | Corrige `find_faust()` com fallback paths | `fix-find-faust-fallbacks` | ✅ |
| T17 | Feature `asio` opt-in, target-gated | `phase-3-T17-asio-feature` | ✅ |
| T18 | ASIO duplex — Device único para input+output | `phase-3-T18-asio-duplex` | ✅ |
| T20 | Negociação de stream config com fallback | `phase-3-T20-config-range` | ✅ (2 rodadas) |
| T30 | Identidade estável por enum_index | `phase-3-T30-device-identity` | ✅ |
| T19 | Dispositivos não-suportados visíveis (desabilitados) | `phase-3-T19-unsupported-formats` | ✅ |

**Novos arquivos:**
- `src/core/audio_config.rs` — negociação de config (708 linhas, 17 testes)
- `src/core/device_identity.rs` — identidade estável (163 linhas, 7 testes)
- `src/core/device_context.rs` — contexto de dispositivo com usable/formatos (337 linhas, 9 testes)

---

## ✅ Phase 4 — Robustez do Ciclo de Vida (T21–T25, T31–T32)

**Status:** 5/7 concluído (T31, T32 pendentes)

| Task | Descrição | Commit | Status |
|------|-----------|--------|--------|
| T21 | Reconciliação de sample rate (output-as-master) | `962e790` | ✅ |
| T22 | Rubato Async resampler no pipeline de input | `546a982` | ✅ |
| T23 | AudioStatus lock-free + hotplug refresh | `5ec251c` | ✅ |
| T24 | Slider mostra buffer real do driver | `7309eec` | ✅ |
| T25 | `compute_latency()` + `set_latency_samples` em `initialize()` | `0036e6d` | ✅ |
| T31 | Testes de stress multi-dispositivo | — | ⏳ |
| T32 | Testes de recuperação de erro | — | ⏳ |

### T22 — Rubato Async Resampler

**Status:** ✅ Concluído e mergeado em `main`
**Merge commit:** `546a982` (2 commits no branch `phase-4-T22-resampler`)

**Commits:**
- `60cc4c7` feat: rubato Async resampler with staging buffer for input stream (T22)
- `f326c2f` feat: wire rubato Async resampler into standalone input pipeline (T22)

**Entregáveis:**
- `src/core/dsp/rt_resampler.rs` — `RtResampler` com API `feed(L, R, callback)`, staging buffer incremental, nunca dá overflow
- `rubato` v1.0.1 como dependência
- Integração nos 3 callbacks de input (F32, I32, I16): resampler `Some` quando input ≠ output rate, `None` (zero overhead) quando taxas batem
- Ring buffer de playthrough dimensionado pelo pior caso de resample ratio
- `emit_processed_block()` + `publish_frames()` extraídos das duplicações inline
- 7 testes unitários

### T23 — AudioStatus Lock-Free

**Status:** ✅ Concluído e mergeado em `main`
**Merge commit:** `5ec251c`

**Entregáveis:**
- `src/core/audio_status.rs` — `AudioStatus` com `code: AtomicU8`, `dropped_errors: AtomicU32`, `underruns/overflows: AtomicU64`
- `set_error()` usa `compare_exchange` — primeiro erro ganha
- `take_error()` usa `swap` atômico único
- 5 callbacks de erro (F32/I32/I16 in, F32/I16 out) trocados de `eprintln!` para `audio_status.set_error()`
- Banner de erro no top panel com botão "Detalhes"
- Botão "🔄 Atualizar Dispositivos" no settings panel
- Teste de concorrência: 8 threads + 1 consumer, nenhum erro perdido

### T24 — Slider de Latência + Driver Buffer

**Status:** ✅ Concluído e mergeado em `main`
**Merge commit:** `7309eec` (2 commits no branch `phase-4-T24-latency-slider`)

**Commits:**
- `1512382` feat: add format_latency helper + test (T24 part 1)
- `52a8ac0` feat: latency slider shows real driver buffer; ring slack only (T24)

**Entregáveis:**
- `format_latency()` em `audio_config.rs` com teste `256@48000 = "5.3 ms"`
- `AtomicU64` para `driver_buffer_frames` + `driver_buffer_rate` observados nos 3 callbacks
- Label na UI: `"Driver buffer: N samples @ SR Hz = X.X ms"`
- Slider reetiquetado para `"Ring buffer slack"` — desacoplado do `BufferSize`

### T25 — DAW Latency Reporting

**Status:** ✅ Concluído e mergeado em `main`
**Merge commit:** `0036e6d`

**Entregáveis:**
- `compute_latency()` em `src/lib.rs`: soma `cabinet_fft_delay + resampler_delay`, retorna 0 para estágios bypassed
- Cabinet UPC reporta 0 (convolução particionada uniforme é zero-latency)
- `set_latency_samples()` chamado uma vez em `initialize()`, guardado por `last_reported_latency`
- Removido de `process()` (não mais chamado por bloco)
- 2 testes: `compute_latency_sums_cabinet_and_resampler_delays`, `compute_latency_omits_inactive_cabinet`

---

## ✅ Phase 5 — Validação & CI (T26–T29)

**Status:** 3/4 concluído (T28 pendente)

| Task | Descrição | Commit | Status |
|------|-----------|--------|--------|
| T26 | CI matrix (Linux + macOS + Windows) | — | ✅ |
| T27 | Docs de cross-platform e troubleshooting | `f5fc8d4` | ✅ |
| T28 | Performance gate (< 5% regressão) | — | ⏳ |
| T29 | Bundle multi-plataforma (Verb::Bundle + test) | `a1fa269` | ✅ |

### T27 — Documentação Cross-Platform

**Status:** ✅ Concluído e mergeado em `main`
**Merge commit:** `f5fc8d4`

**Entregáveis:**
- `docs/BUILD.md` — guia de build com pré-requisitos, troubleshooting, perfis de release
- `docs/UAT.md` — manual de User Acceptance Testing com cenários por SO
- `README.md` — link para BUILD.md
- `CLAUDE.md` — notas de segurança de build

### T29 — Bundle Multi-Plataforma

**Status:** ✅ Concluído e mergeado em `main`
**Merge commit:** `a1fa269`

**Entregáveis:**
- `Verb::Bundle` no `xtask` — chama `cargo xtask pre-build && cargo xtask bundle distortion --release`
- `tests/bundle_deps.rs` — 102 linhas verificando que dependências de bundle estão instaladas

---

## 🐛 Bug Fix: MLC Header Stale

**Status:** ✅ Corrigido e mergeado em `main`
**Merge commit:** `ed31bc4` (branch `fix/mlc-header-stale`, commit `76069e9`)

**Problema:** `dsp/MlcZeroVModule.hpp` estava stale — makeup gain `2.5f` vs `2.24f` no fonte Faust (`mlc_zero_v.dsp:190`). Diferença audível de ~0.95 dB.  
**Correção:** Header regenerado com `faust -lang cpp -cn mlczerov -vec -I faust-ddsp -i dsp/mlc_zero_v.dsp`.

---

## Resumo

| Fase | Tasks | Status |
|------|-------|--------|
| Phase 0 — Build | T1–T5 | ✅ Mergeado |
| Phase 1 — Neural | T6–T10 | ✅ Mergeado |
| Phase 2 — Áudio | T11–T16 | ✅ Mergeado |
| Phase 3 — Windows/ASIO | T17–T20, T30 | ✅ Mergeado |
| Phase 4 — Robustez | T21–T25, T31–T32 | ✅ 5/7 (T31, T32 pendentes) |
| Phase 5 — Validação | T26–T29 | ✅ 3/4 (T28 pendente) |

**Total:** 29/32 tasks concluídas (90.6%)

**Pendentes:** T28 (perf gate), T31 (stress multi-dispositivo), T32 (recuperação de erro)
