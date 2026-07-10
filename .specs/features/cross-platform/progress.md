# Cross-Platform Roadmap — Progresso

Última atualização: 2026-07-10 — **COMPLETO 32/32 (100%)** 🎉

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

---

## ✅ Phase 3 — Windows/ASIO (T17–T20, T30)

**Status:** Concluído e mergeado em `main`

| Task | Descrição | Status |
|------|-----------|--------|
| T17 | Feature `asio` opt-in, target-gated | ✅ |
| T18 | ASIO duplex — Device único para input+output | ✅ |
| T20 | Negociação de stream config com fallback | ✅ |
| T30 | Identidade estável por enum_index | ✅ |
| T19 | Dispositivos não-suportados visíveis (desabilitados) | ✅ |

---

## ✅ Phase 4 — Robustez do Ciclo de Vida (T21–T25, T31–T32)

**Status:** 7/7 concluído ✅

| Task | Descrição | Commit | Status |
|------|-----------|--------|--------|
| T21 | Reconciliação de sample rate (output-as-master) | `962e790` | ✅ |
| T22 | Rubato Async resampler no pipeline de input | `546a982` | ✅ |
| T23 | AudioStatus lock-free + hotplug refresh | `5ec251c` | ✅ |
| T24 | Slider mostra buffer real do driver | `7309eec` | ✅ |
| T25 | `compute_latency()` + `set_latency_samples` em `initialize()` | `0036e6d` | ✅ |
| T31 | Wire AudioStatus telemetry into audio callbacks | `0e12beb` | ✅ |
| T32 | Clock drift controller for duplex resampling | `c971938` | ✅ |

### T31 — AudioStatus Telemetry

**Merge commit:** `0e12beb`  
**Commit:** `6b84a7c`

**Entregáveis:**
- `publish_frames` com `note_overflow()` em 3 sites de push failure (mix, out_l, out_r)
- Output callbacks (F32, I16) com `match pop()` + `note_underrun()` no Err
- AudioStatus clones nos closures de output callback
- UI com seção "📊 Telemetria de Áudio" mostrando contadores underrun/overflow quando não-zero
- Testes: `cargo test --lib audio_status` → 3 passed

### T32 — Clock Drift Controller

**Merge commit:** `c971938`  
**Commit:** `aa2831b`

**Entregáveis:**
- `RtResampler::trim_ratio(fill_ratio)` — controlador proporcional KP=0.05, TARGET_FILL=0.5, ±0.1% clamp
- Smoothing via 16-sample history ring (média aritmética móvel)
- Handoff lock-free via `DriftObservation` (AtomicU64 fill bits + sequence)
- Output callbacks calculam fill via `consumer.slots()`, publicam ~1 vez/segundo
- Input callbacks (F32/I32/I16) chamam `apply_to()` que invoca `trim_ratio` no resampler
- 3 testes: clamping, averaging, history → 10 passed total

---

## ✅ Phase 5 — Validação & CI (T26–T29)

**Status:** 4/4 concluído ✅

| Task | Descrição | Commit | Status |
|------|-----------|--------|--------|
| T26 | CI matrix (Linux + macOS + Windows) | — | ✅ |
| T27 | Docs de cross-platform e troubleshooting | `f5fc8d4` | ✅ |
| T28 | Performance gate (< 5% regressão) | `d2771ce` | ✅ |
| T29 | Bundle multi-plataforma (Verb::Bundle + test) | `a1fa269` | ✅ |

### T27 — Documentação Cross-Platform

**Merge commit:** `f5fc8d4`

**Entregáveis:**
- `docs/BUILD.md` — guia de build com pré-requisitos, troubleshooting, perfis de release
- `docs/UAT.md` — manual de User Acceptance Testing com cenários por SO

### T28 — Performance Gate

**Merge commit:** `d2771ce`  
**Commit:** `8933c4f`

**Entregáveis:**
- 4 criterion benchmarks em `benches/`: `dsp_pipeline`, `faust_eq`, `neural_drive`, `resampler`
- Cold + hot benchmarks para DSP pipeline em 5 buffer sizes (64–1024)
- `benches/check_regressions.py` — parseia `estimates.json`, falha em >5% regressão
- `Makefile`: `bench-baseline` (salva baseline) e `bench` (compara + regression gate)
- Verificação: `cargo bench --no-run` compila todos, `cargo check` limpo

### T29 — Bundle Multi-Plataforma

**Merge commit:** `a1fa269`

---

## 🐛 Bug Fixes

| Bug | Descrição | Commit | Status |
|-----|-----------|--------|--------|
| MLC Header Stale | `dsp/MlcZeroVModule.hpp` regenerado (2.5f → 2.24f, ~0.95 dB) | `ed31bc4` | ✅ |
| `find_faust` fallbacks | Busca em diretórios padrão do Faust | `fix-find-faust-fallbacks` | ✅ |

---

## 📄 Docs Adicionais

| Arquivo | Descrição | Commit |
|---------|-----------|--------|
| `docs/WINDOWS.md` | Windows/ASIO setup, troubleshooting, testing | `9a3d21b` (merge `06c967e`) |

**Correções aplicadas (review opus):**
- `LIBCLANG_PATH` → `bin/` (não `lib/`)
- `ASIO_DIR` → `CPAL_ASIO_DIR` (env var correta)
- `cargo xtask --features asio` → raw cargo (xtask dropava a flag)
- `xtask/src/main.rs` agora passa `--features` corretamente
- CI corrigido de `ASIO_DIR` para `CPAL_ASIO_DIR`

---

## Resumo Final

| Fase | Tasks | Status |
|------|-------|--------|
| Phase 0 — Build | T1–T5 | ✅ Mergeado |
| Phase 1 — Neural | T6–T10 | ✅ Mergeado |
| Phase 2 — Áudio | T11–T16 | ✅ Mergeado |
| Phase 3 — Windows/ASIO | T17–T20, T30 | ✅ Mergeado |
| Phase 4 — Robustez | T21–T25, T31–T32 | ✅ 7/7 |
| Phase 5 — Validação | T26–T29 | ✅ 4/4 |

**Total: 32/32 tasks concluídas (100%) 🎉**

**HEAD final:** `9a3d21b` — Windows/ASIO docs merge
