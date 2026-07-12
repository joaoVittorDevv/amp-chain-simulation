# Relatório: redução de latência ASIO no standalone Windows

Data da análise: 12 de julho de 2026

Escopo: executável `standalone` usando o host ASIO no Windows

Objetivo: reduzir a latência de monitoração da entrada física até a saída física sem comprometer a estabilidade do áudio em tempo real.

## Resumo executivo

O maior ganho imediato não depende de trocar o DSP. Hoje o standalone abre os streams com `cpal::BufferSize::Default`, portanto o valor escolhido na interface **não solicita um buffer menor ao driver ASIO**. Esse valor controla somente a folga do ring buffer entre os callbacks de entrada e saída.

Além disso, o valor inicial atual é 2048 frames. Em 48 kHz isso mantém aproximadamente **42,7 ms apenas no ring**, antes de considerar o bloco de entrada, o bloco de saída, os buffers internos do driver e qualquer reamostragem. Por isso, selecionar ASIO por si só não produz a latência esperada de um aplicativo para guitarra em tempo real.

As prioridades recomendadas são:

1. abrir entrada e saída ASIO com um tamanho fixo negociado, começando em 128 frames;
2. separar na interface o buffer do driver da folga de segurança interna;
3. reduzir a folga do ring para um bloco no caminho ASIO duplex e evitar o padrão de 2048 frames;
4. retirar `Mutex` e trabalho dispensável dos callbacks;
5. medir tempo de DSP e latência física de ida e volta antes de reduzir para 64 ou 32 frames;
6. atualizar o CPAL em uma etapa isolada, pois as versões atuais contêm correções ASIO relevantes.

Uma primeira meta realista é **até 10 ms de latência física de ida e volta em 48 kHz/128 frames**, sem underruns durante 30 minutos. Depois disso, 64 frames pode ser oferecido como modo de baixa latência para máquinas e interfaces que passarem no teste de estabilidade.

## Como o áudio funciona atualmente

No standalone, o callback de entrada é o dono do processamento:

1. recebe o bloco intercalado do dispositivo;
2. converte `F32`, `I32` ou `I16` para os buffers internos `f32`;
3. aplica roteamento mono/estéreo;
4. executa EQ, pré-EQ, amplificador Neural ou MLC Zero V, cabinet, ganho, limiter e saneamento;
5. opcionalmente reamostra para a taxa da saída;
6. publica L/R no ring de playthrough.

O callback de saída retira amostras desse ring e as escreve no dispositivo. Essa separação é necessária para dispositivos com clocks independentes, mas adiciona uma quantidade configurável de áudio armazenado entre captura e reprodução.

No ASIO duplex, as alterações locais já procuram usar o mesmo `cpal::Device` para entrada e saída. Esse é o caso que deve receber um caminho de baixa latência específico: mesmo driver, mesma taxa e nenhum resampler.

## Composição da latência

A interface atual aproxima a latência com:

```text
bloco observado de entrada + preenchimento do ring + bloco observado de saída
```

Essa conta é útil para observar tendências, mas não representa toda a latência física. Ela não inclui de forma completa:

- conversores A/D e D/A da interface;
- buffers internos informados pelo driver ASIO;
- atraso de segurança próprio do hardware;
- tempo efetivo gasto pelo DSP;
- espera parcial e atraso de filtro do resampler;
- possíveis buffers adicionais mantidos pelo backend.

Com a configuração atual de 2048 frames, somente a folga-alvo representa:

| Taxa | Folga de 2048 frames |
| --- | ---: |
| 44,1 kHz | 46,4 ms |
| 48 kHz | 42,7 ms |
| 96 kHz | 21,3 ms |

Como referência, um bloco de 128 frames representa 2,67 ms em 48 kHz. Entrada + um bloco de ring + saída resultam em aproximadamente 8 ms de software antes dos atrasos adicionais do hardware.

## O que as alterações locais já melhoram

As mudanças ainda não commitadas atacam estabilidade e compatibilidade, mas não resolvem sozinhas a principal fonte de latência:

- passam a aceitar `I32` também na saída, necessário para drivers ASIO de 24 bits;
- evitam manter vários drivers ASIO carregados durante a resolução do dispositivo;
- compartilham um único `Device` no caso ASIO duplex;
- tentam reabrir o driver após uma troca de roteamento;
- pré-preenchem o ring com uma folga determinística;
- recuperam underruns aguardando o ring recompor a folga;
- descartam excesso quando deriva de clock faz o ring crescer;
- ajustam o resampler para manter a folga-alvo;
- mostram blocos observados, preenchimento do ring, underruns e overflows;
- reduzem o custo do MLC para fontes mono e habilitam fast-math no C++.

Essas mudanças tornam o sistema mais previsível. Entretanto, o código continua substituindo o tamanho negociado por `BufferSize::Default`, e a folga inicial continua em 2048 frames.

## Plano recomendado

### P0 — medir uma linha de base reproduzível

Antes de mudar os defaults, registrar para pelo menos uma interface ASIO nativa:

