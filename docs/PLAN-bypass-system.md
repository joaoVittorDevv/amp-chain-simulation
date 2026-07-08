# PLAN-bypass-system.md - Planejamento de Implementação de Ativação Modular

Este plano detalha as alterações necessárias para adicionar ativação unitária para os módulos de AMP (Preamp) e Cabinet, além de manter o bypass global da cadeia de sinal no plugin de áudio.

---

## 📅 Visão Geral
O objetivo é permitir que o usuário ative ou desative cada módulo individualmente. O bypass global continuará existindo, mas quando ativo, os módulos individuais deverão refletir visualmente o estado inativo ("apagados").

## 🧩 Componentes Afetados

1.  **Parâmetros do Plugin (`src/core/state/plugin_params.rs`)**:
    *   Adicionar `preamp_active: BoolParam` (Default: `true`).
    *   Adicionar `cabinet_active: BoolParam` (Default: `true`).
2.  **Lógica de Processamento (`src/lib.rs`)**:
    *   Atualizar o método `process` de `BaseIO`.
    *   Regra: `if !bypass { if preamp_active { process_preamp() } if cabinet_active { process_cabinet() } }`.
3.  **Interface do Usuário (UI)**:
    *   **Cadeia de Sinal (`src/core/ui/signal_chain.rs`)**:
        *   Adicionar um botão circular de "Power" no canto superior de cada bloco.
        *   O bloco deve ficar acinzentado se (`!preamp_active` ou `bypass`).
        *   Implementar a lógica de clique para o botão de power.
    *   **Main View (`src/core/ui/main_view.rs`)**: Passar os novos parâmetros para a função de renderização da cadeia.

---

## 🚀 Fases de Implementação

### Fase 1: Atualização dos Parâmetros
*   [x] Definir `preamp_active` e `cabinet_active` em `BaseIOParams`.
*   [x] Inicializar como `true` no `impl Default for BaseIOParams`.

### Fase 2: Implementação da Lógica de DSP
*   [x] Ajustar o loop de processamento em `src/lib.rs`.

### Fase 3: UI - Botões de Power e Visual
*   [x] Criar ícone de Power básico usando o painter do egui.
*   [x] Adicionar detecção de clique para o botão de power.
*   [x] Alterar cores baseadas no estado de ativação.

### Fase 4: Testes e Validação
*   [ ] Testar no modo Standalone.
*   [ ] Testar persistência de parâmetros.

---

## ✅ Checklist de Verificação
- [x] Áudio passa limpo quando `bypass == true`.
- [x] Cada módulo pode ser ativado/desativado de forma independente.
- [x] O visual "apagado" funciona corretamente para bypass global e individual.

