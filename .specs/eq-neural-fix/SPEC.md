# EQ + Neural Amp Fix — Specification

## Problem Statement

O equalizador paramétrico de 3 bandas (Faust) e o módulo Neural Amp (Drive) do plugin Distortion estão com comportamento incorreto:

1. **EQ**: O `ma.tanh` após o chain comprime peaks, o Q mínimo de 0.707 é demasiado estreito para ajustes de tom padrão da indústria, e o filtro SVF perde precisão perto de Nyquist.
2. **Neural Amp**: O arquivo ONNX WaveNet capturado (`wavenet_drive.onnx`) existe mas nunca é executado — o Mojo usa apenas uma aproximação polinomial de `tanh`. Além disso, a lógica de gain/drive entre o standalone e o plugin é inconsistente.

## Goals

- [ ] EQ paramétrico responde fielmente ao que usuário seta (sem compressão de peaks pelo tanh)
- [ ] Faixas de Q cobrindo o padrão da indústria (0.1–10.0)
- [ ] Modelo WaveNet ONNX executado via ONNX Runtime no Mojo
- [ ] Pipeline de drive consistente entre standalone e plugin
- [ ] Pre-EQ IR adequado (>= 44k samples para resposta de tone stack)

## Out of Scope

| Feature | Reason |
|---|---|
| Adicionar mais bandas ao EQ (4+, 5-band) | MVP focado na correção dos 3 problemas principais |
| Treinar novo modelo neural | Modelo existente já foi capturado |
| Trocar Faust por outra linguagem DSP | Faust é a base estável — apenas corrigir o uso |
| Reimplementar o wrapper C++ do Faust | O ParamMapUI funciona — só há bug de uso |

---

## User Stories

### P1: Remover ma.tanh do pipeline EQ ⭐ MVP

**User Story**: Como guitarist que ajusta tone, quero que cada knob de EQ responda linearmente (sem compressão), para que eu consiga a curva de frequência exata que defini.

**Why P1**: O `ma.tanh` no final do chain é a causa raiz de todos os problemas do EQ — knob girado para +6 dB não sobe +6 dB na prática.

**Acceptance Criteria**:

1. WHEN EQ Low Gain é setado para +6 dB THEN a resposta em frequência medida em 100 Hz SHALL aumentar em 6 dB (±0.5 dB)
2. WHEN EQ Mid Gain é setado para -3 dB THEN a resposta em frequência medida em 1 kHz SHALL cair em 3 dB (±0.5 dB)
3. WHEN EQ High Gain é setado para +4 dB THEN a resposta em frequência medida em 8 kHz SHALL aumentar em 4 dB (±0.5 dB)
4. WHEN todos os gains são 0 dB THEN a resposta em frequência SHALL ser flat (±0.1 dB de 20 Hz a 20 kHz)
5. WHEN bypass do EQ é ativado THEN o sinal passa sem processamento de EQ

**Independent Test**: Gerar sine sweep de 20 Hz–20 kHz, processar pelo EQ com gains conhecidos, medir resposta de frequência com FFT — verificar diferença contra curva teórica.

---

### P2: Alargar range de Q para padrão da indústria ⭐ MVP

**User Story**: Como engineer, quero Q mínimo de 0.1 para fazer ajustes globais de tom (broad tonal shaping), e Q máximo de 10.0 para surgical cuts.

**Why P2**: O Q mínimo atual de 0.707 é aproximadamente 2/3 de oitava — muito estreito para uso como shelving suave.

**Acceptance Criteria**:

1. WHEN Low Q é ajustado para 0.1 THEN a banda low shelf SHALL afetar uma região de ±1 oitava ao redor da frequência central
2. WHEN High Q é ajustado para 10.0 THEN a banda high shelf SHALL ter resposta muito estreita (Q-SEM ~= 0.5 oitava)
3. WHEN o knob Q está no mínimo (0.1) THEN não SHALL haver ringing ou ressonância audível em nenhuma frequência

**Independent Test**: Medir resposta de frequência com Q=0.1 e Q=10.0 — largura de -3 dB points deve ser consistente com teoria de filtros IIR.

---

### P3: Integrar inferência ONNX WaveNet no Mojo ⭐ MVP

**User Story**: Como guitarist, quero que o plugin reproduza fielmente a distorção pesada que capturei no NAM (meu amplificador real), não uma aproximação genérica de tanh.

**Why P3**: O `main.mojo` atual ignora completamente o `wavenet_drive.onnx`. O usuário tem um modelo treinado que não está sendo usado.

**Acceptance Criteria**:

1. WHEN audio entra no estágio neural THEN o modelo ONNX WaveNet SHALL ser executado (não a aproximação polinomial)
2. WHEN o modelo ONNX não pode ser carregado (arquivo ausente/corrompido) THEN SHALL fallback para aproximação tanh polinomial com log de warning
3. WHEN drive é variado (0–1.0) THEN o modelo neural SHALL responder de forma diferente de tanh puro (verificável por análise espectral)
4. WHEN buffer de áudio é processado THEN a inferência SHALL completar em tempo real (< buffer_size / sample_rate ms)

