# PLAN: Signal Chain UI & Spectrum Analyzer

## 1. Contexto e Objetivos
O objetivo desta tarefa é implementar uma interface de usuário no plugin de áudio usando `egui` que inclua:
- Um **Analista de Espectro redimensionável** que exibe o sinal de saída final da cadeia de áudio.
- Uma **Cadeia de Sinal visual** (uma linha reta) localizada logo abaixo do analista de espectro.
- Um **Ícone/Caixa** no final da cadeia de sinal (preparado para ser arrastável no futuro).
- Um **Painel Expansível** que já vem aberto por padrão, mostrando opções genéricas do que pode ser alterado na caixa.
- Um **Botão de Bypass** global: quando ativado, desativa o processamento da cadeia, fazendo o analista de espectro exibir o sinal de entrada cru.

## 2. Escopo e Restrições
- **Apenas Visual (Painel)**: As opções no painel, por enquanto, não terão funcionalidade real de alteração de áudio, servindo apenas como mockup visual.
- **Roteamento do Espectro**: A lógica do DSP ou passagem de buffers de áudio para a UI precisará capturar o sinal *após* o processamento (sinal final) para enviá-lo ao analisador, exceto quando o Bypass estiver ativo.
- **Redimensionamento**: O `egui::Plot` do espectro deve ser configurado de forma fluida ou estar dentro de um container/painel que aceite redimensionamento (`egui::TopBottomPanel::resizable` ou divisores customizados).

## 3. Arquitetura da UI (Componentes eGUI)

### 3.1. Layout Central
- **Top / Central Panel**: Conterá o Analisador de Espectro.
  - Pode utilizar um `TopBottomPanel` com a flag de `resizable(true)` para permitir que o usuário altere a altura do analisador, ou incorporar um componente divisor customizado se estiver no painel central.
  - O gráfico exibirá os dados do buffer de saída do DSP.
- **Middle Section**: Onde ficará a Cadeia de Sinal (A "Linha Reta").
  - Utilizar a **Painter API (`ui.painter()`)** para desenhar a linha baseada no `Rect` alocado disponível.
  - Desenhar o "Node/Caixa" sobre a linha.
- **Bottom Section**: Painel de Configurações da Caixa.
  - Um painel que entra em exibição condicional (mas iniciado como `true`).
  - Mostrará *sliders* ou *knobs* simulados.

### 3.2. Controles Globais
- **Botão Bypass**: Pode ser colocado na barra superior (Top Bar) ou ao lado da cadeia de sinal. 
  - Variável de estado: `bypass_active: bool`.

## 4. Integração com o DSP

- **Lógica de Áudio (`process` block do plugin):**
  - O buffer entra -> passa pela cadeia de sinal simulada -> buffer sai.
  - Se `bypass_active` for `true`, copia a entrada para a saída sem processar.
  - O envio de dados para o atualizador do Analisador de Espectro deve vir **depois** do bloco `if !bypass_active { processa(); }`. Assim o analisador sempre lerá o resultado final, seja ele o cru (bypass = true) ou o filtrado (bypass = false).

## 5. Passos de Implementação (Task Breakdown)

- [ ] **Passo 1: Estado da UI e Modelo de Dados**
  - Adicionar ao struct da UI (ou state) variáveis: `panel_open: bool = true`, `bypass_active: bool = false`, `node_position_x: f32`.
- [ ] **Passo 2: Estruturar o Redimensionamento do Analisador**
  - Modificar a renderização atual do espectro para suportar área variável.
  - Alojar o gráfico num container redimensionável (`egui::TopBottomPanel::resizable` ou área flexível).
- [ ] **Passo 3: Widget Customizado (A Cadeia de Sinal)**
  - Construir um componente/espaço customizado logo abaixo do analisador usando `ui.allocate_space`.
  - Desenhar a linha base (`painter.line_segment`).
  - Desenhar o bloco da caixa sobre o fim da linha.
- [ ] **Passo 4: Painel de Configuração Default**
  - Checar a flag `panel_open`. Se `true`, reservar bloco abaixo com `ui.group()` ou outro painel e populá-lo com opções fakes (Sliders mock).
- [ ] **Passo 5: Botão Bypass & Lógica do Buffer**
  - Inserir botão de Bypass (ex: `ui.toggle_value`).
  - Adequar o código Rust (DSP) para enviar o array de áudio de saída ao visualizador dependendo dessa flag.

## 6. Agentes Recomendados (Planejamento)
- `frontend-specialist`: Para montar a arquitetura eGUI de painéis redimensionáveis e Widget customizado.
- `backend-specialist`: Para garantir que a cópia do buffer no loop em tempo real obedeça o Bypass de forma *thread-safe* sem engasgos no áudio.
