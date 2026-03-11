#include "wrapper.h"

// Faust Interfaces (minimal mock for the generated hpp)
#ifndef FAUSTFLOAT
#define FAUSTFLOAT float
#endif

struct Meta {
    virtual void declare(const char* key, const char* value) = 0;
};

struct UI {
    virtual ~UI() {}
    virtual void openTabBox(const char* label) = 0;
    virtual void openHorizontalBox(const char* label) = 0;
    virtual void openVerticalBox(const char* label) = 0;
    virtual void closeBox() = 0;
    virtual void addButton(const char* label, float* zone) = 0;
    virtual void addCheckButton(const char* label, float* zone) = 0;
    virtual void addVerticalBargraph(const char* label, float* zone, float min, float max) = 0;
    virtual void addHorizontalBargraph(const char* label, float* zone, float min, float max) = 0;
    virtual void addNumEntry(const char* label, float* zone, float init, float min, float max, float step) = 0;
    virtual void addHorizontalSlider(const char* label, float* zone, float init, float min, float max, float step) = 0;
    virtual void addVerticalSlider(const char* label, float* zone, float init, float min, float max, float step) = 0;
    virtual void declare(float* zone, const char* key, const char* val) = 0;
};

struct dsp {
    virtual ~dsp() {}
    virtual int getNumInputs() = 0;
    virtual int getNumOutputs() = 0;
    virtual void buildUserInterface(UI* ui_interface) = 0;
    virtual int getSampleRate() = 0;
    virtual void init(int sample_rate) = 0;
    virtual void instanceInit(int sample_rate) = 0;
    virtual void instanceConstants(int sample_rate) = 0;
    virtual void instanceResetUserInterface() = 0;
    virtual void instanceClear() = 0;
    virtual dsp* clone() = 0;
    virtual void metadata(Meta* m) = 0;
    virtual void compute(int count, FAUSTFLOAT** inputs, FAUSTFLOAT** outputs) = 0;
};

#include "FaustModule.hpp" // A classe dinâmica gerada pelo Faust (-lang cpp)
#include <string>
#include <map>

// Override focado 100% na captura in-memory para o FFI Rust em tempo real
class ParamMapUI : public UI {
public:
    std::map<std::string, float*> params;

    void addHorizontalSlider(const char* label, float* zone, float /*init*/, float /*min*/, float /*max*/, float /*step*/) override { params[label] = zone; }
    void addVerticalSlider(const char* label, float* zone, float /*init*/, float /*min*/, float /*max*/, float /*step*/) override { params[label] = zone; }
    void addNumEntry(const char* label, float* zone, float /*init*/, float /*min*/, float /*max*/, float /*step*/) override { params[label] = zone; }
    
    // Virtual voids estruturais sendo ignorados (não nos interessa desenhar UI via Faust)
    void openTabBox(const char* /*label*/) override {}
    void openHorizontalBox(const char* /*label*/) override {}
    void openVerticalBox(const char* /*label*/) override {}
    void closeBox() override {}
    void addButton(const char* /*label*/, float* /*zone*/) override {}
    void addCheckButton(const char* /*label*/, float* /*zone*/) override {}
    void addVerticalBargraph(const char* /*label*/, float* /*zone*/, float /*min*/, float /*max*/) override {}
    void addHorizontalBargraph(const char* /*label*/, float* /*zone*/, float /*min*/, float /*max*/) override {}
    void declare(float* /*zone*/, const char* /*key*/, const char* /*val*/) override {}
};

struct FaustInstance {
    mydsp* dsp;          // mydsp é instanciado em FaustModule.hpp
    ParamMapUI* ui;
};

extern "C" {

FaustHandle faust_create() {
    FaustInstance* inst = new FaustInstance();
    inst->dsp = new mydsp();
    inst->ui = new ParamMapUI();
    inst->dsp->buildUserInterface(inst->ui); // Captura a referência nativa dos paramétros
    return (FaustHandle)inst;
}

void faust_init(FaustHandle handle, float sample_rate) {
    if (!handle) return;
    ((FaustInstance*)handle)->dsp->init(sample_rate);
}

void faust_process(FaustHandle handle, float* buffer, f_size_t length) {
    if (!handle || !buffer) return;
    FaustInstance* inst = (FaustInstance*)handle;
    
    // Em Faust mono-to-stereo ou stereo, passamos o mesmo buffer para In/Out
    // Se a dsp foi feita 1-in, basta passar buffer (simulando mono)
    // Se ela for 2-in 2-out (como o EQ series que fizemos com `_,_`)
    // Passamos como pseudo-stereo.
    FAUSTFLOAT* in_channels[2] = { buffer, buffer };
    FAUSTFLOAT* out_channels[2] = { buffer, buffer };
    inst->dsp->compute(length, in_channels, out_channels);
}

void faust_destroy(FaustHandle handle) {
    if (!handle) return;
    FaustInstance* inst = (FaustInstance*)handle;
    delete inst->ui;
    delete inst->dsp;
    delete inst;
}

// Otimização Zero-Lookup runtime FFI
#define SET_PARAM(LABEL, VAL) \
    if (handle) { \
        auto& p = ((FaustInstance*)handle)->ui->params; \
        if (p.count(LABEL)) *p[LABEL] = VAL; \
    }

void faust_set_eq_low_freq(FaustHandle handle, float freq) { SET_PARAM("EQ Low Freq", freq); }
void faust_set_eq_low_gain(FaustHandle handle, float gain) { SET_PARAM("EQ Low Gain", gain); }
void faust_set_eq_low_q(FaustHandle handle, float q)       { SET_PARAM("EQ Low Q", q); }

void faust_set_eq_mid_freq(FaustHandle handle, float freq) { SET_PARAM("EQ Mid Freq", freq); }
void faust_set_eq_mid_gain(FaustHandle handle, float gain) { SET_PARAM("EQ Mid Gain", gain); }
void faust_set_eq_mid_q(FaustHandle handle, float q)       { SET_PARAM("EQ Mid Q", q); }

void faust_set_eq_high_freq(FaustHandle handle, float freq){ SET_PARAM("EQ High Freq", freq); }
void faust_set_eq_high_gain(FaustHandle handle, float gain){ SET_PARAM("EQ High Gain", gain); }
void faust_set_eq_high_q(FaustHandle handle, float q)      { SET_PARAM("EQ High Q", q); }

}
