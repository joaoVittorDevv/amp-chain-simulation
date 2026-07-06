# MLC ZERO V Signature Amp — Specification

**Feature ID:** `MLCZERO`
**Status:** Planning | **✅ Cross-vendor review COMPLETED (pi, 2026-07-06)**

## Problem Statement

O plugin atualmente possui apenas um tipo de amplificador: o Neural Drive baseado em Mojo (saturação tanh). Para produção de metal moderno, guitarristas precisam de um amplificador high-gain modelado com a resposta tonal completa de um amp real — incluindo tone stack, múltiplos estágios de ganho, presença, depth, e controles de voicing. O MLC ZERO V Signature do Vogg (Decapitated) é a referência escolhida por representar o timbre de metal moderno: tight, agressivo, com graves controlados e alta definição.

## Goals

- [ ] Adicionar um segundo modelo de amplificador (MLC ZERO V Signature) ao pipeline DSP
- [ ] Permitir alternância entre Neural Amp (existente) e MLC ZERO V via seletor
- [ ] Modelar o canal Drive II (high-gain) com todos os controles do amp real
- [ ] Saída line-level compatível com o Cabinet IR existente
- [ ] Manter zero alocações na thread de áudio

## Out of Scope

| Feature | Reason |
|---------|--------|
| Canal Clean | MVP focado em Drive II — high gain metal |
| Canal Drive I | MVP focado em Drive II |
| MIDI control mapping | Complexidade adicional; amp real tem MIDI mas plugin já é controlado pelo host |
| FX Loop simulation | O plugin se insere exatamente onde um FX loop estaria na cadeia |
| Multi-mic cabinet | Já existe cabinet IR com impulse responses do usuário |

---

## User Stories

### P1: Selecionar Modelo de Amplificador ⭐ MVP

**User Story**: Como guitarrista, quero alternar entre o Neural Amp e o MLC ZERO V Signature para comparar timbres e escolher o melhor para cada situação.

**Why P1**: É o ponto de entrada da feature — sem o seletor, o novo amp não é acessível.

**Acceptance Criteria**:

1. WHEN o usuário seleciona "MLC ZERO V" no seletor de amp THEN o sistema SHALL desativar o Neural Drive e ativar o processamento MLC ZERO V
2. WHEN o usuário seleciona "Neural" no seletor THEN o sistema SHALL restaurar o comportamento atual (Mojo tanh)
3. WHEN ocorre a transição entre modelos THEN o sistema SHALL aplicar crossfade de 10ms para evitar clicks/artefatos
4. WHEN o amp MLC ZERO V está ativo THEN os parâmetros do Neural Drive SHALL ser ocultados na UI

**Independent Test**: Abrir o plugin, alternar entre Neural e MLC ZERO V, verificar que o áudio processado muda e que a UI responde corretamente.

---

### P2: Controles de Ganho e Tone Stack ⭐ MVP

**User Story**: Como guitarrista, quero controlar o gain, bass, middle e treble do MLC ZERO V exatamente como no amplificador real para esculpir meu timbre.

**Why P2**: São os controles fundamentais de qualquer amplificador — sem eles o amp não é usável.

**Acceptance Criteria**:

1. WHEN o usuário ajusta o knob GAIN THEN o sistema SHALL controlar o drive do pré-amplificador modelado
2. WHEN o usuário ajusta BASS, MIDDLE ou TREBLE THEN o sistema SHALL aplicar a equalização correspondente no tone stack
3. WHEN qualquer knob é ajustado THEN o sistema SHALL responder em tempo real sem artefatos
4. WHEN os knobs estão no meio (12h) THEN o sistema SHALL produzir o timbre de referência do tone stack (curva FMV natural com seu mid-scoop característico — não uma resposta plana/linear)

**Independent Test**: Carregar o amp MLC ZERO V, tocar guitarra, girar cada knob e ouvir a mudança tonal correspondente.

---

### P3: Controles de Power Amp (Presence, Depth, Master) ⭐ MVP

**User Story**: Como guitarrista, quero controlar presence, depth e master volume do power amp para finalizar meu timbre como faria no amp real.

**Why P3**: Presence e Depth são essenciais para o timbre de metal moderno — definem o "tightness" e a projeção.

**Acceptance Criteria**:

1. WHEN o usuário ajusta PRESENCE THEN o sistema SHALL modificar a resposta de altas frequências no estágio de power amp
2. WHEN o usuário ajusta DEPTH THEN o sistema SHALL modificar a resposta de graves no estágio de power amp
3. WHEN o usuário ajusta MASTER THEN o sistema SHALL controlar o volume de saída do amp (pré-cabinet)
4. WHEN o MASTER está no mínimo THEN o sistema SHALL produzir silêncio (sem leak de sinal)

**Independent Test**: Com o amp ativo, ajustar Presence/Depth e verificar mudança no espectro (analisador FFT). Ajustar Master e verificar variação de volume.

---

### P4: Switches de Voicing (Bright, M45, WARCLAW, Feedback)

**User Story**: Como guitarrista, quero usar os switches de voicing do MLC ZERO V (Bright I/II, M45 on/off, WARCLAW, Feedback LO/HI) para acessar diferentes caráteres tonais do amplificador.

**Why P4**: Esses switches são o que tornam o MLC ZERO V único — o WARCLAW especialmente é parte do "signature sound".

**Acceptance Criteria**:

1. WHEN o usuário alterna BRIGHT entre I e II THEN o sistema SHALL modificar o capacitor de bright no pré-amplificador
2. WHEN o usuário ativa M45 THEN o sistema SHALL reduzir o ganho e alterar a resposta para o modo "Plexi/JCM"
3. WHEN o usuário ativa WARCLAW THEN o sistema SHALL aplicar saturação/boost adicional característico
4. WHEN o usuário seleciona FEEDBACK LO/HI THEN o sistema SHALL modificar o negative feedback do power amp

