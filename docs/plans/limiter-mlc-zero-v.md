# Plano: Brickwall Limiter para MLC ZERO V Amp

**Data:** 2026-07-07
**Status:** Planejamento aprovado
**Área de impacto:** `src/core/dsp/`, `src/lib.rs`, `src/bin/standalone.rs`

---

## 1. Objetivo

Adicionar um **limiter brickwall** ao final da cadeia de sinal do MLC ZERO V Amp para garantir que o sinal **nunca clipe**, independentemente de quão alto o usuário configure gain, master, ou qualquer outro parâmetro.

## 2. Diagnóstico Atual

### Pipeline de processamento (ordem exata, `src/lib.rs:880–1062`)

```
Input Routing → Faust EQ → Pre-EQ Convolver → Amp (MLC ZERO V / Neural)
    → Cabinet IR → Master Gain (-30/+30 dB) → Neural Amp Vol → NaN Sanitization → Output
```

### Pontos de clipping

| Fonte | Risco |
|---|---|
| MLC ZERO V `gain` (-60 a 0 dB) + ressonância de EQ | Médio |
| MLC ZERO V `master` (-60 a 0 dB) | Médio |
| Cabinet IR (ganho de ressonância do falante) | Baixo-Médio |
| **Master Gain (-30 a +30 dB)** | **Alto — ganho de até 31.6×** |
| Neural Amp Volume (-24 a +12 dB) | Médio |

O Master Gain de +30 dB multiplica o sinal por ~31.6, garantindo que **qualquer sinal acima de -30 dBFS vai clipar**. Não há absolutamente nenhuma proteção.

### Ponto de inserção recomendado

**Entre Stage 5 (Master Gain) e Stage 7 (NaN Sanitization).** No código:

- **Plugin** (`src/lib.rs`): após linha 1039 (master gain loop), antes da sanitização (linha 1055)
- **Standalone** (`src/bin/standalone.rs`): equivalente após o master gain loop, antes da sanitização

Isto captura clipping de TODAS as fontes combinadas: EQ, amp, cabinet, master gain.

---

## 3. Design do Limiter

### Tipo

**Envelope-following peak limiter** — zero-latência, puro Rust, sem FFI.

- **Detector de pico** com attack instantâneo (sample-accurate) e release exponencial
- **Gain reduction** aplicada por multiplicação (divisão simples: `sample *= target_peak / current_peak`)
- **Sem oversampling** — aceita inter-sample peaks como tradeoff por zero-latência
- **Sem lookahead** — mantém latência 0 do plugin (crítico para uso em tempo real)

### Arquitetura

```
entrada (já com master gain aplicado)
    │
    ▼
┌─────────────────────────┐
│ PeakLimiter             │
│  ├─ ceiling: f32 (0-1)  │  ← teto linear (ex: 10^(-1/20) ≈ 0.891)
│  ├─ envelope: f32        │  ← estado interno: pico atual do envelope
│  ├─ release_coeff: f32   │  ← calculado do release em ms × sample_rate
│  │                       │
│  └─ process(sample)      │
│    1. abs_sample = |sample|
│    2. if abs_sample > envelope: envelope = abs_sample     (attack instantâneo)
│    3. else: envelope += (abs_sample - envelope) × coeff   (release exponencial)
│    4. if envelope > ceiling: sample *= ceiling / envelope (gain reduction)
│    5. return sample                                       (garantia: |sample| ≤ ceiling)
└─────────────────────────┘
    │
    ▼
saída (garantida ≤ ceiling)
```

### Por que este design

| Decisão | Razão |
|---|---|
| **Zero-latência** | Plugin de guitarra em tempo real. Lookahead exigiria `set_latency_samples()` e quebraria monitoramento |
| **Envelope follower** | Evita aliasing de hard-clip (`sample.clamp()`) que gera distorção desagradável |
| **Attack instantâneo** | Picos são capturados sample-accurate — nenhum overshoot |
| **Release exponencial** | Suave, sem pumping audível. Controlado por parâmetro de usuário |
| **Puro Rust** | Sem FFI, sem Faust, sem dependências externas. Testável com `#[test]` |

---

## 4. Parâmetros

