# Documentação da Estrutura do Projeto - Distortion Plugin

Esta documentação descreve a nova organização modular do projeto após a refatoração arquitetural, detalhando a responsabilidade de cada arquivo na interface e no processamento.

## 📂 Visão Geral da Estrutura `src/`

O projeto segue agora uma separação clara entre a lógica do hospedeiro (Plugin/Standalone) e o "Core" da aplicação (Lógica, Áudio e UI).

---

### 1. 🔌 Camada de Entrada (Entry Points)

*   **`lib.rs`**: O coração do plugin (VST3/CLAP). Ele gerencia o ciclo de vida do plugin, a comunicação com a DAW e delega a renderização da UI para o módulo de visualização compartilhada.
*   **`bin/standalone.rs`**: O aplicativo executável que roda fora de uma DAW. Ele configura o áudio do sistema (CPAL) e usa os mesmos componentes de interface do plugin para manter a consistência.

---

### 2. 🧠 Núcleo do Aplicativo (`src/core/`)

O diretório `core` é onde reside a inteligência do plugin, dividido em três domínios principais:

#### 📂 `state/` (Gerenciamento de Dados)
*   **`mod.rs`**: Expõe o sub-módulo de estado.
*   **`plugin_params.rs`**: Define todos os parâmetros que o usuário pode alterar (Ganho, Bypass, Seleção de Entrada) e o estado interno da GUI (`EditorState`), como se o painel de configurações está aberto ou não.

#### 📂 `dsp/` (Processamento de Sinal Digital)
*   **`mod.rs`**: Centraliza e expõe as funcionalidades de processamento.
*   **`analyzer.rs`**: Contém a lógica matemática do Analisador de Espectro. Utiliza FFT (Fast Fourier Transform) para converter o som em dados visuais de frequências/decibéis.

#### 📂 `ui/` (Interface do Usuário)
*   **`mod.rs`**: Organiza e expõe os componentes visuais.
*   **`spectrum.rs`**: Responsável exclusivo por desenhar o gráfico do espectro (as linhas de frequência e a curva do áudio).
*   **`signal_chain.rs`**: Desenha a "Cadeia de Sinal", que é a linha horizontal com o bloco interativo (ícone "DIST"). Gerencia o clique para abrir/fechar o painel.
*   **`main_view.rs`**: **O Componente Mestre.** Ele organiza o layout (Header, Gráfico, Cadeia de Sinal e Painel de Configurações). É chamado tanto pelo `lib.rs` quanto pelo `standalone.rs`, garantindo que a interface seja idêntica em ambos.

---

## 🛠️ Fluxo de Funcionamento Atual

1.  O **Áudio** entra pelo `process()` (em `lib.rs`) ou pelo worker (em `standalone.rs`).
2.  O **Estado** checa se o `Bypass` está ativo.
3.  O **DSP** processa o sinal e envia uma cópia para o `analyzer.rs`.
4.  O **Main View** recebe os dados do espectro e renderiza os painéis usando os componentes de `spectrum.rs` e `signal_chain.rs`.
5.  O **Usuário** interage com o bloco na cadeia de sinal, o que altera o estado em `plugin_params.rs` e faz o painel inferior aparecer ou sumir através do eGUI.