- modelo da interface, driver e firmware;
- 48 kHz com 32, 64, 128, 256, 512 e 2048 frames;
- tamanho solicitado, tamanho observado em cada callback e tamanho reportado pelo driver;
- tempo médio, p95, p99 e máximo do processamento do callback de entrada;
- underruns, overflows e ressincronizações;
- modelo Neural e MLC Zero V, mono e estéreo;
- latência física medida por loopback, com cabo da saída para a entrada.

O teste deve usar build `release`, sinal conhecido e duração mínima de 30 minutos para cada configuração candidata. A medição física deve ser a fonte de verdade; a estimativa da interface é telemetria auxiliar.

### P0 — negociar de verdade o buffer ASIO

No caminho ASIO, trocar o `BufferSize::Default` incondicional por uma política explícita:

1. obter o intervalo anunciado pelo `SupportedBufferSize`;
2. limitar o valor solicitado ao intervalo do driver;
3. tentar `BufferSize::Fixed(valor)` na entrada e na saída;
4. exigir o mesmo valor para o par duplex;
5. se o driver rejeitar, tentar os tamanhos suportados vizinhos;
6. usar `Default` somente como fallback visível, nunca silencioso.

O CPAL documenta que `Default` usa a configuração do host, que pode ser grande, enquanto `Fixed` é a opção indicada quando baixa latência é desejada. O tamanho real ainda deve ser observado nos callbacks, pois o hardware pode arredondar ou variar o pedido.

Default recomendado para ASIO:

- 128 frames em 44,1/48 kHz;
- 256 frames como modo estável;
- 64 frames como modo de baixa latência, habilitado após validação;
- 32 frames apenas como modo experimental.

O aplicativo deve mostrar claramente “solicitado”, “negociado/observado” e qualquer fallback aplicado.

### P0 — separar buffer do driver e folga do ring

O controle atual mistura dois conceitos diferentes. A interface deve apresentar:

- **Buffer ASIO**: tamanho solicitado ao driver;
- **Folga de segurança**: quantidade que o ring procura manter entre os callbacks.

Para o mesmo dispositivo ASIO duplex, iniciar a folga em **um bloco negociado**, e não em 2048 frames. Oferecer perfis:

| Perfil | Buffer ASIO | Folga do ring | Uso esperado |
| --- | ---: | ---: | --- |
| Baixa latência | 64 | 1 bloco | máquina/interface validada |
| Equilibrado | 128 | 1 bloco | padrão recomendado |
| Estável | 256 | 2 blocos | DSP pesado ou driver sensível |

Quando entrada e saída forem dispositivos diferentes, manter a política mais conservadora e a compensação de drift. Esse cenário não deve ser apresentado como o modo ASIO de menor latência.

### P0 — garantir que o DSP caiba no prazo

O prazo de um callback é `frames / sample_rate`:

| Frames em 48 kHz | Prazo total |
| ---: | ---: |
| 32 | 0,67 ms |
| 64 | 1,33 ms |
| 128 | 2,67 ms |
| 256 | 5,33 ms |

O tempo de DSP p99 deve ficar abaixo de 70% desse prazo, deixando margem para conversão, ring, driver e variações do agendador. Para 128 frames, o alvo p99 é menor que 1,87 ms; para 64 frames, menor que 0,93 ms.

Manter e validar as otimizações locais de fast-math e processamento mono do MLC. Se o MLC estéreo não cumprir o orçamento em 128 frames, o aplicativo deve impedir o perfil incompatível ou aplicar degradação controlada; não deve depender apenas de aumentar silenciosamente a folga.

### P1 — remover contenção dos callbacks

Atualmente os endpoints do ring são envolvidos em `Arc<Mutex<...>>` e acessados com `try_lock`. Embora não bloqueie, uma falha de aquisição produz silêncio ou descarta áudio, e a tentativa de lock ocorre no caminho de tempo real.

Refatorar a criação dos streams para que:

- o callback de entrada possua diretamente o `Producer`;
- o callback de saída possua diretamente o `Consumer`;
- cada tentativa de configuração crie seu próprio par de endpoints;
- a UI receba apenas contadores atômicos, sem tocar nos endpoints de playthrough.

O analisador deve continuar isolado: perda de dados visuais nunca deve contar como overflow do caminho audível.

### P1 — caminho ASIO duplex sem resampler

Para entrada e saída do mesmo driver ASIO:

- negociar uma única taxa comum;
- recusar iniciar o modo de baixa latência se as taxas divergirem;
- manter `RtResampler` desativado nesse caminho;
- mostrar explicitamente quando o resampler estiver ativo.

O resampler atual trabalha em chunks de 512 frames. Em 44,1 kHz ele pode esperar até aproximadamente 11,6 ms para completar um chunk, além do atraso do filtro. Portanto, uma sessão com resampler ativo não deve anunciar o mesmo perfil de baixa latência do ASIO duplex nativo.

### P1 — atualizar o CPAL em uma mudança isolada

O projeto usa CPAL 0.15.2. A linha atual do CPAL contém mudanças importantes para este trabalho, incluindo consulta do tamanho negociado do stream e correções ASIO para:

