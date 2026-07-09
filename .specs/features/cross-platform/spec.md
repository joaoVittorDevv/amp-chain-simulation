# Cross-Platform Support (Linux → Windows + macOS) Specification

**Feature ID:** `CROSS`
**Status:** Draft
**Source:** `docs/CROSS_PLATFORM_ROADMAP.md` (19 itens: B1–B8, L1–L9, M1–M2 — o índice do roadmap diz "17", mas a sua própria tabela tem 19 linhas)
**Revisão:** codex, ronda 1 — 8 lacunas, 6 aceites, 1 aceite parcialmente, 1 rejeitada; 4 debates fechados (D1–D4)

---

## Problem Statement

O plugin compila e corre exclusivamente em Linux: `build.rs` invoca `which`, lê `$HOME`, exige o
compilador Mojo (que não existe para Windows), emite `neural/libneural.so` com extensão hardcoded, e
liga `-lneural` incondicionalmente. Ao mesmo tempo, dois bugs de áudio ativos (drenagem I16 a meia
velocidade, truncamento silencioso de frames) degradam o som em **todas** as plataformas, incluindo
Linux. A janela para agir é agora: o pipeline DSP acaba de estabilizar (tone stack, tube waveshaping,
ADAA) e cada semana de novo código inline em `src/bin/standalone.rs` aumenta o custo da refatoração
que a portabilidade exige.

---

## Goals

- [ ] `cargo build --release` conclui com sucesso em Linux, Windows (MSVC x64) e macOS (arm64 + x86_64)
- [ ] Um único comando (`cargo xtask <verbo>`) cobre `check-env`, `pre-build`, `build`, `run`, `bundle`, `clean` nas 3 plataformas
- [ ] O backend neural produz saída numericamente equivalente (|Δ| ≤ 1e-6) entre Mojo e Rust puro
- [ ] Zero regressão de performance: FFI zero-copy, zero alocações e zero locks na thread de áudio, latência inalterada
- [ ] Streams de entrada aceitam `F32`, `I32` e `I16` sem descartar frames
- [ ] CI valida cada commit nas 3 plataformas
- [ ] Nenhum sample é silenciosamente descartado nem lido a taxa incorreta em nenhum `SampleFormat`
- [ ] Bundles de release (`.vst3`/`.clap`) não dependem de nenhuma biblioteca nativa fora do próprio binário (D4)
- [ ] Nenhum drop de áudio (input ou output) ocorre sem ser contabilizado num contador observável (D1)
- [ ] O drift de clock entre dispositivos físicos distintos é compensado, não apenas o desvio de sample rate nominal (D3)

---

## Out of Scope

| Feature | Reason |
|---|---|
| Suporte a Windows 32-bit / i686 | Nenhum host DAW moderno o exige; `f_size_t` = `size_t` resolve LLP64 sem custo extra |
| Suporte a Linux ARM (aarch64) | Sem hardware de teste; nada no design o impede posteriormente |
| Compilação Mojo no Windows | O SDK não existe; coberto pelo fallback Rust (CROSS-03) |
| Distribuir o backend Mojo dentro de bundles VST3/CLAP | **D4**: exigiria `@loader_path` fixups, codesigning da lib aninhada e o runtime do Mojo (`libKGENCompilerRTShared`, …) por zero ganho audível — os dois backends são equivalentes a 1e-6 (CROSS-04). Mojo é toolchain de desenvolvimento, não de distribuição (CROSS-24) |
| Exigir Mojo para releases macOS | **D2**: mesma razão; o fallback Rust é o backend de release em todas as plataformas |
| Contribuir `ASIOSTInt24LSB` para o CPAL upstream | Fora do controlo do projeto; CROSS-16 apenas deixa de esconder o dispositivo |
| Assinatura/notarização de bundles macOS | Distribuição, não portabilidade; feature separada |
| CoreAudio aggregate devices como caminho suportado | Requer verificação manual em hardware; coberto por UAT (CROSS-23), não por CI |
| Instaladores (`.msi`, `.pkg`, AppImage) | Distribuição, não portabilidade |
| Cross-compilation a partir de Linux | CI nativa multi-OS é mais barata e mais fiável (CROSS-17) |
| Reescrita do pipeline DSP | Preservação bit-a-bit é requisito, não objetivo de mudança |

