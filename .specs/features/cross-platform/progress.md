# Cross-Platform Roadmap — Progresso

Última atualização: 2026-07-09

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

## ⏳ Phase 4 — Robustez do Ciclo de Vida (T21–T25, T31–T32)

**Status:** Pendente

| Task | Descrição |
|------|-----------|
| T21 | Resampler para drift de clock |
| T22 | Recuperação de erro de dispositivo |
| T23 | Device hotplug (reconectar stream) |
| T24 | Telemetria de underrun/overrun |
| T25 | Log de latência e buffer |
| T31 | Testes de stress multi-dispositivo |
| T32 | Testes de recuperação de erro |

---

## ⏳ Phase 5 — Validação & CI (T26–T29)

**Status:** Pendente

| Task | Descrição |
|------|-----------|
| T26 | CI matrix (Linux + macOS) |
| T27 | Docs de cross-platform e troubleshooting |
| T28 | Bundle multi-plataforma |
| T29 | Performance gate (< 5% regressão) |

---

## Resumo

| Fase | Tasks | Status |
|------|-------|--------|
| Phase 0 — Build | T1–T5 | ✅ Mergeado |
| Phase 1 — Neural | T6–T10 | ✅ Mergeado |
| Phase 2 — Áudio | T11–T16 | ✅ Mergeado |
| Phase 3 — Windows/ASIO | T17–T20, T30 | ✅ Mergeado |
| Phase 4 — Robustez | T21–T25, T31–T32 | ⏳ Pendente |
| Phase 5 — Validação | T26–T29 | ⏳ Pendente |

**Total:** 22/32 tasks concluídas (69%)
