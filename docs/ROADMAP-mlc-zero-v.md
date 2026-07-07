# MLC Zero-V — Roadmap de Melhorias

> Checklist mestre de todas as melhorias pesquisadas para o MLC Zero-V.
> Status: ✅ feito | ⬜ pendente | 🔬 em pesquisa | ❌ descartado

**Última atualização:** 2026-07-07

---

## Tier 1 — Imediato (baixo custo, alto impacto)

| # | Melhoria | Status | Linhas | Descrição |
|---|---|---|---|---|
| 1.1 | **Oversampling 4×** | ⬜ (→Tier 2) | ~20 | Faust 2.85.1 não suporta `declare options "-os 4"` — precisa de plugin-level ou DSP manual |
| 1.2 | **Tight (HPF entre stages)** | ✅ | 3 | Highpass 80Hz entre stage1→stage2 — remove "lama" |
| 1.3 | **Asymmetry slider** | ✅ | 5 | Controle 0–1 do bias no `clip_atanh` — par/ímpar variável |
| 1.4 | **Pre-Shape EQ** | ✅ | 8 | Low shelf 150Hz + peak 1kHz antes do `gain_stages` |
| 1.5 | **Multi-clipping** | ✅ | ~600 | 11 curvas → 2 (Asymmetric Tanh + Exponential) |
| 1.6 | **Limiter default -0.2 dB** | ✅ | 1 | Ajuste de ceiling padrão |

---

## Tier 2 — Curto prazo (médio custo, alto impacto)

| # | Melhoria | Status | Linhas | Descrição |
|---|---|---|---|---|
| 2.0 | **Oversampling 4×** | ⬜ | ~20 | Plugin-level (nih_plug) ou DSP manual (`ba.oversampling`) — Faust 2.85.1 nativo não suporta |
| 2.1 | **Clean Blend** | ⬜ | 15 | Split input → caminho limpo filtrado + saturado → mix 0–25% |
| 2.2 | **Tone stack real** | ⬜ | 10 | Substituir tone stack genérico por `tonestacks.lib` (24 modelos) |
| 2.3 | **Sag de power amp** | ⬜ | 20 | Envelope follower → redução de headroom no ataque |
| 2.4 | **Bright cap dinâmico** | ⬜ | 5 | `bright_gain` diminui automaticamente com `gain` alto |
| 2.5 | **Clip type por estágio** | ⬜ | 15 | Stage1, stage2, stage3 com clipagens diferentes e independentes |
| 2.6 | **HPF/LPF entre todos os estágios** | ⬜ | 10 | Filtros de acoplamento como em amps reais (evita "flub" e "fizz") |

---

## Tier 3 — Médio prazo (médio/alto custo, médio impacto)

| # | Melhoria | Status | Linhas | Descrição |
|---|---|---|---|---|
| 3.1 | **Pós-EQ paramétrico** | ⬜ | 20 | Mid sweep + fizz control (LPF 5–9kHz) pós-clipping |
| 3.2 | **Waveshaping por LUT** | ⬜ | 15 | Usar `tubes.lib` (12AX7, 6V6 reais) via `rdtable` |
| 3.3 | **Chebyshev harmonics** | ⬜ | 10 | Controle preciso de H2, H3, H4 individuais |
| 3.4 | **Power amp NFB loop real** | ⬜ | 25 | Feedback negativo com `~` do Faust (presence/resonance autêntico) |
| 3.5 | **Multi-band clipping** | ⬜ | 30 | Split 3-band → clip diferente por banda → mix |
| 3.6 | **ADAA anti-aliasing** | ⬜ | 20 | Alternativa a oversampling para hard clipping |

---

## Tier 4 — Longo prazo / Exploração

| # | Melhoria | Status | Descrição |
|---|---|---|---|
| 4.1 | **IR dinâmica (Dynamic Convolution)** | ⬜ | Resposta ao impulso que muda com o nível do sinal |
| 4.2 | **State-space modeling de válvula** | ⬜ | Modelo físico completo (não só função de transferência) |
| 4.3 | **Transformer saturation** | ⬜ | Não-linearidade magnética do transformador de saída |
| 4.4 | **Diode ladder model** | ⬜ | Modelagem física de diodos (Shockley equation) |
| 4.5 | **Dual amp / stereo routing** | ⬜ | Dois caminhos de amp independentes com pan |
| 4.6 | **Capture de hardware real** | ⬜ | LUT a partir de gravações de amplificadores/pedais físicos |

---

## Parâmetros Novos na UI (Tier 1 + 2)

| Parâmetro | Tipo | Range | Default | Grupo |
|---|---|---|---|---|
| `tight_enable` | Bool | on/off | on | Tight |
| `asymmetry_enable` | Bool | on/off | on | Harmonics |
| `asymmetry` | Float | 0.0–1.0 | 0.5 | Harmonics |
| `preshape_enable` | Bool | on/off | off | Pre-Shape |
| `preshape_tight` | Float | 0 a -6 dB | -3 dB | Pre-Shape |
| `preshape_bite` | Float | 0 a +6 dB | +3 dB | Pre-Shape |
| `clean_blend` | Float | 0–0.25 | 0.0 | Blend |
| `sag_amount` | Float | 0.0–1.0 | 0.0 | Power Amp |
| `bright_dynamic` | Bool | on/off | off | Bright |
| `clip_stage1` | Enum | Asymmetric/Exponential | Asymmetric | Stage Clip |
| `clip_stage2` | Enum | Asymmetric/Exponential | Asymmetric | Stage Clip |
| `clip_stage3` | Enum | Asymmetric/Exponential | Asymmetric | Stage Clip |
| `tonestack_model` | Enum | JCM800/Bassman/... | (atual) | Tone Stack |

---

## Histórico de Implementações

| Data | O que | Commits |
|---|---|---|
| 2026-07-07 | 11 curvas de clipagem → selecionáveis | `8c0b070` |
| 2026-07-07 | Refinamento: 2 curvas + limiter -0.2 + docs | `049e36b`, `22a79d5` |
| 2026-07-07 | Tier 1: Tight, Harmonics (asymmetry), Pre-Shape (3/4) | `ccb830c`..`9bcf231` (6 commits) — oversampling movido p/ Tier 2 |
| 2026-07-07 | Pesquisa: topologias, pedais, DSP avançado | `docs/REF-clipping-curves.md`, `docs/dsp-advanced-techniques-report.md` |

---

## Referências

- **DSP principal:** `dsp/mlc_zero_v.dsp`
- **Parâmetros:** `src/core/state/plugin_params.rs`
- **Bridge:** `src/bridge/mlc_zero_v.rs`
- **UI Plugin:** `src/core/ui/mlc_zero_v_panel.rs`
- **UI Standalone:** `src/bin/standalone.rs`
- **Docs:** `docs/REF-clipping-curves.md`, `docs/dsp-advanced-techniques-report.md`