---

## User Stories

### P1: Compilar e correr nas três plataformas ⭐ MVP

**User Story**: As a developer trabalhando em Linux, I want que o projeto compile e arranque em
Windows e macOS sem alterações manuais so that eu possa distribuir o plugin sem manter três árvores
de código.

**Why P1**: Nada mais no roadmap é observável enquanto o build falhar. `build.rs` aborta em
`pre_build_check()` antes sequer de chegar ao código de áudio, e o `f_size_t` de 32 bits corrompe
memória em MSVC de forma silenciosa. Esta história é a fundação literal das outras duas.

**Acceptance Criteria**:
1. WHEN `cargo build --release` corre num host sem o compilador Mojo instalado THEN o system SHALL concluir o build usando o backend neural em Rust puro, sem panic
2. WHEN `build.rs` procura os compiladores Faust e Mojo em Windows THEN o system SHALL usar `where.exe` e `%USERPROFILE%`, e em Unix `which` e `$HOME`
3. WHEN o backend Mojo é compilado THEN o system SHALL emitir `neural/libneural.so` em Linux, `neural/libneural.dylib` em macOS e `neural/neural.dll` em Windows
4. WHEN o backend Mojo está ausente THEN o system SHALL omitir toda a diretiva `cargo:rustc-link-lib=neural` e todo o bloco `extern "C"` correspondente
5. WHEN `bindgen` processa `dsp/wrapper.h` ou `dsp/mlc_zero_v_wrapper.h` sob MSVC x64 THEN o system SHALL gerar `usize` para `f_size_t` (8 bytes), idêntico a Linux e macOS
6. WHEN o mesmo bloco de amostras é processado por `MojoProcessor` e por `RustNeuralProcessor` THEN o system SHALL produzir saídas cuja diferença absoluta máxima é ≤ 1e-6
7. WHEN um `lab.db` criado em Linux (com nós `mojo-neural`) é aberto em Windows THEN o system SHALL resolver o nó para o backend disponível sem erro de variante desconhecida
8. WHEN o linker de macOS liga a `libneural.dylib` THEN o system SHALL receber a flag `-rpath` apontando para `neural/`
9. WHEN um developer corre `cargo xtask build`, `run`, `bundle`, `clean` ou `check-env` em qualquer uma das 3 plataformas THEN o system SHALL executar a mesma semântica sem dependência de `bash`, `make` ou `command -v`
10. WHEN `cargo xtask bundle` é invocado THEN o system SHALL forçar o backend Rust (`have_mojo` desativado), produzindo bundles sem dependências nativas externas
11. WHEN `cargo test` corre num build com `have_mojo` ativo THEN o system SHALL localizar `libneural.so`/`.dylib` via `-rpath` embutido, sem exigir `LD_LIBRARY_PATH` no ambiente do utilizador
12. WHEN o Faust está ausente e o mtime de um `.dsp` é mais recente que o do `.hpp` correspondente THEN o system SHALL abortar o build com erro, em vez de compilar silenciosamente um header obsoleto

**Independent Test**: Num container Windows Server e num runner macOS, sem Mojo instalado, correr
`cargo xtask check-env && cargo build --release && cargo test`. Em Linux, correr o teste de
equivalência `neural_backends_are_numerically_equivalent` com o Mojo presente.

---

### P2: Áudio correto em qualquer driver

**User Story**: As a guitarrista usando uma interface ASIO ou WASAPI, I want que o standalone aceite o
formato nativo do meu driver e reproduza cada sample exatamente uma vez so that eu ouça o meu tom e
não uma oitava abaixo com frames em falta.

