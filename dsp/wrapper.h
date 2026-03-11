#ifndef FAUST_WRAPPER_H
#define FAUST_WRAPPER_H

// Usando unsigned long explicitamente invés de stddef para evitar Clang issues de sysroot local
typedef unsigned long f_size_t;

#ifdef __cplusplus
extern "C" {
#endif

// Opaque pointer guardando a instância do Faust
typedef void* FaustHandle;

// Cria uma nova instância do processador Faust
FaustHandle faust_create();

// Inicializa a instância com o sample rate
void faust_init(FaustHandle handle, float sample_rate);

// Processa o bloco de áudio (substitui no mesmo buffer)
void faust_process(FaustHandle handle, float* buffer, f_size_t length);

// Libera a memória alocada
void faust_destroy(FaustHandle handle);

// --- Novos Parâmetros FFI (Equalizador) ---
void faust_set_eq_low_freq(FaustHandle handle, float freq);
void faust_set_eq_low_gain(FaustHandle handle, float gain);
void faust_set_eq_low_q(FaustHandle handle, float q);

void faust_set_eq_mid_freq(FaustHandle handle, float freq);
void faust_set_eq_mid_gain(FaustHandle handle, float gain);
void faust_set_eq_mid_q(FaustHandle handle, float q);

void faust_set_eq_high_freq(FaustHandle handle, float freq);
void faust_set_eq_high_gain(FaustHandle handle, float gain);
void faust_set_eq_high_q(FaustHandle handle, float q);

#ifdef __cplusplus
}
#endif

#endif // FAUST_WRAPPER_H
