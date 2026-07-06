# EQ + Neural Amp Fix — Tasks

**Design**: `.specs/eq-neural-fix/SPEC.md`
**Status**: In Progress

---

## Execution Plan

### Phase 1: Foundation (Sequential — Fixes diretos, dependência de compilação)

```
T1 → T2 → T3
```

### Phase 2: Core Implementation (ONNX Integration + Unificação)

```
T3 complete, então:
     ┌→ T4 [P] ─────────────────────────┐
T3 ─┼→ T5 [P] (drive unificado)         ├─→ T7
     └→ T6 [P] (pre-EQ Faust) ───────────┘
```

### Phase 3: Integration

```
T4 + T5 + T6 complete → T7 (build + verify)
```

---

## Task Breakdown

### T1: Remover ma.tanh do pipeline EQ ✅ COMPLETO

**What**: Remover `ma.tanh` do chain de EQ em `dsp/main.dsp` e regenerar `FaustModule.hpp`. O EQ SHALL ser linear — sem compressão de soft-clipping no final.

**Where**: `dsp/main.dsp`, `dsp/FaustModule.hpp` (regenerado)

**Depends on**: None

**Requirement**: EQ-01, EQ-02, EQ-03

**Done when**:
- [ ] `eq_chain = eq_low : eq_mid : eq_high` (sem `ma.tanh`)
- [ ] `FaustModule.hpp` regenerado via `faust -lang cpp`
- [ ] `make build` compila sem erros
- [ ] `cargo check` passa (bindings atualizados)

**Tests**: none (teste manual: sine sweep + FFT)
**Gate**: build

**Commit**: `fix(eq): remove ma.tanh from EQ chain for linear response`

---

### T2: Alargar range de Q para 0.1–10.0 ✅ COMPLETO

**What**: Mudar o Q mínimo de 0.707 para 0.1 em todos os knobs de EQ (plugin params e standalone). Isso permite ajustes de tom broad (Q=0.1 ≈ 2 oitavas de largura) e surgical (Q=10.0 ≈ 1/3 oitava).

**Where**: `src/core/state/plugin_params.rs` (linhas 152-167), `src/bin/standalone.rs` (linhas 991-1007)

**Depends on**: T1 (para rebuild do Faust)

**Requirement**: Q-01, Q-02

**Done when**:
- [ ] `eq_low_q` range: `min: 0.1, max: 10.0`
- [ ] `eq_mid_q` range: `min: 0.1, max: 10.0`
- [ ] `eq_high_q` range: `min: 0.1, max: 10.0`
- [ ] Standalone knobs com mesmo range
- [ ] `make build` compila

**Tests**: none
**Gate**: build

**Commit**: `fix(eq): widen Q range from 0.707 to 0.1 for industry-standard tonal shaping`

---

### T3: Adicionar ONNX Runtime ao projeto Rust ✅ COMPLETO

**What**: Adicionar `ort` (ONNX Runtime Rust bindings) como dependência em `Cargo.toml`. O ONNX Runtime é necessário para inferência do modelo WaveNet em tempo real no Mojo. Validar que o modelo ONNX existente é compatível (formato opset).

**Where**: `Cargo.toml`

**Depends on**: None

**Requirement**: NN-01

**Done when**:
- [ ] `ort = "2.0"` ou versão mais recente adicionada em `[dependencies]`
- [ ] `cargo fetch` baixa a dependência sem conflito
- [ ] `cargo check` passa com novo crate
- [ ] Modelo `wavenet_drive.onnx` é válido (onnx version/ir_version verificado)
- [ ] Doc: criar `neural/ONNX_INTEGRATION.md` com notas sobre o modelo

**Tests**: none
**Gate**: build

**Commit**: `feat(neural): add ONNX Runtime (ort) crate for WaveNet inference`

---

### T4: Integrar ONNX no Mojo via ONNX Runtime [P]

**What**: Reescrever `neural/main.mojo` para carregar e executar `wavenet_drive.onnx` via ONNX Runtime. O Mojo age como wrapper que chama o ONNX Runtime (ou alternativamente, um novo crate Rust `src/bridge/wavenet.rs` faz a inferência e Mojo apenas faz o bypass). **Decisão técnica a ser tomada**: se Mojo pode chamar `ort` diretamente ou se o Rust faz o bridge. Recomendação: criar `src/bridge/wavenet.rs` em Rust (ONNX via `ort`) + Mojo recebe saída pronta.

**Where**: `src/bridge/wavenet.rs` (novo), `src/bridge/mojo.rs` (modificar), `neural/main.mojo`

**Depends on**: T3

**Requirement**: NN-01, NN-02, NN-03, NN-04

**Done when**:
- [ ] `WavenetProcessor` (impl `ExternalProcessor`) carrega `wavenet_drive.onnx`
- [ ] `process_block` executa inferência ONNX in-place
- [ ] Fallback para `tanh_polynomial` se ONNX falha (arquivo ausente, opset incompatível)
- [ ] `cargo check` passa
- [ ] Saída neural é verificavelmente diferente de `tanh(x)` (análise espectral de sine wave)

**Tests**: none (teste manual com áudio real)
**Gate**: build

**Commit**: `feat(neural): integrate ONNX WaveNet model via ort crate with tanh fallback`

---

### T5: Unificar lógica de drive entre standalone e plugin [P]

**What**: Padronizar que `neural_drive` é um multiplicador linear de 0.0–3.0x (não dB) em ambos os binários. `neural_output_gain` é o ganho de saída (0.0–2.0x). Remover a lógica de "aplicar volume duas vezes" no standalone.