**Why P2**: Depende de P1 (nada disto se pode testar sem build). Contém dois bugs que já degradam
Linux hoje (CROSS-08, CROSS-09) e o gate `F32`-only que torna o ASIO inutilizável — o formato nativo
de Focusrite, RME e Behringer é `I32`.

**Acceptance Criteria**:
1. WHEN o driver de saída negocia `SampleFormat::I16` THEN o system SHALL retirar do ring buffer duas amostras por frame (L e R), correspondendo às duas que o produtor insere
2. WHEN o callback de entrada recebe um bloco de `N` frames maior que a capacidade dos buffers internos THEN o system SHALL processar todos os `N` frames em blocos sucessivos, sem descartar nenhum e sem alocar na thread de áudio
3. WHEN o driver de entrada reporta `SampleFormat::I32` ou `I16` THEN o system SHALL converter para `f32` normalizado em `[-1.0, 1.0]` na fronteira do driver e alimentar o pipeline DSP inalterado
4. WHEN o host selecionado é ASIO THEN o system SHALL construir os streams de entrada e saída a partir de um único `cpal::Device`
5. WHEN um dispositivo ASIO expõe apenas `ASIOSTInt24LSB` THEN o system SHALL listá-lo na UI marcado como "formato não suportado" em vez de o omitir silenciosamente
6. WHEN a feature `asio` não está ativa THEN o system SHALL compilar em Windows contra WASAPI sem exigir o ASIO SDK nem `ASIO_DIR`
7. WHEN qualquer braço de `SampleFormat` processa um bloco THEN o system SHALL executar exatamente o mesmo código de pipeline DSP (uma única definição, sem duplicação)

**Independent Test**: Teste de equivalência null: alimentar o pipeline extraído com um bloco conhecido
via `F32`, `I32` e `I16` e verificar que as três saídas coincidem dentro do erro de quantização do
formato. Para o drain I16, um teste de contagem: `producer.push` de `2N` amostras, `N` frames de
output, buffer vazio no fim.

---

### P3: Ciclo de vida de dispositivos robusto e regressões apanhadas por CI

**User Story**: As a utilizador do standalone, I want ver imediatamente na UI quando a minha interface
é desligada ou quando input e output discordam de sample rate so that eu não passe 20 minutos a
depurar silêncio.

**Why P3**: Qualidade de experiência, não correção. Nada aqui bloqueia distribuição — o áudio já está
correto após P2. A CI (CROSS-17) vive aqui porque só tem algo a validar depois de P1 e P2 existirem.

**Acceptance Criteria**:
1. WHEN o `error_callback` de um stream dispara THEN o system SHALL sinalizar o erro à UI através de um canal lock-free e sem alocação, e a UI SHALL exibir um banner
2. WHEN dois erros distintos ocorrem antes de a UI ler o estado THEN o system SHALL preservar o **primeiro** erro (causa raiz) e contabilizar os subsequentes num contador, sem os perder silenciosamente
3. WHEN a UI lê o estado de erro THEN o system SHALL fazê-lo com uma operação atómica única (`swap`), tornando impossível perder um erro escrito entre a leitura e a limpeza
4. WHEN o utilizador clica "Refresh Devices" THEN o system SHALL reenumerar dispositivos e atualizar ambos os ComboBox
5. WHEN dois dispositivos reportam o mesmo nome THEN o system SHALL distingui-los por identidade estável (índice de enumeração + nome), e não por comparação de `String`
6. WHEN um dispositivo é selecionado THEN o system SHALL escolher a configuração a partir de `supported_input_configs()` / `supported_output_configs()`, e não apenas de `default_*_config()`
7. WHEN input e output não partilham nenhuma sample rate suportada THEN o system SHALL adotar a sample rate do output como master e ativar resampling em tempo real no caminho de entrada
8. WHEN o resampler está ativo THEN o system SHALL usar buffers pré-alocados, sem alocar na thread de áudio
9. WHEN input e output são dispositivos físicos distintos com clocks independentes THEN o system SHALL ajustar continuamente o rácio de resampling em função da ocupação do ring buffer, impedindo drift acumulado
10. WHEN o ring buffer transborda ou esvazia THEN o system SHALL incrementar um contador atómico dedicado (`overflows` / `underruns`) e expô-lo na UI
11. WHEN o utilizador consulta a latência física THEN o system SHALL exibir o buffer real do driver em samples e em milissegundos, derivado do tamanho observado do callback
12. WHEN o plugin é inicializado num host DAW THEN o system SHALL reportar a latência real do pipeline via `set_latency_samples`, e SHALL invocá-lo apenas quando o valor muda
13. WHEN um commit é enviado para o repositório THEN o system SHALL correr build e testes em `ubuntu-latest`, `windows-latest` e `macos-latest`