- enumeração que retornava apenas o primeiro dispositivo ao usar `collect()`;
- enumeração e criação de stream a partir de threads criadas pela aplicação;
- alinhamento de `BufferSize::Fixed` às restrições de passo do driver;
- mudanças de buffer notificadas pelo driver;
- mudanças de latência notificadas pelo driver;
- callbacks duplicados de drivers não conformes;
- relatório de overload.

A atualização deve ser um PR/commit separado, com adaptação da API e toda a matriz de testes Windows. Parte das alterações locais de identidade pode se tornar redundante depois da atualização, mas só deve ser removida após teste com interfaces reais e nomes duplicados.

### P1 — melhorar a telemetria de latência

Separar na interface:

- buffer solicitado;
- bloco observado de entrada;
- folga-alvo e preenchimento atual do ring;
- bloco observado de saída;
- atraso reportado pelo backend/driver, quando a versão do CPAL oferecer a consulta;
- atraso do resampler e sua espera de staging;
- estimativa de software;
- resultado da última medição física, quando executada pelo teste de loopback.

Não chamar a soma atual de “latência total” sem qualificação. Usar “estimativa do caminho de software”, pois A/D, D/A e buffers privados do driver não são observáveis por essa conta.

### P2 — prioridade de thread e comportamento do Windows

Após atualizar o CPAL, avaliar a promoção suportada da thread de áudio para prioridade de tempo real/MMCSS no Windows. A mudança só deve ser mantida se:

- ocorrer fora do callback;
- falhar de forma segura e observável;
- reduzir picos de tempo sem prejudicar o sistema;
- passar por testes prolongados com WASAPI e ASIO.

Prioridade mais alta não corrige DSP acima do orçamento e não substitui buffers corretamente negociados.

### P2 — ajuste automático opcional

Depois que as medições forem confiáveis, oferecer um assistente:

1. inicia em 256 frames;
2. testa 128 e 64 frames por um período controlado;
3. monitora p99 do callback e glitches;
4. escolhe o menor tamanho que mantém margem;
5. persiste o resultado por identidade do dispositivo, taxa e modelo de amplificador.

Nunca reduzir automaticamente durante uma apresentação ou sessão ativa. A troca exige reconstruir os streams e deve ser uma ação explícita.

## Critérios de aceite

### Funcionais

- O valor de buffer ASIO selecionado é realmente solicitado aos dois streams.
- A interface mostra o tamanho solicitado e o observado.
- Entrada e saída duplex usam o mesmo dispositivo, taxa e tamanho.
- O modo de baixa latência não ativa resampler silenciosamente.
- Trocar de buffer reconstrói os streams sem deixar um driver ASIO preso.
- `I32`, `I16` e `F32` continuam funcionando conforme anunciado pelo driver.

### Desempenho e estabilidade

- 48 kHz/128 frames, perfil equilibrado: 30 minutos sem underrun ou overflow.
- DSP p99 abaixo de 70% do prazo do callback.
- Latência física de ida e volta até 10 ms em hardware compatível.
- 48 kHz/64 frames: oferecido somente quando p99 e teste de 30 minutos passarem.
- Nenhuma alocação, espera bloqueante ou log formatado dentro dos callbacks.
- Analyzer cheio ou UI lenta não afetam o áudio.

### Regressão

- WASAPI mantém sua política atual, salvo mudança deliberada e testada.
- ASIO de 24 bits continua abrindo entrada e saída `I32`.
- Dispositivos com nomes duplicados continuam sendo resolvidos pela identidade correta.
- Hot-unplug mantém a aplicação aberta e apresenta erro recuperável.
- O fallback para buffer padrão é mostrado ao usuário e registrado na telemetria.

## Ordem sugerida de implementação

1. adicionar medição do tempo do callback e teste físico de loopback;
2. separar os controles de buffer ASIO e folga do ring;
3. implementar tentativa `Fixed(128)` com negociação e fallback visível;
4. reduzir o ring ASIO duplex para um bloco;
5. remover `Mutex` dos endpoints de playthrough;
6. validar MLC/Neural em 128 e 64 frames;
7. atualizar o CPAL isoladamente;
8. integrar tamanho e latência reportados pela nova API;
9. avaliar prioridade de thread e ajuste automático.

## Referências primárias

- [CPAL — documentação de `BufferSize`](https://docs.rs/cpal/latest/cpal/enum.BufferSize.html)
- [CPAL — repositório e configuração do backend ASIO](https://github.com/RustAudio/cpal)
- [CPAL — changelog atual](https://docs.rs/crate/cpal/latest/source/CHANGELOG.md)
- [CPAL — guia de atualização](https://docs.rs/crate/cpal/latest/source/UPGRADING.md)

## Decisão recomendada

Implementar primeiro `Fixed(128)` + ring de um bloco + telemetria correta, mantendo 256 frames como fallback de estabilidade. Esse conjunto ataca diretamente a latência adicionada pelo software sem depender de uma reescrita do pipeline. A atualização do CPAL deve vir logo depois, mas isolada, para não misturar mudanças de API com a mudança de política de buffer.
