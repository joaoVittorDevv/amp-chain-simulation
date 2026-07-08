import("stdfaust.lib");
filters = library("svf.lib");    // project-local stable SVF filters (faust-ddsp/svf.lib)
ts = library("tonestacks.lib");   // Tier 2.2 — real passive tone-stack models
tb = library("tubes.lib");        // Tier 3.2 — LUT tube waveshaping
aa = library("aanl.lib");         // Tier 3.6 — ADAA anti-aliased saturators

gain = hslider("Gain", 0.25118864, 0.001, 1.0, 0.0001) : si.smoo;
master = hslider("Master", 0.5011872, 0.001, 1.0, 0.0001) : si.smoo;
bass = hslider("Bass [unit:dB]", 0.0, -12.0, 12.0, 0.1) : si.smoo;
middle = hslider("Middle [unit:dB]", 0.0, -12.0, 12.0, 0.1) : si.smoo;
treble = hslider("Treble [unit:dB]", 0.0, -12.0, 12.0, 0.1) : si.smoo;
presence = hslider("Presence [unit:dB]", 0.0, -12.0, 12.0, 0.1) : si.smoo;
depth = hslider("Depth [unit:dB]", 0.0, -12.0, 12.0, 0.1) : si.smoo;
gate_thresh_db = hslider("Gate [unit:dB]", -80.0, -80.0, 0.0, 0.1) : si.smoo;

bright = nentry("Bright", 1.0, 0.0, 1.0, 1.0);
m45 = nentry("M45", 0.0, 0.0, 1.0, 1.0);
warclaw = nentry("WARCLAW", 0.0, 0.0, 1.0, 1.0);
feedback = nentry("Feedback", 1.0, 0.0, 1.0, 1.0);
gate_pos = nentry("Gate Pos", 0.0, 0.0, 1.0, 1.0);

// --- Per-stage clip type (0 = Asymmetric Tanh, 1 = Exponential, 2 = Chebyshev) ---
// Stage 3 defaults to Exponential for a more aggressive final stage.
clip_type1 = nentry("Clip Type 1", 0, 0, 2, 1);
clip_type2 = nentry("Clip Type 2", 0, 0, 2, 1);
clip_type3 = nentry("Clip Type 3", 1, 0, 2, 1);

// --- Tier 1 gain-staging controls ---
tight = nentry("Tight", 1, 0, 1, 1);
asymmetry_enable = nentry("Asymmetry Enable", 1, 0, 1, 1);
asymmetry = nentry("Asymmetry", 0.5, 0, 1, 0.01);
preshape = nentry("Pre-Shape", 0, 0, 1, 1);
preshape_tight = nentry("Pre-Shape Tight", -3, -6, 0, 0.1);
preshape_bite = nentry("Pre-Shape Bite", 3, 0, 6, 0.1);

// --- Clean blend, power-amp sag and Chebyshev harmonic mix ---
clean_blend = hslider("Clean Blend", 0.0, 0.0, 0.25, 0.01) : si.smoo;
sag_amount = hslider("Sag", 0.0, 0.0, 1.0, 0.01) : si.smoo;
h2 = hslider("H2", 0.0, 0.0, 1.0, 0.01) : si.smoo;
h3 = hslider("H3", 0.7, 0.0, 1.0, 0.01) : si.smoo;
h4 = hslider("H4", 0.2, 0.0, 1.0, 0.01) : si.smoo;

// --- Tier 2.2  Tone-Stack model selector (0..24) ---
tonestack_model = nentry("Tone Stack Model", 0, 0, 24, 1);

// --- Tier 3.2  Tube waveshaping (18 models = 6 tubes x 3 stages) ---
tube_model  = nentry("Tube Model", 0, 0, 17, 1);
tube_drive  = hslider("Tube Drive [unit:dB]", 0.0, -20.0, 20.0, 0.1) : ba.db2linear : si.smoo;
tube_bypass = checkbox("Tube Bypass");

// --- Tier 3.4  Power-amp negative-feedback loop ---
nfb_presence  = hslider("NFB Presence", 0.0, 0.0, 1.0, 0.01) : si.smoo;
nfb_resonance = hslider("NFB Resonance", 0.0, 0.0, 1.0, 0.01) : si.smoo;
nfb_depth     = hslider("NFB Depth", 0.7, 0.0, 1.0, 0.01) : si.smoo;
nfb_bypass    = checkbox("NFB Bypass");

