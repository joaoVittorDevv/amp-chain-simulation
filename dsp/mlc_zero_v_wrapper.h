#ifndef MLC_ZERO_V_WRAPPER_H
#define MLC_ZERO_V_WRAPPER_H

// MSVC's cl.exe does not define __SIZE_TYPE__ (a GCC/Clang builtin), so it
// must fall back to <stddef.h>. GCC/Clang keep __SIZE_TYPE__ since it avoids
// a header include entirely (bindgen/libclang have been unreliable finding
// stddef.h without a complete sysroot — see CROSS-05).
#ifdef _MSC_VER
#include <stddef.h>
typedef size_t f_size_t;
#else
typedef __SIZE_TYPE__ f_size_t;
#endif

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

/* Tier 2.2 / 3.x additions */
void mlc_zero_v_set_ts_model(FaustHandle handle, float value);
void mlc_zero_v_set_tube_model(FaustHandle handle, float value);
void mlc_zero_v_set_tube_drive(FaustHandle handle, float value);
void mlc_zero_v_set_tube_bypass(FaustHandle handle, float value);
void mlc_zero_v_set_nfb_presence(FaustHandle handle, float value);
void mlc_zero_v_set_nfb_resonance(FaustHandle handle, float value);
void mlc_zero_v_set_nfb_depth(FaustHandle handle, float value);
void mlc_zero_v_set_nfb_bypass(FaustHandle handle, float value);
void mlc_zero_v_set_mbc_bypass(FaustHandle handle, float value);
void mlc_zero_v_set_mbc_cf_lo(FaustHandle handle, float value);
void mlc_zero_v_set_mbc_cf_hi(FaustHandle handle, float value);
void mlc_zero_v_set_mbc_drive_lo(FaustHandle handle, float value);
void mlc_zero_v_set_mbc_drive_mid(FaustHandle handle, float value);
void mlc_zero_v_set_mbc_drive_hi(FaustHandle handle, float value);
void mlc_zero_v_set_adaa_order(FaustHandle handle, float value);

#ifdef __cplusplus
}
#endif

#endif