**Independent Test**: Desligar fisicamente a interface durante a reprodução e confirmar o banner de
erro. Forçar input a 44.1 kHz e output a 48 kHz e confirmar que o resampler ativa e a saída não faz
drift ao fim de 60 s. Abrir um PR e confirmar 3 jobs verdes.

---

## Edge Cases

- WHEN `build.rs` corre num host onde Faust está ausente mas `dsp/FaustModule.hpp` e `dsp/MlcZeroVModule.hpp` já existem, **e nenhum `.dsp` é mais recente que o `.hpp` correspondente** THEN o system SHALL usar os headers existentes e emitir um warning, em vez de fazer panic
- WHEN Faust está ausente e algum `.dsp` é mais recente que o seu `.hpp` THEN o system SHALL fazer panic — um header obsoleto compilaria DSP errado sem qualquer sinal
- WHEN Faust está ausente e os headers gerados **não** existem THEN o system SHALL fazer panic com uma mensagem que nomeia a plataforma e o comando de instalação
- WHEN Mojo está presente mas a compilação de `neural/main.mojo` falha THEN o system SHALL fazer panic em vez de cair silenciosamente para o fallback Rust (falha de build ≠ ausência de toolchain)
- WHEN o callback de áudio entrega um bloco de 0 frames THEN o system SHALL retornar sem tocar nos buffers
- WHEN o ring buffer de playthrough está vazio no callback de saída (underrun) THEN o system SHALL emitir silêncio e incrementar um contador atómico de underruns, sem bloquear
- WHEN o ring buffer de playthrough está cheio no callback de entrada (overflow) THEN o system SHALL descartar as amostras excedentes **e** incrementar um contador atómico de overflows — o descarte é permitido, o descarte silencioso não (D1)
- WHEN o contador de overflows cresce de forma sustentada com o resampler ativo THEN o controlador de drift SHALL reduzir o rácio de resampling até a ocupação do ring buffer estabilizar
- WHEN o controlador de drift atinge o limite de `max_resample_ratio_relative` THEN o system SHALL saturar no limite e sinalizar `ClockDriftUnrecoverable` à UI, em vez de divergir
- WHEN dois dispositivos partilham exatamente o mesmo nome (ex.: duas interfaces idênticas em Windows) THEN o system SHALL selecionar aquele cujo índice de enumeração coincide com o guardado, e não o primeiro que corresponda ao nome
- WHEN um nome de dispositivo contém caracteres não-ASCII THEN o system SHALL preservá-lo e exibi-lo intacto (`cpal::Device::name()` devolve `String`, que é UTF-8 por construção)
- WHEN `data.len()` não é múltiplo de `channels` THEN o system SHALL processar apenas os frames completos
- WHEN a conversão `I32 → f32` recebe `i32::MIN` THEN o system SHALL produzir um valor em `[-1.0, -1.0 - 1/2^31]` sem overflow (dividir por `i32::MAX as f64`, não por `abs(i32::MIN)`)
- WHEN o host ASIO é selecionado mas nenhum driver ASIO está instalado THEN o system SHALL exibir a lista vazia com uma nota explicativa, sem panic
- WHEN `lab.db` referencia o impl id `mojo-neural` numa plataforma sem Mojo THEN o system SHALL instanciar `RustNeuralProcessor` sob o mesmo id, preservando os valores de parâmetros do snapshot
- WHEN `set_latency_samples` é chamado a cada bloco com o mesmo valor THEN o system SHALL suprimir a notificação repetida ao host
- WHEN dois `cargo xtask` correm em paralelo (ex.: CI) THEN o system SHALL tolerar a corrida sobre `dsp/*.hpp` porque a regeneração é idempotente por mtime

