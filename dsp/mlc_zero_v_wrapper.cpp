#include "mlc_zero_v_wrapper.h"

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

#include "MlcZeroVModule.hpp"
#include <map>
#include <string>

class MlcParamMapUI : public UI {
public:
    std::map<std::string, float*> params;

    void addHorizontalSlider(const char* label, float* zone, float, float, float, float) override { params[label] = zone; }
    void addVerticalSlider(const char* label, float* zone, float, float, float, float) override { params[label] = zone; }
    void addNumEntry(const char* label, float* zone, float, float, float, float) override { params[label] = zone; }
    void addCheckButton(const char* label, float* zone) override { params[label] = zone; }
    void openTabBox(const char*) override {}
    void openHorizontalBox(const char*) override {}
    void openVerticalBox(const char*) override {}
    void closeBox() override {}
    void addButton(const char*, float*) override {}
    void addVerticalBargraph(const char*, float*, float, float) override {}
    void addHorizontalBargraph(const char*, float*, float, float) override {}
    void declare(float*, const char*, const char*) override {}
};

struct MlcZeroVInstance {
    mlczerov* dsp;
    MlcParamMapUI* ui;
};

extern "C" {

FaustHandle mlc_zero_v_create() {
    MlcZeroVInstance* inst = new MlcZeroVInstance();
    inst->dsp = new mlczerov();
    inst->ui = new MlcParamMapUI();
    inst->dsp->buildUserInterface(inst->ui);
    return (FaustHandle)inst;
}

void mlc_zero_v_init(FaustHandle handle, float sample_rate) {
    if (!handle) return;
    ((MlcZeroVInstance*)handle)->dsp->init(sample_rate);
}

void mlc_zero_v_process(FaustHandle handle, float* buffer, f_size_t length) {
    if (!handle || !buffer) return;
    MlcZeroVInstance* inst = (MlcZeroVInstance*)handle;
    FAUSTFLOAT* in_channels[2] = { buffer, buffer };
    FAUSTFLOAT* out_channels[2] = { buffer, buffer };
    inst->dsp->compute(length, in_channels, out_channels);
}

void mlc_zero_v_destroy(FaustHandle handle) {
    if (!handle) return;
    MlcZeroVInstance* inst = (MlcZeroVInstance*)handle;
    delete inst->ui;
    delete inst->dsp;
    delete inst;
}

#define MLC_SET_PARAM(LABEL, VAL) \
    if (handle) { \
        auto& p = ((MlcZeroVInstance*)handle)->ui->params; \
        if (p.count(LABEL)) *p[LABEL] = VAL; \
    }

void mlc_zero_v_set_gain(FaustHandle handle, float value) { MLC_SET_PARAM("Gain", value); }
void mlc_zero_v_set_master(FaustHandle handle, float value) { MLC_SET_PARAM("Master", value); }
void mlc_zero_v_set_bass(FaustHandle handle, float value) { MLC_SET_PARAM("Bass", value); }
void mlc_zero_v_set_middle(FaustHandle handle, float value) { MLC_SET_PARAM("Middle", value); }
void mlc_zero_v_set_treble(FaustHandle handle, float value) { MLC_SET_PARAM("Treble", value); }
void mlc_zero_v_set_presence(FaustHandle handle, float value) { MLC_SET_PARAM("Presence", value); }
void mlc_zero_v_set_depth(FaustHandle handle, float value) { MLC_SET_PARAM("Depth", value); }
void mlc_zero_v_set_gate(FaustHandle handle, float value) { MLC_SET_PARAM("Gate", value); }
void mlc_zero_v_set_bright(FaustHandle handle, float value) { MLC_SET_PARAM("Bright", value); }
void mlc_zero_v_set_m45(FaustHandle handle, float value) { MLC_SET_PARAM("M45", value); }
void mlc_zero_v_set_warclaw(FaustHandle handle, float value) { MLC_SET_PARAM("WARCLAW", value); }
void mlc_zero_v_set_feedback(FaustHandle handle, float value) { MLC_SET_PARAM("Feedback", value); }
void mlc_zero_v_set_gate_pos(FaustHandle handle, float value) { MLC_SET_PARAM("Gate Pos", value); }
void mlc_zero_v_set_clip_type1(FaustHandle handle, float value) { MLC_SET_PARAM("Clip Type 1", value); }
void mlc_zero_v_set_clip_type2(FaustHandle handle, float value) { MLC_SET_PARAM("Clip Type 2", value); }
void mlc_zero_v_set_clip_type3(FaustHandle handle, float value) { MLC_SET_PARAM("Clip Type 3", value); }
void mlc_zero_v_set_clean_blend(FaustHandle handle, float value) { MLC_SET_PARAM("Clean Blend", value); }
void mlc_zero_v_set_sag(FaustHandle handle, float value) { MLC_SET_PARAM("Sag", value); }
void mlc_zero_v_set_h2(FaustHandle handle, float value) { MLC_SET_PARAM("H2", value); }
void mlc_zero_v_set_h3(FaustHandle handle, float value) { MLC_SET_PARAM("H3", value); }
void mlc_zero_v_set_h4(FaustHandle handle, float value) { MLC_SET_PARAM("H4", value); }
void mlc_zero_v_set_tight(FaustHandle handle, float value) { MLC_SET_PARAM("Tight", value); }
void mlc_zero_v_set_asymmetry_enable(FaustHandle handle, float value) { MLC_SET_PARAM("Asymmetry Enable", value); }
void mlc_zero_v_set_asymmetry(FaustHandle handle, float value) { MLC_SET_PARAM("Asymmetry", value); }
void mlc_zero_v_set_preshape(FaustHandle handle, float value) { MLC_SET_PARAM("Pre-Shape", value); }
void mlc_zero_v_set_preshape_tight(FaustHandle handle, float value) { MLC_SET_PARAM("Pre-Shape Tight", value); }
void mlc_zero_v_set_preshape_bite(FaustHandle handle, float value) { MLC_SET_PARAM("Pre-Shape Bite", value); }

// Tier 2.2 / 3.x additions. Labels match the Faust hslider/nentry/checkbox names
// (Faust strips the [unit:dB] metadata from the label used as the map key).
void mlc_zero_v_set_ts_model(FaustHandle handle, float value) { MLC_SET_PARAM("Tone Stack Model", value); }
void mlc_zero_v_set_tube_model(FaustHandle handle, float value) { MLC_SET_PARAM("Tube Model", value); }
void mlc_zero_v_set_tube_drive(FaustHandle handle, float value) { MLC_SET_PARAM("Tube Drive", value); }
void mlc_zero_v_set_tube_bypass(FaustHandle handle, float value) { MLC_SET_PARAM("Tube Bypass", value); }
void mlc_zero_v_set_nfb_presence(FaustHandle handle, float value) { MLC_SET_PARAM("NFB Presence", value); }
void mlc_zero_v_set_nfb_resonance(FaustHandle handle, float value) { MLC_SET_PARAM("NFB Resonance", value); }
void mlc_zero_v_set_nfb_depth(FaustHandle handle, float value) { MLC_SET_PARAM("NFB Depth", value); }
void mlc_zero_v_set_nfb_bypass(FaustHandle handle, float value) { MLC_SET_PARAM("NFB Bypass", value); }
void mlc_zero_v_set_mbc_bypass(FaustHandle handle, float value) { MLC_SET_PARAM("Multi-Band Bypass", value); }
void mlc_zero_v_set_mbc_cf_lo(FaustHandle handle, float value) { MLC_SET_PARAM("XOver Low", value); }
void mlc_zero_v_set_mbc_cf_hi(FaustHandle handle, float value) { MLC_SET_PARAM("XOver High", value); }
void mlc_zero_v_set_mbc_drive_lo(FaustHandle handle, float value) { MLC_SET_PARAM("Drive Lo", value); }
void mlc_zero_v_set_mbc_drive_mid(FaustHandle handle, float value) { MLC_SET_PARAM("Drive Mid", value); }
void mlc_zero_v_set_mbc_drive_hi(FaustHandle handle, float value) { MLC_SET_PARAM("Drive Hi", value); }
void mlc_zero_v_set_adaa_order(FaustHandle handle, float value) { MLC_SET_PARAM("ADAA Order", value); }

}
