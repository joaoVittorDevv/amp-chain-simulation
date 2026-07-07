# Catálogo de Curvas de Clipagem / Saturação

> **Referência para desenvolvimento de estágios de distorção em Faust DSP.**  
> Curadoria originada da pesquisa para o MLC Zero-V, mantida como guia de implementação
> para futuros amplificadores e pedais no projeto.

---

## Índice

1. [Soft Clipping — Substituição direta do Tanh](#1-soft-clipping--substituição-direta-do-tanh)
2. [Hard Clipping](#2-hard-clipping)
3. [Assimetria](#3-assimetria)
4. [Wave Folding](#4-wave-folding)
5. [Avançado / Exótico](#5-avançado--exótico)
6. [Guia de Implementação no MLC Zero-V](#6-guia-de-implementação-no-mlc-zero-v)
7. [Checklist para Adicionar uma Nova Curva](#7-checklist-para-adicionar-uma-nova-curva)

---

## 1. Soft Clipping — Substituição direta do Tanh

Curvas que substituem `ma.tanh` 1:1 nos gain stages. Mudança de 1 linha.  
Sem oversampling necessário (o soft clipping naturalmente limita harmônicos).

### 1.1 Tanh (referência)

```
clip(x) = ma.tanh(x)
```

| Característica | Valor |
|---|---|
| **Som** | Compressão suave e cremosa. Referência clássica de amplificador valvulado. |
| **Harmônicos** | Ímpares (3º, 5º, 7º...) — som "quente" |
| **Custo** | Médio (função transcendental) |
| **Uso** | Linha de base, funciona em qualquer gain stage |

---

### 1.2 Algebraic Sigmoid ★★☆

```
clip(x) = x / sqrt(1.0 + x * x)
```

| Característica | Valor |
|---|---|
| **Som** | Cremoso nos médios, sem fizz nos agudos. Potencialmente a mais musical. |
| **Harmônicos** | Ímpares, compressão mais gradual que tanh |
| **Custo** | Baixo (1 sqrt + 1 div) |
| **Destaque** | Excelente relação custo × qualidade sonora |

---

### 1.3 ArcTan ★★☆

```
clip(x) = 2.0 / ma.PI * atan(ma.PI / 2.0 * x)
```

| Característica | Valor |
|---|---|
| **Som** | Saturação mais aberta que Tanh. Som limpo e articulado com menos compressão. |
| **Harmônicos** | Ímpares, joelho mais suave |
| **Custo** | Médio (atan transcendental) |
| **Uso** | Pré-amplificador, estágios iniciais de ganho |

---

### 1.4 Soft Sine ★☆☆

```
clip(x) = sin(max(0.0 - ma.PI / 2.0, min(ma.PI / 2.0, x)))
```

| Característica | Valor |
|---|---|
| **Som** | Extremamente suave. Som limpo com leve saturação "beira da distorção". |
| **Harmônicos** | Muito poucos — quase linear com soft knee |
| **Custo** | Alto (sin transcendental + clamp) |
| **Uso** | Boost limpo, edge-of-breakup |

---

### 1.5 Cubic ★☆☆

```
clip(x) = min(max(x - x * x * x / 3.0, -0.667), 0.667)
```

| Característica | Valor |
|---|---|
| **Som** | Boost limpo com saturação muito sutil. Ideal para pré-amplificador. |
| **Harmônicos** | Apenas 3º harmônico (polinomial) |
| **Custo** | Baixíssimo (polinomial puro) |
| **Uso** | Pré-amp clean, primeiro estágio de ganho |

---

## 2. Hard Clipping

⚠️ **Hard clipping SEMPRE precisa de oversampling** (4×–8×) para evitar aliasing severo.  
Sem oversampling, harmônicos acima de Nyquist dobram de volta como inarmônicos desagradáveis.

### 2.1 Hard Clip

```
clip(x) = min(max(x, -threshold), threshold)  // threshold típico: 0.5
```

| Característica | Valor |
|---|---|
| **Som** | Clip abrupto. Fuzz agressivo tipo transistor (Big Muff, RAT). |
| **Harmônicos** | Todos os ímpares, fortes |
| **Custo** | Baixíssimo (2 comparações) |
| **Requisito** | Oversampling 4×–8× |

---

### 2.2 Rational (germânio fuzz)

```
clip(x) = x / (1.0 + abs(x))
```

| Característica | Valor |
|---|---|
| **Som** | Agressivo e fuzzy. Caráter de transistor de germânio. |
| **Harmônicos** | Ímpares com saturação assimétrica |
| **Custo** | Baixo (1 abs + 1 add + 1 div) |
| **Nota** | Meio termo entre soft e hard clipping |

---

### 2.3 Exponential

```
sign_of(x) = ba.if(x > 0, 1.0, ba.if(x < 0, -1.0, 0.0))
clip(x) = (1.0 - exp(0.0 - abs(x))) * sign_of(x)
```

| Característica | Valor |
|---|---|
| **Som** | Muito agressivo. Timbre de pedal RAT/DS-1. |
| **Harmônicos** | Ricos em ímpares, transição abrupta |
| **Custo** | Alto (exp transcendental) |
| **Destaque** | ✅ **Selecionada para o MLC Zero-V** — melhor caráter high-gain |

---

## 3. Assimetria

Curvas com thresholds diferentes para positivo/negativo ou bias DC.  
Produzem harmônicos **pares** (2º, 4º, 6º...) — som mais "valvulado" e rico.

### 3.1 Asymmetric Tanh ★★★

```
clip(x) = ma.tanh(x + bias) - ma.tanh(bias)  // bias típico: 0.15–0.35
```

| Característica | Valor |
|---|---|
| **Som** | Harmônicos pares — som valvulado rico, "3D", tridimensional. |
| **Harmônicos** | Pares + ímpares (espectro completo) |
| **Custo** | Médio (2 tanh) |
| **Destaque** | ✅ **Selecionada para o MLC Zero-V** — padrão, melhor som geral |

---

### 3.2 Asymmetric Hard

```
clip(x) = min(max(x, -neg_threshold), pos_threshold)
// Exemplo: pos_threshold = 0.35, neg_threshold = 0.6
```

| Característica | Valor |
|---|---|
| **Som** | Hard clip assimétrico. Distorção agressiva com caráter valvulado. |
| **Harmônicos** | Pares + ímpares fortes |
| **Custo** | Baixíssimo |
| **Requisito** | Oversampling 4×–8× |

---

## 4. Wave Folding

Dobra a forma de onda sobre si mesma quando excede um threshold.  
Produz espectro complexo e metálico.

### 4.1 Wave Fold

```
sign_of(x) = ba.if(x > 0, 1.0, ba.if(x < 0, -1.0, 0.0))
clip(x) = x - 2.0 * max(0.0, abs(x) - threshold) * sign_of(x)
// threshold típico: 0.6
```

| Característica | Valor |
|---|---|
| **Som** | Metálico, cortante, timbre tipo "djent" / synth. |
| **Harmônicos** | Espectro complexo, muitos harmônicos agudos |
| **Custo** | Baixo (aritmética simples) |
| **Uso** | Estilos modernos, metal progressivo |
| **Requisito** | Oversampling 4×–8× recomendado |

---

## 5. Avançado / Exótico

Técnicas que vão além da substituição 1:1 de função.

### 5.1 Chebyshev Polynomials

Usa polinômios de Chebyshev para controle preciso do espectro harmônico.  
Permite adicionar exatamente o 2º, 3º, 4º harmônico em proporções controladas.

```
// T₂(x) = 2x² - 1   → 2º harmônico
// T₃(x) = 4x³ - 3x  → 3º harmônico
clip(x) = wet * (h2 * T2(x) + h3 * T3(x)) + dry * x
```

Prós: controle cirúrgico do timbre. Contras: complexidade alta, ganho de volume imprevisível.

---

### 5.2 Diode Ladder Model

Modelagem física de diodos (Shockley equation):

```
// i = I_s * (exp(v / (n*V_t)) - 1)
clip(x) = ...  // requer solver iterativo ou LUT
```

Prós: altíssimo realismo analógico. Contras: custo computacional extremo.

---

### 5.3 Lookup Table (LUT)

Pré-computa a curva de clipagem em uma tabela e interpola em tempo real.

```faust
clip_curve = waveform{...}  // tabela de 1024 pontos
clip(x) = clip_curve : rdtable  // com interpolação linear
```

Prós: permite qualquer formato de onda (capturado de hardware real).  
Contras: resolução fixa, memory bandwidth.

---

### 5.4 Multi-band Clipping

Divide o sinal em 2–3 bandas de frequência e aplica clipagem diferente em cada uma:

```
low_band  → soft clip (tanh)  → saturação quente nos graves
mid_band  → asymmetric clip    → caráter valvulado nos médios
high_band → hard clip          → brilho e corte nos agudos
```

Prós: controle tonal sem equalização pós-clipagem.  
Contras: latência dos crossovers, complexidade 3×.

---

## 6. Guia de Implementação no MLC Zero-V

O MLC Zero-V usa um pipeline de 3 estágios de ganho em cascata + 1 estágio WARCLAW,
todos compartilhando a mesma função de clipagem. Adicionar ou modificar curvas
requer tocar **4 camadas**:

### Camada 1 — DSP Faust (`dsp/mlc_zero_v.dsp`)

```faust
// 1. Declarar o parâmetro (range 0..N-1)
clip_type = nentry("Clip Type", 0, 0, N-1, 1)

// 2. Implementar a função de clipagem
clip_nova(x) = ...  // sua fórmula aqui

// 3. Adicionar ao seletor
clip(x) = ba.selectn(N, int(clip_type),
    clip_0(x),    // índice 0
    clip_1(x),    // índice 1
    clip_nova(x), // índice 2 — NOVO
    ...
)
```

**Regras Faust:**
- Use `ma.tanh`, `ma.sin`, `ma.atan`, `ma.abs`, `ma.sqrt` da standard library
- `-expressão` unário não funciona no Faust → use `0.0 - expressão`
- `signum()` pode não estar disponível → use helper com `ba.if`
- `int(clip_type)` garante que o float do parâmetro vire índice inteiro

### Camada 2 — Wrapper C

Após editar o `.dsp`, **regenerar o header**:

```bash
faust -lang cpp -cn mlczerov -I faust-ddsp -i dsp/mlc_zero_v.dsp -o dsp/MlcZeroVModule.hpp
```

O wrapper (`mlc_zero_v_wrapper.cpp`) usa `MLC_SET_PARAM("Clip Type", value)` — o label
`"Clip Type"` casa com o `nentry` do Faust, então **não precisa mexer no wrapper**
ao adicionar novas curvas (só o range do parâmetro muda automaticamente).

### Camada 3 — Bridge Rust (`src/bridge/mlc_zero_v.rs`)

```rust
// Ajustar o clamp no setter
pub fn set_clip_type(&mut self, value: f32) {
    self.clip_type = value.clamp(0.0, (N - 1) as f32).round();
}
```

### Camada 4 — Plugin Params + UI (`src/core/state/plugin_params.rs`)

```rust
// 1. Adicionar variante ao enum ClipType
#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ClipType {
    #[name = "Tanh"]
    Tanh,
    #[name = "Nova Curva"]
    NovaCurva,  // ← NOVA
}

// 2. Registrar no array ALL (DEVE seguir a mesma ordem do ba.selectn!)
pub const ALL: [ClipType; N] = [
    ClipType::Tanh,
    ClipType::NovaCurva,  // ← MESMA ordem do DSP
];

// 3. Mapear índice → f32 no as_f32()
pub fn as_f32(self) -> f32 {
    match self {
        ClipType::Tanh => 0.0,
        ClipType::NovaCurva => 1.0,  // ← NOVO
    }
}

// 4. Adicionar label() e description()
```

### Camada 5 — UI (plugin + standalone)

O ComboBox itera sobre `ClipType::ALL`, então **adicionar uma variante ao enum
automaticamente a expõe na UI** — sem código adicional.

```rust
// Plugin (mlc_zero_v_panel.rs)
for clip in ClipType::ALL {
    ui.selectable_value(&mut value, clip, clip.description());
}

// Standalone (standalone.rs)
for clip in ClipType::ALL {
    ui.selectable_value(&mut clip_type, clip, clip.description());
}
```

---

## 7. Checklist para Adicionar uma Nova Curva

- [ ] **DSP**: Adicionar `clip_nova(x)` no `.dsp` + entrada no `ba.selectn`
- [ ] **DSP**: Ajustar range do `nentry` (0..N-1)
- [ ] **Faust**: Regenerar `MlcZeroVModule.hpp` com `faust -lang cpp -cn mlczerov -I faust-ddsp -i ...`
- [ ] **Bridge**: Ajustar `clamp(0.0, (N-1) as f32)` no `set_clip_type`
- [ ] **Params**: Adicionar variante ao `ClipType` enum + `ALL` + `as_f32()` + `label()` + `description()`
- [ ] **Testes**: Atualizar assertions de quantização no `mlc_zero_v.rs` (valor máximo = N-1)
- [ ] **Build**: `cargo check` limpo
- [ ] **Testes**: `cargo test` — todos passando
- [ ] **Áudio**: Testar sonoramente com diferentes níveis de gain

---

## Referências

- **DSP principal**: `dsp/mlc_zero_v.dsp`
- **Parâmetros**: `src/core/state/plugin_params.rs` → `ClipType` enum
- **Bridge**: `src/bridge/mlc_zero_v.rs` → `MlcZeroVProcessor`
- **Build Faust**: `Makefile` → target `build-faust-mlc`

---

*Catálogo compilado em Julho/2026. Pesquisa original por sub-agentes pi + síntese polly.*