// --- Tier 3.5  Multi-band clipping ---
mbc_bypass    = checkbox("Multi-Band Bypass");
mbc_cf_lo     = hslider("XOver Low", 300.0, 100.0, 800.0, 1.0) : si.smoo;
mbc_cf_hi     = hslider("XOver High", 3000.0, 1500.0, 6000.0, 1.0) : si.smoo;
mbc_drive_lo  = hslider("Drive Lo", 1.0, 0.1, 4.0, 0.01) : si.smoo;
mbc_drive_mid = hslider("Drive Mid", 1.0, 0.1, 4.0, 0.01) : si.smoo;
mbc_drive_hi  = hslider("Drive Hi", 1.0, 0.1, 4.0, 0.01) : si.smoo;

// --- Tier 3.6  ADAA order (0 = Off, 1 = ADAA1, 2 = ADAA2) ---
adaa_order = nentry("ADAA Order", 0, 0, 2, 1);

// --- Selectable clipping / saturation curves ---
// 0  Asymmetric Tanh     - DC bias, even harmonics (default)
//    Asymmetry slider drives the DC bias (0 = symmetric tanh, 1 = strong even harmonics).
clip_atanh(x) = ba.if(asymmetry_enable > 0.5,
    ma.tanh(x + asymmetry * 0.5) - ma.tanh(asymmetry * 0.5),
    ma.tanh(x));
// 1  Exponential         - very aggressive (RAT/DS-1 voicing)
clip_exp(x)   = (1.0 - exp(0.0 - abs(x))) * ba.if(x > 0, 1.0, ba.if(x < 0, -1.0, 0.0));
// 2  Chebyshev           - explicit even/odd harmonic generator (item 7)
//    T2 = 2x^2 - 1 (2nd, even), T3 = 4x^3 - 3x (3rd, odd), T4 = 8x^4 - 8x^2 + 1 (4th, even).
cheby_t2(x) = 2.0 * x * x - 1.0;
cheby_t3(x) = 4.0 * x * x * x - 3.0 * x;
cheby_t4(x) = 8.0 * x * x * x * x - 8.0 * x * x + 1.0;
clip_cheby_core(x) = x * (1.0 - h2 - h3 - h4)
    + cheby_t2(x) * h2 * 0.5
    + cheby_t3(x) * h3 * 0.33
    + cheby_t4(x) * h4 * 0.25;
// The raw Chebyshev polynomial is unbounded (T4 grows as 8x^4), so a large input
// or several Chebyshev stages in series can push a value beyond the f32 range
// before it reaches the limiter. A final tanh soft-clamp keeps the output within
// ±1.0 for any input while leaving the low-level harmonic content unchanged.
clip_cheby(x) = ma.tanh(clip_cheby_core(x));

// ADAA (Tier 3.6): aanl.lib supplies bandlimited saturators whose analytic
// antiderivatives are built in, so we cannot ADAA-wrap our custom curves directly
// (they have no closed-form antiderivative). Instead each base curve maps to the
// closest anti-aliased saturator. All chosen saturators have unity small-signal
// slope at the origin, so the low-level gain is unchanged vs. the non-ADAA path
// (max gain per curve ≈ ±1.0 ceiling). ADAA1 adds 0.5-sample, ADAA2 ~1-sample delay.
//   base 0 (tanh-ish)  -> aa.hyperbolic  (x / sqrt(1+x^2))
//   base 1 (aggressive)-> aa.hardclip    (clip(-1,1,x))
//   base 2 (chebyshev) -> aa.arctan
clip0_sel(x) = ba.selectn(3, int(adaa_order), clip_atanh(x), aa.hyperbolic(x), aa.hyperbolic2(x));
clip1_sel(x) = ba.selectn(3, int(adaa_order), clip_exp(x),   aa.hardclip(x),   aa.hardclip2(x));
clip2_sel(x) = ba.selectn(3, int(adaa_order), clip_cheby(x), aa.arctan(x),     aa.arctan2(x));

// Per-stage clip dispatch — the integer selector picks the curve for that stage,
// then the ADAA order (above) picks off/anti-aliased for that curve.
clip_sel(ct, x) = ba.selectn(3, int(ct),
    clip0_sel(x),
    clip1_sel(x),
    clip2_sel(x));

gate_thresh = ba.db2linear(gate_thresh_db);
gate_env(x) = x : abs : max ~ *(0.995);
gate_gain(x) = gate_env(x) : >(gate_thresh) : si.smoo;
gate_stage(x) = x * gate_gain(x);

// Bright cap is progressively bypassed as the gain pot is raised (item 2):
// at low gain the bright boost is full, at max gain it is reduced by 70%.
bright_gain_base = 1.5 + bright * 1.2;
bright_gain_eff = bright_gain_base * (1.0 - gain * 0.7);
m45_trim = 1.0 - (m45 * 0.35);
drive = 8.0 + gain * 72.0;

stage1 = *(drive * 0.22 * bright_gain_eff * m45_trim) : clip_sel(clip_type1) : *(0.78);
stage2 = *(drive * 0.34 * m45_trim) : +(0.03) : clip_sel(clip_type2) : *(0.68);
stage3 = *(drive * 0.46) : clip_sel(clip_type3) : *(0.62);

