#ifndef MLC_ZERO_V_WRAPPER_H
#define MLC_ZERO_V_WRAPPER_H

typedef unsigned long f_size_t;

#ifdef __cplusplus
extern "C" {
#endif

typedef void* FaustHandle;

FaustHandle mlc_zero_v_create();
void mlc_zero_v_init(FaustHandle handle, float sample_rate);
void mlc_zero_v_process(FaustHandle handle, float* buffer, f_size_t length);
void mlc_zero_v_destroy(FaustHandle handle);

void mlc_zero_v_set_gain(FaustHandle handle, float value);
void mlc_zero_v_set_master(FaustHandle handle, float value);
void mlc_zero_v_set_bass(FaustHandle handle, float value);
void mlc_zero_v_set_middle(FaustHandle handle, float value);
void mlc_zero_v_set_treble(FaustHandle handle, float value);
void mlc_zero_v_set_presence(FaustHandle handle, float value);
void mlc_zero_v_set_depth(FaustHandle handle, float value);
void mlc_zero_v_set_gate(FaustHandle handle, float value);
void mlc_zero_v_set_bright(FaustHandle handle, float value);
void mlc_zero_v_set_m45(FaustHandle handle, float value);
void mlc_zero_v_set_warclaw(FaustHandle handle, float value);
void mlc_zero_v_set_feedback(FaustHandle handle, float value);
void mlc_zero_v_set_gate_pos(FaustHandle handle, float value);
void mlc_zero_v_set_clip_type1(FaustHandle handle, float value);
void mlc_zero_v_set_clip_type2(FaustHandle handle, float value);
void mlc_zero_v_set_clip_type3(FaustHandle handle, float value);
void mlc_zero_v_set_clean_blend(FaustHandle handle, float value);
void mlc_zero_v_set_sag(FaustHandle handle, float value);
void mlc_zero_v_set_h2(FaustHandle handle, float value);
void mlc_zero_v_set_h3(FaustHandle handle, float value);
void mlc_zero_v_set_h4(FaustHandle handle, float value);
void mlc_zero_v_set_tight(FaustHandle handle, float value);
void mlc_zero_v_set_asymmetry_enable(FaustHandle handle, float value);
void mlc_zero_v_set_asymmetry(FaustHandle handle, float value);
void mlc_zero_v_set_preshape(FaustHandle handle, float value);
void mlc_zero_v_set_preshape_tight(FaustHandle handle, float value);
void mlc_zero_v_set_preshape_bite(FaustHandle handle, float value);

#ifdef __cplusplus
}
#endif

#endif