---

## Requirement Traceability

| ID | Story | Roadmap | Phase | Status |
|---|---|---|---|---|
| CROSS-01 | P1: Compilar nas 3 plataformas | B1 | Design | Pending |
| CROSS-02 | P1: Compilar nas 3 plataformas | B3 | Design | Pending |
| CROSS-03 | P1: Compilar nas 3 plataformas | B2 | Design | Pending |
| CROSS-04 | P1: Compilar nas 3 plataformas | B2 (nota 3) | Design | Pending |
| CROSS-05 | P1: Compilar nas 3 plataformas | B4 | Design | Pending |
| CROSS-06 | P2: Áudio correto em qualquer driver | B5 | Design | Pending |
| CROSS-07 | P2: Áudio correto em qualquer driver | B6 | Design | Pending |
| CROSS-08 | P2: Áudio correto em qualquer driver | B7 | Design | Pending |
| CROSS-09 | P2: Áudio correto em qualquer driver | B8 | Design | Pending |
| CROSS-10 | P3: Ciclo de vida robusto | L1 | Design | Pending |
| CROSS-11 | P3: Ciclo de vida robusto | L2 | Design | Pending |
| CROSS-12 | P3: Ciclo de vida robusto | L3 | Design | Pending |
| CROSS-13 | P3: Ciclo de vida robusto | L4 | Design | Pending |
| CROSS-14 | P3: Ciclo de vida robusto | L5 | Design | Pending |
| CROSS-15 | P2: Áudio correto em qualquer driver | L6 | Design | Pending |
| CROSS-16 | P2: Áudio correto em qualquer driver | L7 | Design | Pending |
| CROSS-17 | P3: Ciclo de vida robusto | L8 | Design | Pending |
| CROSS-18 | P3: Ciclo de vida robusto | L9 | Design | Pending |
| CROSS-19 | P1: Compilar nas 3 plataformas | M1 | Design | Pending |
| CROSS-20 | P1: Compilar nas 3 plataformas | M2 | Design | Pending |
| CROSS-21 | P1: Compilar nas 3 plataformas | (novo — build system) | Design | Pending |
| CROSS-22 | P1 + P2: invariantes de RT-safety | (novo — constraint transversal) | Design | Pending |
| CROSS-23 | P1: Compilar nas 3 plataformas | B5 (docs) | Design | Pending |
| CROSS-24 | P1: Compilar nas 3 plataformas | (revisão — lacuna 3, D4) | Design | Pending |
| CROSS-25 | P3: Ciclo de vida robusto | (revisão — lacuna 6) | Design | Pending |
| CROSS-26 | P3: Ciclo de vida robusto | (revisão — lacuna 8, D1) | Design | Pending |
| CROSS-27 | P1: Compilar nas 3 plataformas | (revisão — lacuna 7) | Design | Pending |

### Descrição dos requisitos