| Parâmetro | ID | Tipo | Range | Default | Suavização | Unidade |
|---|---|---|---|---|---|---|
| Limiter Enable | `limiter_enable` | `BoolParam` | — | `true` | — | — |
| Limiter Ceiling | `limiter_ceiling` | `FloatParam` | -12.0 a 0.0 dB | -1.0 dB | Linear 10ms | dB |
| Limiter Release | `limiter_release` | `FloatParam` | 10 a 500 ms | 50 ms | Linear 50ms | ms |

### Justificativa dos defaults

| Parâmetro | Default | Por quê |
|---|---|---|
| Ceiling | **-1.0 dB** | Margem de segurança de 1 dB para ISPs (inter-sample peaks). -0.3 dB seria suficiente mas -1.0 é mais seguro para conversão D/A e mp3 |
| Release | **50 ms** | Rápido o suficiente para não "pompar" em guitarras, lento o suficiente para não distorcer graves. Valor padrão em limiters de guitarra (ex: Precision Maximizer) |
| Enable | **true** | Usuário tem que explicitamente desligar se quiser correr risco de clip |

---

## 5. Arquivos a Modificar

### 5.1 Novo módulo: `src/core/dsp/limiter.rs` (~50 linhas)

```rust
/// Peak envelope-following brickwall limiter.
/// Zero-latency, sample-accurate attack, exponential release.
pub struct PeakLimiter {
    ceiling: f32,
    envelope: f32,
    release_coeff: f32,
}

impl PeakLimiter {
    pub fn new(ceiling_db: f32, release_ms: f32, sample_rate: f32) -> Self { ... }
    pub fn set_params(&mut self, ceiling_db: f32, release_ms: f32, sample_rate: f32) { ... }
    pub fn process(&mut self, sample: f32) -> f32 { ... }
    pub fn reset(&mut self) { ... }
}
```

### 5.2 `src/core/dsp/mod.rs`

Adicionar:
```rust
pub mod limiter;
```

### 5.3 `src/core/state/plugin_params.rs`

Adicionar 3 novos campos no struct `PluginParams`:

```rust
#[id = "lim_en"]
pub limiter_enable: BoolParam,
#[id = "lim_ceil"]
pub limiter_ceiling: FloatParam,
#[id = "lim_rel"]
pub limiter_release: FloatParam,
```

+ inicialização no `Default::default()`.

### 5.4 `src/lib.rs`

**No struct `BaseIO`:** adicionar campo `limiter: PeakLimiter`.

**Em `initialize()`:** inicializar o limiter com os defaults dos parâmetros.

**Em `process()`:**
1. Após `param.smoothed.next()` do master gain (~linha 870), adicionar leitura dos novos parâmetros:
   ```rust
   let limiter_enable = self.params.limiter_enable.value();
   let limiter_ceiling = self.params.limiter_ceiling.smoothed.next();
   let limiter_release = self.params.limiter_release.smoothed.next();
   ```
2. Após o loop de master gain (após linha 1039), adicionar o estágio do limiter:
   ```rust
   // ESTÁGIO 6: Brickwall Limiter
   if limiter_enable {
       self.limiter.set_params(limiter_ceiling, limiter_release, sample_rate);
       for sample in channel_samples.iter_mut() {
           *sample = self.limiter.process(*sample);
       }
   }
   ```

### 5.5 `src/bin/standalone.rs`

Mesmo padrão:
1. Adicionar `limiter: PeakLimiter` no struct de estado
2. Inicializar no `init()`
3. Aplicar após master gain, antes da sanitização

### 5.6 `src/core/ui/mlc_zero_v_panel.rs`

Adicionar seção "Limiter" com 3 controles:
- Toggle **Enable** (on/off)
- Knob **Ceiling** (-12 a 0 dB, label com valor absoluto)
- Knob **Release** (10 a 500 ms, label com valor absoluto)

### 5.7 `src/core/ui/main_view.rs`

Se necessário, passar os closures do limiter para o orquestrador de painéis.

---

## 6. Plano de Testes

### Testes unitários (`src/core/dsp/limiter.rs`)

