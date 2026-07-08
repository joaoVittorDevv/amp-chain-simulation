# Plano de Integração: LibTorch & Neural Amp Standalone (V2 - Upgrade de API)

## Objetivo
Resolver a incompatibilidade de API C++ entre a crate `tch` (v0.23.0) e o LibTorch (v2.2.2). O LibTorch atual é antigo demais e não fornece as APIs necessárias para as bindings da v0.23.0.

---

## 📋 Fase 1: Atualização de Infraestrutura (DevOps)
O código gerado pela crate falha porque funções como `_assert_scalar` e `_batch_norm_no_update` não existem na v2.2.2 do LibTorch. É necessário o **Upgrade para PyTorch 2.4.0 ou 2.5.0**.

- [ ] **Download do Novo LibTorch:**
    *   Substituir a pasta `~/libtorch` pela versão **2.4.0 (CPU)** ou superior.
- [ ] **Remoção de Bypasses:**
    *   Remover `LIBTORCH_BYPASS_VERSION_CHECK` do Makefile, pois precisamos de uma versão compatível de verdade de agora em diante.
- [ ] **Reinjeção no Makefile:**
    *   Manter a exportação do `LIBTORCH` automático.

## 📋 Fase 2: Sincronização DSP (Backend)
- [ ] **Concluir Correções no Standalone:**
    *   Sincronizar os botões e loops faltantes (já em andamento, aguardando compilação).

## 📋 Fase 3: Validação (Testing)
- [ ] **Execução com Novo LibTorch:**
    *   Verificar se a compilação de `torch-sys` passa com os novos headers.
- [ ] **Verificação de Runtime:**
    *   Garantir que a inferência do modelo continua funcionando.
