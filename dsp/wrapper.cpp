#include "wrapper.h"
#include <iostream>

// Esse C++ será compilado e linkado na aplicação Rust via cc.
// Quando os arquivos .hpp reais forem gerados pelo faust -lang cpp,
// este wrapper vai apenas instanciar a classe e invocar compute().

extern "C" {

FaustHandle faust_create() {
    // Retorna ponteiro opaco (mock por enquanto)
    int* mock_instance = new int(1);
    return (FaustHandle)mock_instance;
}

void faust_init(FaustHandle handle, float sample_rate) {
    if (!handle) return;
    // std::cout << "[FAUST] Initialized at " << sample_rate << "Hz" << std::endl;
}

void faust_process(FaustHandle handle, float* buffer, f_size_t length) {
    if (!handle || !buffer) return;
    // Processamento dummy Faust: aplica ganho baixo para comprovar
    for(f_size_t i = 0; i < length; ++i) {
        // buffer[i] *= 0.99f;
    }
}

void faust_destroy(FaustHandle handle) {
    if(!handle) return;
    int* instance = (int*)handle;
    delete instance;
}

}