**Independent Test**: Processar mesma sine wave através do modelo — saída SHALL ser diferente da aproximação polinomial em pelo menos 10% dos samples (distorção harmônica não-linear perceptível).

---

### P4: Unificar lógica de drive entre standalone e plugin ⭐ MVP

**User Story**: Como usuário, quero que o knob de drive tenha comportamento idêntico no standalone e no plugin, para que meus presets funcionem igual nos dois.

**Why P4**: Standalone usa `neural_vol` (0–1.0) como gain duplo; plugin usa `db_to_gain(0–30 dB)` — ranges e curva completamente diferentes.

**Acceptance Criteria**:

1. WHEN neural_drive é setado em 0.5 no plugin AND no standalone THEN ambos SHALL produzir gain de entrada equivalente (±0.1 dB)
2. WHEN neural_output_gain (makeup) é variado THEN o ganho de saída SHALL variar de forma independente do drive
3. WHEN usuário abre preset salvo no plugin THEN os parâmetros de drive SHALL ser restaurados corretamente

**Independent Test**: Comparar saída de ambos com mesmos parâmetros — diferença RMS < -60 dB.

---

### P5: Substituir Pre-EQ IR curta por filtro paramétrico Faust ⭐ P2

**User Story**: Como guitarist, quero que o pre-EQ reproduza curvas de tone stack reais (Marshall, Fender, Vox), não apenas 256 samples de resposta arbitrária.

**Why P5**: O `pre_eq_ir.wav` de 5.8 ms é curto demais para representar curvas de tone stack (que precisam de 50–500 ms de resposta de frequência na banda baixa).

**Acceptance Criteria**:

1. WHEN pre_EQ está ativo THEN a resposta de frequência medida SHALL ser equivalente a um filtro paramétrico com frequência e Q configuráveis
2. WHEN pre_EQ IR não pode ser carregado THEN o processamento neural SHALL continuar sem pre_EQ (não é blocker)
3. WHEN pre_EQ bypass é ativado THEN o sinal passa direto para o estágio neural

**Independent Test**: Comparar resposta de frequência com e sem pre_EQ — diferença mensurável em espectro.

---

## Edge Cases

- WHEN sample rate é 48000 Hz THEN o EQ Faust SHALL manter estabilidade numérica (sem overflow em filtros IIR perto de Nyquist)
- WHEN drive está em 0 (bypass de fato) THEN o módulo neural SHALL passar o sinal sem processamento
- WHEN o modelo ONNX tem formato incompatível THEN SHALL fallback para tanh com warning no stderr
- WHEN buffer size é pequeno (< 128 samples) THEN a inferência neural SHALL completar sem xruns

---

## Requirement Traceability

| Req ID | Story | Phase | Status |
|---|---|---|---|
| EQ-01 | P1: Remover ma.tanh | Spec | Pending |
| EQ-02 | P1: EQ responde linearmente | Spec | Pending |
| EQ-03 | P1: EQ flat quando 0dB | Spec | Pending |
| EQ-04 | P1: EQ bypass funcional | Spec | Pending |
| Q-01 | P2: Q range 0.1–10.0 | Spec | Pending |
| Q-02 | P2: Q=0.1 sem ringing | Spec | Pending |
| NN-01 | P3: ONNX executado por Mojo | Spec | Pending |
| NN-02 | P3: Fallback tanh se ONNX falha | Spec | Pending |
| NN-03 | P3: Drive neural diferente de tanh | Spec | Pending |
| NN-04 | P3: Real-time performance | Spec | Pending |
| DR-01 | P4: Drive consistente plugin/standalone | Spec | Pending |
| DR-02 | P4: Makeup gain independente | Spec | Pending |
| DR-03 | P4: Presets restauram parâmetros | Spec | Pending |
| PE-01 | P5: Pre-EQ paramétrico | Spec | Pending |
| PE-02 | P5: Pre-EQ graceful degradation | Spec | Pending |

**Coverage**: 16 total, 0 mapped to tasks, 0 unmapped ⚠️

---

## Success Criteria

- [ ] Gerar sine sweep de 20 Hz–20 kHz, processar pelo EQ, verificar curva de resposta contra teórica (MAX 0.5 dB erro)
- [ ] Verificar que saída neural é diferente de aproximação tanh simples (não é mais genérico)
- [ ] Plugin e standalone produzem mesma saída com mesmos parâmetros (RMS diff < -60 dB)
- [ ] `make build` completa sem errors
- [ ] `make run` inicia sem crashes
- [ ] Nenhum warning de ONNX Runtime ao carregar modelo válido