**Where**: `src/bin/standalone.rs` (linhas 344-353), `src/bridge/mojo.rs`, `src/core/state/plugin_params.rs` (linhas 115-141)

**Depends on**: T4 (para garantir que Mojo aceita os parâmetros corretos)

**Requirement**: DR-01, DR-02, DR-03

**Done when**:
- [ ] Standalone `set_drive(val)` recebe `0.0–3.0` (linear)
- [ ] Plugin `neural_drive` range: `min: 0.0, max: 3.0` (linear, não `db_to_gain`)
- [ ] `neural_output_gain` range: `min: 0.0, max: 2.0` (linear)
- [ ] Standalone NÃO aplica `* neural_vol` duas vezes
- [ ] `make build` compila ambos
- [ ] `make run` (standalone) inicia sem crash

**Tests**: none
**Gate**: build

**Commit**: `fix(dsp): unify drive/gain logic between standalone and plugin`

---

### T6: Substituir Pre-EQ IR curta por Faust parametric filter [P]

**What**: Trocar a convolução de `pre_eq_ir.wav` (256 samples, inútil) por um filtro paramétrico de 2 estágios (low-pass shelf + high-pass shelf) implementado via Faust — equivalente a um tone stack. Os parâmetros são lidos dos knobs de EQ (ou de um knob separado "Pre-EQ Tone").

**Where**: `src/lib.rs` (pipeline), `src/bin/standalone.rs` (pipeline), `dsp/main.dsp` (adicionar estágio pre-EQ)

**Depends on**: T1 (Faust regenerado)

**Requirement**: PE-01, PE-02

**Done when**:
- [ ] Pipeline não tenta carregar `pre_eq_ir.wav` (removido do path)
- [ ] Estágio pre-EQ usa Faust com 2 filtros paramétricos
- [ ] Plugin e standalone carregam pre-EQ via FaustProcessor existente
- [ ] Se pre-EQ não inicializa, processamento continua sem crash
- [ ] `make build` compila

**Tests**: none
**Gate**: build

**Commit**: `refactor(pre-eq): replace short IR convolution with Faust parametric filter`

---

### T7: Verificar pipeline completo

**What**: Build final + teste desanvolvimento (não é teste automático — é validação manual documentada).

**Where**: Todo o projeto

**Depends on**: T4, T5, T6

**Requirement**: EQ-04, DR-03

**Done when**:
- [ ] `make build --release` compila sem warnings de deprecation
- [ ] `cargo clippy -- -D warnings` passa (se aplicável)
- [ ] `make run` inicia o standalone sem crash
- [ ] Bypass do EQ funciona corretamente
- [ ] Doc: `docs/VERIFICATION.md` com procedimento de teste manual (sine sweep + FFT)

**Tests**: none (manual)
**Gate**: build

**Commit**: `docs(verification): add manual verification procedure for EQ and neural fix`

---

## Parallel Execution Map

```
Phase 1 (Sequential):
  T1 (EQ tanh) ──→ T2 (Q range) ──→ T3 (ONNX dep)

Phase 2 (Parallel):
  T3 complete, then:
    ├── T4 [P] (ONNX integration)
    ├── T5 [P] (drive unificação)
    └── T6 [P] (pre-EQ Faust)

Phase 3 (Sequential):
  T4 + T5 + T6 complete → T7 (integration verify)
```

---

## Granularity Check

| Task | Scope | Status |
|---|---|---|
| T1: Remove ma.tanh | 1 arquivo .dsp | ✅ Granular |
| T2: Widen Q range | 2 arquivos params | ✅ Granular |
| T3: ONNX dep | Cargo.toml | ✅ Granular |
| T4: ONNX integration | 3 arquivos (Rust + Mojo) | ✅ Granular |
| T5: Unify drive logic | 3 arquivos | ✅ Granular |
| T6: Pre-EQ Faust | Pipeline em 2 arquivos | ✅ Granular |
| T7: Verify pipeline | Projeto inteiro | ✅ Granular |

---

## Diagram-Definition Cross-Check

| Task | Depends On (body) | Diagram Shows | Status |
|---|---|---|---|
| T1 | None | T1 → T2 | ✅ Match |
| T2 | T1 | T1 → T2 | ✅ Match |
| T3 | None | T2 → T3 | ✅ Match |
| T4 | T3 | T3 → T4 | ✅ Match |
| T5 | T4 | T3 → T5 | ✅ Match (T5 deps on T4 para parâmetros corretos) |
| T6 | T1 | T3 → T6 | ✅ Match (T6 deps on Faust regenerado) |
| T7 | T4, T5, T6 | T4+T5+T6 → T7 | ✅ Match |

---

## Test Co-location Validation

Este projeto não possui suíte de testes (`no #[test]` functions). Todos os Gates são `build` — verificação via compilação e teste manual. Tasks marked `none` para testes estão corretas.

| Task | Code Layer | Matrix Requires | Task Says | Status |
|---|---|---|---|---|
| T1 | DSP Faust | none | none | ✅ OK |
| T2 | Params | none | none | ✅ OK |
| T3 | Cargo deps | none | none | ✅ OK |
| T4 | Bridge Neural | none | none | ✅ OK |
| T5 | DSP params | none | none | ✅ OK |
| T6 | DSP pipeline | none | none | ✅ OK |
| T7 | All | none | none | ✅ OK |