**Independent Test**: Alternar cada switch durante uma performance e verificar mudança audível no timbre.

---

### P5: Noise Gate Integrado

**User Story**: Como guitarrista de metal, quero um noise gate integrado ao amp MLC ZERO V para controlar o ruído de alto ganho sem precisar de plugins externos.

**Why P5**: O amp real tem gate integrado; para metal moderno com alto ganho, gate é indispensável.

**Acceptance Criteria**:

1. WHEN o sinal de entrada cai abaixo do threshold THEN o sistema SHALL silenciar a saída (gate fecha)
2. WHEN o sinal de entrada sobe acima do threshold THEN o sistema SHALL abrir o gate com attack suave (sem clicks)
3. WHEN o usuário ajusta o parâmetro GATE THEN o sistema SHALL modificar o threshold de abertura/fechamento

**Independent Test**: Parar de tocar com ganho alto — verificar que o ruído de fundo é silenciado. Voltar a tocar — verificar abertura sem artefatos.

---

### P6: UI Panel do MLC ZERO V

**User Story**: Como usuário, quero um painel visual no plugin que mostre todos os controles do MLC ZERO V organizados de forma intuitiva, similar ao layout do amplificador real.

**Why P6**: A UI é como o usuário interage com o amp — se não for clara e responsiva, a experiência é prejudicada.

**Acceptance Criteria**:

1. WHEN o amp MLC ZERO V está selecionado THEN o sistema SHALL exibir o painel com todos os knobs e switches
2. WHEN o amp Neural está selecionado THEN o sistema SHALL exibir o painel Neural existente
3. WHEN qualquer knob é ajustado na UI THEN o sistema SHALL atualizar o parâmetro correspondente em tempo real
4. WHEN o parâmetro é automatizado pelo host THEN a UI SHALL refletir o valor atual

**Independent Test**: Navegar entre os painéis, ajustar knobs com mouse, verificar resposta visual e sonora.

---

### P7: Paridade Standalone

**User Story**: Como usuário do modo standalone, quero ter os mesmos controles do MLC ZERO V disponíveis, com a mesma qualidade de áudio do plugin.

**Why P7**: O standalone é o ambiente de desenvolvimento e teste principal (`make run`). Sem paridade, o standalone fica quebrado.

**Acceptance Criteria**:

1. WHEN o MLC ZERO V está selecionado no standalone THEN o sistema SHALL aplicar o mesmo processamento DSP do plugin
2. WHEN qualquer parâmetro é ajustado no standalone THEN o sistema SHALL responder identicamente ao plugin
3. Todos os parâmetros do MLC ZERO V SHALL estar disponíveis na UI do standalone

**Independent Test**: Rodar `make run`, carregar configuração do MLC, verificar que todos os controles funcionam.

---

## Edge Cases

- WHEN o buffer de áudio tem tamanho zero THEN o sistema SHALL retornar imediatamente sem processar
- WHEN o sample rate muda (host) THEN o sistema SHALL reinicializar os coeficientes do Faust com o novo sample rate
- WHEN o usuário alterna rapidamente entre modelos THEN o sistema SHALL completar cada crossfade antes de iniciar o próximo
- WHEN o Faust falha ao inicializar THEN o sistema SHALL fazer fallback para bypass silencioso no canal MLC
- WHEN o parâmetro está fora do range (automação extrema) THEN o sistema SHALL clamp ao valor válido mais próximo
- WHEN o gate está configurado como "Pre" THEN o sistema SHALL aplicar o gate antes do pré-amplificador (input gate tradicional)
- WHEN o gate está configurado como "Post" THEN o sistema SHALL aplicar o gate após o tone stack (gate moderno)
- WHEN o seletor de gate position é alterado THEN o sistema SHALL aplicar a nova posição no próximo bloco de áudio
- WHEN o parâmetro `eq_tanh_bypass` está ativado THEN o sistema SHALL remover a saturação `ma.tanh` do estágio EQ Faust
- WHEN `eq_tanh_bypass` está desativado (default) THEN o sistema SHALL manter o comportamento atual do EQ com `ma.tanh`

---

## Requirement Traceability

| Requirement ID | Story | Phase | Status |
|---------------|-------|-------|--------|
| MLCZERO-01 | P1: Amp Selector | Design | Pending |
| MLCZERO-02 | P2: Gain + Tone Stack | Design | Pending |
| MLCZERO-03 | P3: Presence + Depth + Master | Design | Pending |
| MLCZERO-04 | P4: Voicing Switches | Design | Pending |
| MLCZERO-05 | P5: Noise Gate | Design | Pending |
| MLCZERO-06 | P6: UI Panel | Design | Pending |
| MLCZERO-07 | P7: Standalone Parity | Design | Pending |

**Coverage:** 7 total, 7 mapped to tasks, 0 unmapped ✅

---

## Success Criteria

- [ ] Usuário consegue alternar entre Neural e MLC ZERO V sem clicks ou silêncio
- [ ] Todos os controles do MLC ZERO V respondem em tempo real (< 5ms de latência de parâmetro)
- [ ] Timbre do MLC ZERO V Drive II é reconhecível como high-gain metal moderno
- [ ] Gate reduz ruído de forma transparente sem cortar notas sustentadas
- [ ] Zero alocações na thread de áudio (assert pelo nih_plug)
- [ ] Build passa limpo: `cargo build --release && cargo test`
- [ ] **⚠️ Cross-vendor review executado e aprovado antes de qualquer implementação**
