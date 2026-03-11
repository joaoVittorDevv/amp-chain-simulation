// neural_amp/mod.rs
//
// Este módulo existia para encapsular a inferência ONNX (tract-onnx) em uma thread de background.
// A inferência ONNX foi REMOVIDA e substituída pelo processador Mojo (FFI Zero-Copy).
//
// O processamento neural agora é gerenciado diretamente pelo MojoProcessor em src/bridge/mojo.rs.
// Este módulo é mantido para preservar a estrutura de diretórios do projeto.
//
// Arquitetura anterior (ONNX):
//   Thread Áudio → RingBuffer → Thread Inferência (Tract/ONNX) → RingBuffer → Thread Áudio
//
// Arquitetura atual (Mojo):
//   Thread Áudio → FFI direta → Mojo (libneural.so) [in-place, zero-copy, síncrono]
//
// ⚠️ REAL-TIME SAFETY:
// O `mojo_process_block` é compilado nativamente pelo Mojo (MLIR/LLVM).
// Não há alocação de memória, nem syscalls no caminho de processamento.