// Pre-Shape EQ before the gain stages. When preshape = 0 both gains collapse to
// 0 dB so the block is a no-op; preshape = 1 applies the configured cut/boost.
pre_shape(x) = x
    : filters.low_shelf(preshape * preshape_tight, 150, 0.7)
    : filters.peak_eq(preshape * preshape_bite, 1000, 0.8);

// Tight HPF (80 Hz) inserted between stage1 and stage2 to keep the low end
// firm before the second round of gain. Bypassed when tight <= 0.5.
tight_hpf(x) = ba.if(tight > 0.5, x : filters.hp(80, 0.707), x);

// Inter-stage band limiting (item 3): a gentle HPF removes sub-bass mud and a
// LPF removes fizz above 12 kHz, curbing IMD build-up between stages.
interstage(x) = x : filters.hp(80.0, 0.707) : filters.lp(12000.0, 0.707);

// Original cascade of the three drive/clip stages.
cascade = stage1 : tight_hpf : interstage : stage2 : interstage : stage3;

// Tier 3.5 — Multi-band clipping. Perfect-reconstruction subtractive 3-band split
// (lo + mid + hi == input exactly, so the split itself is spectrally flat), each
// band driven, clipped and level-compensated independently, then summed. The
// pre-drive matches the cascade energy; the per-band 1/max(1,drive) makeup keeps
// each band near the clip ceiling. Worst-case band gain ≈ 0.6 before summation.
mband(x) = bl + bm + bh
with {
    pre  = x : *(drive * 0.30);          // into the bands; max ≈ 80 * 0.30 = 24
    lo   = pre : filters.lp(mbc_cf_lo, 0.707);
    rest = pre - lo;
    mid  = rest : filters.lp(mbc_cf_hi, 0.707);
    hi   = rest - mid;
    bl = lo  : *(mbc_drive_lo)  : clip_sel(clip_type1) : *(0.6 / max(1.0, mbc_drive_lo));
    bm = mid : *(mbc_drive_mid) : clip_sel(clip_type2) : *(0.6 / max(1.0, mbc_drive_mid));
    bh = hi  : *(mbc_drive_hi)  : clip_sel(clip_type3) : *(0.6 / max(1.0, mbc_drive_hi));
};

// Cascade vs. multi-band clipping. Multi-band is bypassed by default (host pushes
// mbc_bypass = 1), so the default voicing is the original cascade unchanged.
clip_section(x) = ba.if(mbc_bypass > 0.5, x : cascade, mband(x));

// Tier 3.2 — Tube waveshaping stage, inserted AFTER the clip cascade and BEFORE
// the tone stack. Measured intrinsic gain of the LUT tube stages ≈ +1.5..+3.5 dB
// (×1.2..1.5), so tube_out_trim ≈ 0.7 restores ~unity at tube_drive = 0 dB.
// Worst-case gain ≈ 1.5 × tube_drive(max 10×) × 0.7 ≈ 10.5 before the limiter.
tube_out_trim = 0.7;
tube_stage(x) = ba.selectn(18, int(tube_model),
    x : tb.T1_12AX7, x : tb.T2_12AX7, x : tb.T3_12AX7,   // 0-2
    x : tb.T1_12AT7, x : tb.T2_12AT7, x : tb.T3_12AT7,   // 3-5
    x : tb.T1_12AU7, x : tb.T2_12AU7, x : tb.T3_12AU7,   // 6-8
    x : tb.T1_6V6,   x : tb.T2_6V6,   x : tb.T3_6V6,     // 9-11
    x : tb.T1_6DJ8,  x : tb.T2_6DJ8,  x : tb.T3_6DJ8,    // 12-14
    x : tb.T1_6C16,  x : tb.T2_6C16,  x : tb.T3_6C16)    // 15-17
    : *(tube_drive * tube_out_trim);
// Bypassed by default (host pushes tube_bypass = 1).
tube_block(x) = ba.if(tube_bypass > 0.5, x, tube_stage(x));

gain_stages = pre_shape : clip_section : tube_block;

