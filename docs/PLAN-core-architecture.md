# PLAN: Arquitetura e Refatoração do Core

## 1. Contexto e Objetivos
O objetivo desta tarefa é reestruturar o código fonte do plugin (`src/`) implementando princípios sólidos de Arquitetura de Software. Atualmente, o projeto possui as seguintes **Trade-offs (Dificuldades)**:
- **Violação do DRY (Don't Repeat Yourself)**: A montagem completa dos painéis UI (Painel Superior de Espectro, Painel Inferior de Cadeia, Checkboxes) está duplicada entre o `src/lib.rs` e o `src/bin/standalone.rs`.
- **Sobrecarga de Responsabilidade (Fat Modules)**: O arquivo `src/lib.rs` contém lógica do Plugin VST, Parâmetros, Setup do Editor e Loop de Processamento. O arquivo `standalone.rs` possui toda a lógica do CPAL *junto* com a renderização principal da tela.
- **Acoplamento**: A interface gráfica não tem um arquivo próprio para organizar a *Árvore de Componentes*; os componentes apenas são jogados diretamente dentro da declaração do eGUI.

## 2. Nova Estrutura de Diretórios Proposta

Vamos dividir as responsabilidades em pequenos módulos dentro de `src/core/`.

```text
src/
├── bin/
│   └── standalone.rs        (Lidará apenas com: CPAL, Worker de Áudio e Wrapper do eframe)
├── core/
│   ├── mod.rs               (Exposição dos sub-módulos)
│   ├── dsp/                 (Domínio: Processamento de Áudio)
│   │   ├── mod.rs
│   │   ├── analyzer.rs      (Responsável pelo AnalyzerDsp, realfft e visualização)
│   │   └── processor.rs     (Nova casa para a lógica bruta que hoje fica em lib.rs -> process())
│   ├── ui/                  (Domínio: Interface do Usuário)
│   │   ├── mod.rs
│   │   ├── spectrum.rs      (Renomeado. Atual draw_spectrum)
│   │   ├── signal_chain.rs  (Renomeado. Atual draw_signal_chain)
│   │   └── main_view.rs     (Novo! Ponto de união para as janelas que ambos DAW e Standalone vão compartilhar)
│   └── state/               (Domínio: Parâmetros e Modelo)
│       ├── mod.rs
│       └── plugin_params.rs (Nova casa para EditorState, BaseIOParams, InputSelect)
└── lib.rs                   (Ficará "magro", servindo apenas como uma camada de delegação para o core)
```

## 3. Passos de Implementação (Task Breakdown)

### Fase 1: Domínio de Parâmetros e Estado
- [ ] Criar diretório `src/core/state` e adicionar `plugin_params.rs`.
- [ ] Transferir `BaseIOParams`, struct `EditorState` e o enum `InputSelect` do `lib.rs` para o novo módulo.
- [ ] Corrigir referências e publicações no `mod.rs`.

### Fase 2: Domínio de Processamento de Áudio (DSP)
- [ ] Agrupar e reorganizar as ferramentas de DSP dividindo o atual `dsp.rs` dentro de uma pasta `dsp/`.
- [ ] Mover as coisas pertencentes à visualização (`AnalyzerDsp`) para `src/core/dsp/analyzer.rs`.
- [ ] Isolar a lógica do processamento cru do áudio (`fn process(...)` no actual `lib.rs`) em algum formatador reutilizável se possível (Opcional, focar na clareza do `lib.rs`).

### Fase 3: Domínio de Interface do Usuário (UI shared)
- [ ] Separar `draw_spectrum` e `draw_signal_chain` em seus próprios arquivos (`spectrum.rs` e `signal_chain.rs`).
- [ ] **Crucial:** Criar `main_view.rs`. Esse arquivo terá uma função com a assinatura de desenhar os Painéis Centrais, Superiores e Inferiores de forma genérica para o plugin.
- [ ] Modificar o `lib.rs` e o `standalone.rs` para simplesmente importarem e chamarem `main_view::render_plugin_app_layout(ctx, state)`.

### Fase 4: Limpeza e Polimento do Código
- [ ] Revisar as importações (`use` statements) que ficarão órfãs na refatoração.
- [ ] Validar a segurança e performance na separação de módulos.

## 4. Checklist Socrático (Análise de Trade-off)
- *Trade-off*: Vai adicionar mais arquivos ao projeto? *Resposta*: Sim, aumentará o boilerplate inicial de `mod.rs` no Rust, porém deixará cada escopo perfeitamente isolado ajudando muito o manuseio futuro de múltiplas visualizações na "Cadeia de Sinal".
- *Segurança*: A implementação compartilhada requer que entendamos exatamente as limitações ao compartilhar o Contexto do eGUI para ambos (`standalone` vs `plugin`).

## 5. Agentes Recomendados (Implementação)
- `project-planner`: Coordenação global a cada finalização de fase.
- `backend-specialist`: Para lidar perfeitamente com a estruturação de Módulos (Rust Modules `mod.rs`) e reconfiguração do DSP.
- `frontend-specialist`: Para lidar com a injeção universal das telas do eGUI (removendo duplicações entre standalones e DAWs).