```rust
#[test]
fn test_limiter_passes_quiet_signal() { ... }     // sinal abaixo do ceiling = sem alteração
#[test]
fn test_limiter_attenuates_loud_signal() { ... }   // sinal acima do ceiling = atenuado ao ceiling
#[test]
fn test_limiter_instant_attack() { ... }           // primeiro sample acima do ceiling é atenuado
#[test]
fn test_limiter_release_envelope() { ... }         // envelope decai após pico
#[test]
fn test_limiter_ceiling_zero_db() { ... }          // ceiling 0 dB = sem alteração em sinal ≤ 1.0
#[test]
fn test_limiter_ceiling_minus_twelve_db() { ... }  // ceiling -12 dB = atenua forte
#[test]
fn test_limiter_reset() { ... }                    // reset limpa envelope
```

### Testes de integração

- `test_limiter_prevents_clipping_with_max_gain` — master +30 dB, ceiling -1 dB, verifica que output ≤ ceiling
- `test_limiter_bypass_passes_through` — enable=false, sinal passa sem alteração

---

## 7. Verificação de Garantia Anti-Clipping

Para **provar** matematicamente que o limiter previne clipping:

### Cenário de pior caso

```
MLC gain = 0 dB (ganho 1.0)
MLC master = 0 dB (ganho 1.0)
Cabinet IR = ressonância máxima (pior caso: ~1.0)
Master Gain = +30 dB (ganho ~31.62)
→ Sinal máximo teórico = 1.0 × 1.0 × 1.0 × 31.62 = 31.62
```

### Com limiter ativo (ceiling = -1.0 dB)

```
ceiling_linear = 10^(-1.0/20) = 0.8913
Para qualquer sample com |sample| > 0.8913:
    sample *= 0.8913 / |sample|  → |sample| = 0.8913
→ Saída máxima = 0.8913 < 1.0  ✓ NUNCA CLIPA
```

### Com limiter ativo (ceiling = -0.3 dB, pior caso aceitável)

```
ceiling_linear = 10^(-0.3/20) = 0.9661
→ Saída máxima = 0.9661 < 1.0  ✓ NUNCA CLIPA
```

**Garantia:** Enquanto `ceiling_linear < 1.0` (ceiling < 0 dB), a saída **nunca** excede o ceiling, portanto **nunca** atinge ±1.0 (clipping digital).

---

## 8. Ordem de Implementação

| Tarefa | Arquivo(s) | Prioridade |
|---|---|---|
| T1 | `src/core/dsp/limiter.rs` (novo) | 🔴 Crítico |
| T2 | `src/core/dsp/mod.rs` (+1 linha) | 🔴 Crítico |
| T3 | `src/core/state/plugin_params.rs` (+3 params) | 🔴 Crítico |
| T4 | `src/lib.rs` (struct + init + process) | 🔴 Crítico |
| T5 | `src/bin/standalone.rs` (struct + init + audio) | 🟡 Alto |
| T6 | `src/core/ui/mlc_zero_v_panel.rs` (UI) | 🟡 Alto |
| T7 | Testes unitários `src/core/dsp/limiter.rs` | 🔴 Crítico |
| T8 | Teste de integração `tests/lab_integration.rs` | 🟡 Alto |
| T9 | `cargo check` + `cargo test` — 0 erros | 🔴 Crítico |

---

## 9. UI Design

### Painel MLC ZERO V — Seção Limiter

```
┌─ LIMITER ────────────────────────────────────┐
│ [✓] Enable          ┌──────┐  ┌──────┐      │
│                     │ Ceil │  │  Rel │      │
│                     │ -1.0 │  │50 ms │      │
│                     │  dB  │  │      │      │
│                     └──────┘  └──────┘      │
└──────────────────────────────────────────────┘
```

- **Enable**: toggle switch no topo da seção
- **Ceiling**: knob com display de valor (`{:.1} dB`)
- **Release**: knob com display de valor (`{:.0} ms`)
- Layout horizontal (knobs lado a lado), seguindo o padrão existente no painel MLC

---

## 10. Riscos e Mitigações

| Risco | Mitigação |
|---|---|
| Release muito curto pode causar distorção harmônica audível | Default 50ms é conservador. Usuário ajusta a gosto |
| Ceiling = 0 dB pode não proteger contra ISPs em DACs | Default -1.0 dB. Documentar que -0.3 é o mínimo seguro |
| Limiter pode "achatar" dinâmica do amp | Só atua quando sinal excede ceiling. Com gain razoável, é transparente |
| Performance (branch por sample) | Branch trivial, bem menor que o custo do FFTConvolver. Zero alocações |