// Tier 2.2 — Tone Stack (real passive tone-stack circuits). bass/mid/treble are in
// dB (-12..+12); the library expects normalized 0..1 knobs, so 0 dB (flat) maps to
// 0.5. Passive tone stacks have significant insertion loss (measured -3..-21 dB
// across the 25 models/bands; typical passband ≈ -4..-8 dB), so ts_makeup ≈ ×2.24
// (+7 dB) restores approximate unity without over-driving the loudest models.
// Worst-case makeup'd gain: -3 dB insertion + 7 dB makeup = +4 dB (e.g. Gibson),
// caught by the output limiter.
ts_makeup = 2.24;
ts_norm(v) = (v + 12.0) / 24.0;
select_ts(t, m, b, x) = ba.selectn(25, int(tonestack_model),
    x : ts.bassman(t,m,b),        // 0
    x : ts.mesa(t,m,b),           // 1
    x : ts.twin(t,m,b),           // 2
    x : ts.princeton(t,m,b),      // 3
    x : ts.fender_blues(t,m,b),   // 4
    x : ts.fender_default(t,m,b), // 5
    x : ts.fender_deville(t,m,b), // 6
    x : ts.jcm800(t,m,b),         // 7
    x : ts.jcm2000(t,m,b),        // 8
    x : ts.jtm45(t,m,b),          // 9
    x : ts.mlead(t,m,b),          // 10
    x : ts.m2199(t,m,b),          // 11
    x : ts.ac30(t,m,b),           // 12
    x : ts.ac15(t,m,b),           // 13
    x : ts.soldano(t,m,b),        // 14
    x : ts.sovtek(t,m,b),         // 15
    x : ts.peavey(t,m,b),         // 16
    x : ts.ibanez(t,m,b),         // 17
    x : ts.roland(t,m,b),         // 18
    x : ts.ampeg(t,m,b),          // 19
    x : ts.ampeg_rev(t,m,b),      // 20
    x : ts.bogner(t,m,b),         // 21
    x : ts.groove(t,m,b),         // 22
    x : ts.crunch(t,m,b),         // 23
    x : ts.gibsen(t,m,b));        // 24
tone_stack(x) = select_ts(ts_norm(treble), ts_norm(middle), ts_norm(bass), x) : *(ts_makeup);

// Power-amp sag (item 6): a slow energy envelope (~10 Hz) drives a downward gain
// in the [0.3, 1.0] range so loud passages compress like a sagging supply rail.
sag_env(x) = x * x : filters.lp(10.0, 0.707);
sag(x) = x * (1.0 - sag_amount * 0.7 * min(1.0, sag_env(x)));

warclaw_stage = (*(1.0 + warclaw * 1.9) : filters.peak_eq(warclaw * 4.0, 950.0, 1.2) : clip_sel(clip_type3)) : *(1.0 - warclaw * 0.22);

feedback_tight = 0.75 + feedback * 0.25;
power_amp =
    filters.low_shelf(depth * (1.25 - feedback * 0.35), 95.0, 0.8)
    : filters.high_shelf(presence * feedback_tight, 3600.0, 0.7);

// Tier 3.4 — Power-amp negative feedback loop (real feedback via the ~ operator).
// Presence reduces feedback at highs (→ boosts output highs); Resonance reduces
// feedback at lows (→ boosts output lows). At neutral (presence=resonance=0) the
// feedback shelves are flat and nfb_makeup = 1 + loop_gain restores approximate
// unity small-signal gain for any depth (net 0.99×, trivially ~unity), so the loop
// only ever attenuates (max gain ≈ 1.0).
// Bypassed by default (host pushes nfb_bypass = 1).
nfb_poweramp(x) = ba.if(nfb_bypass > 0.5, x, loop(x) * nfb_makeup)
with {
    power_tube = *(1.8) : ma.tanh : *(0.55);                 // small-signal gain ≈ 0.99
    fb_res_filter  = filters.low_shelf(0.0 - nfb_resonance * 9.0, 90.0, 0.8);
    fb_pres_filter = filters.high_shelf(0.0 - nfb_presence * 9.0, 3500.0, 0.8);
    fb_path = *(0.0 - nfb_depth * 0.5) : filters.lp(18000.0, 0.707) : fb_res_filter : fb_pres_filter;
    loop = (+ : power_tube) ~ fb_path;
    nfb_makeup = 1.0 + nfb_depth * 0.5 * 0.99;               // restore unity small-signal gain
};

// Sag sits right after the tone stack, before WARCLAW and the power amp.
pre_gate_path = gate_stage : *(gain) : gain_stages : tone_stack : sag : warclaw_stage;
post_gate_path = *(gain) : gain_stages : tone_stack : sag : warclaw_stage : gate_stage;

// The NFB loop sits after the tone stack / EQ power-amp, before the master volume.
wet_core = _ <: pre_gate_path, post_gate_path : select2(gate_pos) : power_amp : nfb_poweramp;

// Dry (clean) path with a gentle lowpass so the parallel blend adds body and
// low-end punch without re-introducing fizz (item 4).
dry_core = filters.lp(8000.0, 0.707);

// Parallel clean blend: wet * (1 - blend) + dry * blend, then master volume.
amp_core = _ <: (wet_core : *(1.0 - clean_blend)), (dry_core : *(clean_blend)) :> *(master);

process = amp_core, amp_core;