| ID | Requisito |
|---|---|
| CROSS-01 | Descoberta de toolchain (`faust`, `mojo`) portável: `where.exe`/`which`, `%USERPROFILE%`/`$HOME`; Faust opcional quando os `.hpp` gerados existem |
| CROSS-02 | Nome e extensão da biblioteca neural derivados do target (`.so` / `.dylib` / `.dll`); linking condicional |
| CROSS-03 | `RustNeuralProcessor` — fallback puro-Rust do `mojo_process_block`, selecionado por capacidade (não por SO) |
| CROSS-04 | Teste de equivalência numérica Mojo ↔ Rust, tolerância ≤ 1e-6 |
| CROSS-05 | `f_size_t` = `size_t` em `dsp/wrapper.h` **e** `dsp/mlc_zero_v_wrapper.h` (LLP64 vs LP64) |
| CROSS-06 | Feature Cargo `asio` opt-in, dependência `cpal` target-gated a Windows |
| CROSS-07 | Braços `I32` e `I16` no stream de entrada, com pipeline DSP extraído para definição única |
| CROSS-08 | Braço `I16` de saída retira 2 amostras por frame |
| CROSS-09 | Nenhum frame descartado: processamento em blocos sobre buffers pré-alocados |
| CROSS-10 | Erros de stream e hot-plug propagados à UI por canal lock-free |
| CROSS-11 | Negociação via `SupportedStreamConfigRange` |
| CROSS-12 | Reconciliação de sample rate input/output |
| CROSS-13 | Resampling em tempo real (`rubato::Async`) com buffers pré-alocados **e compensação adaptativa de drift de clock** (D3) |
| CROSS-14 | Slider de latência reflete o buffer real do driver |
| CROSS-15 | Um único `cpal::Device` para duplex ASIO |
| CROSS-16 | Dispositivos com formato não suportado permanecem visíveis e marcados |
| CROSS-17 | CI matriz `ubuntu` / `windows` / `macos` |
| CROSS-18 | `set_latency_samples` reporta latência real, apenas quando muda |
| CROSS-19 | `-rpath` emitido para Linux **e** macOS |
| CROSS-20 | Artefactos `.dylib`/`.dll` reconhecidos e ignorados por git |
| CROSS-21 | Entrada de build unificada e portável (`cargo xtask`), substituindo `Makefile` + scripts bash |
| CROSS-22 | Invariantes preservados: FFI zero-copy, zero alocação, zero lock, latência não degradada |
| CROSS-23 | Documentação de pré-requisitos por plataforma (LLVM/libclang, ASIO SDK, Faust, Mojo) + guião de UAT manual para CoreAudio/ASIO |
| CROSS-24 | Estratégia de carregamento em runtime: `-rpath` para dev; bundles de release forçam o backend Rust e não carregam bibliotecas nativas externas |
| CROSS-25 | Identidade de dispositivo estável (índice + nome), tolerante a nomes duplicados e não-ASCII |
| CROSS-26 | Telemetria de overflow/underrun do ring buffer; nenhum drop silencioso |
| CROSS-27 | Deteção de `.hpp` obsoleto quando o Faust está ausente (comparação de mtime `.dsp` vs `.hpp`) |

---

## Success Criteria

- [ ] Os 3 jobs da matriz de CI (`ubuntu-latest`, `windows-latest`, `macos-latest`) passam `cargo build --release` e `cargo test --release`
- [ ] `cargo xtask check-env` corre e reporta corretamente em Linux, Windows e macOS; `Makefile` e `scripts/*.sh` deixaram de ser o caminho canónico
- [ ] `grep -rn "unsigned long" dsp/` não devolve resultados
- [ ] `grep -rn 'Command::new("which")' build.rs` não devolve resultados
- [ ] Teste `neural_backends_are_numerically_equivalent` passa em Linux (onde ambos os backends existem)
- [ ] Teste de contagem de drain I16 passa: `2N` push → `N` frames → ring buffer vazio
- [ ] Teste de não-truncamento passa: bloco de 4096 frames com buffers de 512 → 4096 frames processados
- [ ] Nenhum `#[cfg(target_os = ...)]` decide qual backend neural usar (usa-se `#[cfg(have_mojo)]`)
- [ ] O bench do pipeline DSP não regride mais de 2 % face ao baseline em `main` pré-feature
- [ ] `cargo test --features asio` compila em Windows com `ASIO_DIR` definido, e `cargo build` compila sem ele
- [ ] `docs/BUILD.md` documenta os pré-requisitos das 3 plataformas e é verificado por um developer em cada uma
- [ ] `nih_plug` `assert_process_allocs` continua ativo e nenhum teste de integração dispara o assert
- [ ] `ldd`/`otool -L` sobre o binário de um bundle de release não lista `libneural.*` nem bibliotecas do runtime Mojo
- [ ] `cargo test` passa sem `LD_LIBRARY_PATH`/`DYLD_LIBRARY_PATH` definidos no ambiente
- [ ] Toda a inserção no ring buffer que falha incrementa `overflows`; `grep -n "let _ = .*push(" src/bin/standalone.rs` devolve vazio
- [ ] Teste de drift: 10 min a 44100 (in) → 48000 (out) com clocks divergentes simulados (±100 ppm) mantém a ocupação do ring buffer dentro de ±10 % do alvo
- [ ] `grep -rn "FftFixedInOut\|SincFixedInOut" src/` devolve vazio (APIs inexistentes em `rubato 1.0.1`; os documentos nomeiam-nas apenas para registar a correção)
- [ ] Guião de UAT manual executado em macOS (CoreAudio, incl. um aggregate device) e em Windows (WASAPI + ASIO)

