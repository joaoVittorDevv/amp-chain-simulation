import("stdfaust.lib");
filters = library("filters.lib");

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

// Per-stage clip dispatch — the integer selector picks the curve for that stage.
clip_sel(ct, x) = ba.selectn(3, int(ct),
    clip_atanh(x),
    clip_exp(x),
    clip_cheby(x));

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

gain_stages = pre_shape : stage1 : tight_hpf : interstage : stage2 : interstage : stage3;

tone_stack =
    filters.low_shelf(bass, 120.0, 0.707)
    : filters.peak_eq(middle - 2.5, 760.0, 0.85)
    : filters.high_shelf(treble, 3400.0, 0.707);

// Power-amp sag (item 6): a slow energy envelope (~10 Hz) drives a downward gain
// in the [0.3, 1.0] range so loud passages compress like a sagging supply rail.
sag_env(x) = x * x : filters.lp(10.0, 0.707);
sag(x) = x * (1.0 - sag_amount * 0.7 * min(1.0, sag_env(x)));

warclaw_stage = (*(1.0 + warclaw * 1.9) : filters.peak_eq(warclaw * 4.0, 950.0, 1.2) : clip_sel(clip_type3)) : *(1.0 - warclaw * 0.22);

feedback_tight = 0.75 + feedback * 0.25;
power_amp =
    filters.low_shelf(depth * (1.25 - feedback * 0.35), 95.0, 0.8)
    : filters.high_shelf(presence * feedback_tight, 3600.0, 0.7);

// Sag sits right after the tone stack, before WARCLAW and the power amp.
pre_gate_path = gate_stage : *(gain) : gain_stages : tone_stack : sag : warclaw_stage;
post_gate_path = *(gain) : gain_stages : tone_stack : sag : warclaw_stage : gate_stage;

wet_core = _ <: pre_gate_path, post_gate_path : select2(gate_pos) : power_amp;

// Dry (clean) path with a gentle lowpass so the parallel blend adds body and
// low-end punch without re-introducing fizz (item 4).
dry_core = filters.lp(8000.0, 0.707);

// Parallel clean blend: wet * (1 - blend) + dry * blend, then master volume.
amp_core = _ <: (wet_core : *(1.0 - clean_blend)), (dry_core : *(clean_blend)) :> *(master);

process = amp_core, amp_core;