---

## Resolução dos Debates (D1–D4)

| # | Questão | Posição | Requisito |
|---|---|---|---|
| D1 | Output drop aceitável quando o ring enche? | **Drop é aceitável; drop silencioso não.** Ver abaixo | CROSS-26 |
| D2 | Fallback Rust default em macOS sem Mojo? | **Sim — e é o backend de release mesmo com Mojo presente** | CROSS-24 |
| D3 | Resampling fixo ou adaptativo? | **Adaptativo.** Fixo é tecnicamente impossível de corrigir a posteriori | CROSS-13 |
| D4 | Bundles carregam Mojo? | **Não.** Mojo é toolchain de dev/Linux; releases usam Rust | CROSS-24 |

**D1** — Truncar o input (CROSS-09) destrói informação que nunca mais existe; descartar output quando o
ring enche é uma consequência *inevitável* de o consumidor ser mais lento que o produtor, e a única
alternativa seria bloquear a thread de áudio. O drop mantém-se permitido. O que passa a ser proibido é
o `let _ = producer.push(x)` que o esconde: cada falha de `push` incrementa `overflows`. Com o
controlador de drift (D3), um overflow sustentado deixa de ser aceitável em regime permanente — passa
a ser o sinal de entrada do controlador.

**D2** — O teste de equivalência (CROSS-04) exige |Δ| ≤ 1e-6 entre os backends. Se o Rust é
indistinguível do Mojo, "paridade" não é argumento para exigir Mojo em macOS: os 27 linhas de tanh
polinomial não justificam adicionar um SDK à infraestrutura de release. O Mojo permanece compilável e
testado em macOS para desenvolvimento; simplesmente não é o backend de release.

**D3** — Esta não é uma escolha de gosto. `rubato::Fft` (o resampler síncrono) devolve
`ResampleError::SyncNotAdjustable` em `set_resample_ratio` **e** em `set_resample_ratio_relative`
(`synchro.rs:606,616`). Um resampler de rácio fixo é, por construção, incapaz de corrigir drift: dois
cristais nominalmente a 48 kHz divergem tipicamente ±50 ppm, o que acumula ≈ 4.3 s de erro por dia, ou
um overflow/underrun do ring buffer a cada poucos minutos. A única API de `rubato 1.0.1` com trim de
rácio é `Async` (`asynchro.rs:161,242`), cujo parâmetro `max_resample_ratio_relative` existe
precisamente para isto. Portanto: `Async::new_sinc` + controlador proporcional sobre a ocupação do
ring buffer.

**D4** — Embutir `libneural.dylib` num `.vst3` exige reescrever o load path para `@loader_path`,
codesignar a lib aninhada separadamente, e ainda arrastar o runtime do Mojo. O ganho audível é nulo
(D2). `cargo xtask bundle` define `DISTORTION_FORCE_RUST_NEURAL=1`, que `build.rs` respeita
suprimindo `have_mojo`. Consequência: o `-rpath` de CROSS-19 serve **apenas** builds de
desenvolvimento e `cargo test`, o que resolve a lacuna 3 sem qualquer mecanismo de descoberta em
runtime.
