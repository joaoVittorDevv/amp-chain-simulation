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
clip_type = nentry("Clip Type", 0, 0, 10, 1);

// --- Selectable clipping / saturation curves ---
// sign helper (avoid signum which may be unavailable)
sign_of(x) = ba.if(x > 0, 1.0, ba.if(x < 0, -1.0, 0.0));

// 0  Tanh (default)      - smooth, creamy tube-like compression
clip_tanh(x)  = ma.tanh(x);
// 1  Hard Clip           - abrupt transistor-fuzz clip
clip_hard(x)  = min(max(x, -0.5), 0.5);
// 2  Soft Sine           - very soft, edge of breakup
clip_sine(x)  = sin(max(0.0 - ma.PI / 2.0, min(ma.PI / 2.0, x)));
// 3  ArcTan              - more open than tanh
clip_atan(x)  = 2.0 / ma.PI * atan(ma.PI / 2.0 * x);
// 4  Algebraic           - creamy mids, less fizz
clip_alg(x)   = x / sqrt(1.0 + x * x);
// 5  Rational            - aggressive, germanium fuzz
clip_rat(x)   = x / (1.0 + abs(x));
// 6  Exponential         - very aggressive
clip_exp(x)   = (1.0 - exp(0.0 - abs(x))) * sign_of(x);
// 7  Cubic               - clean boost with slight saturation
clip_cubic(x) = min(max(x - x * x * x / 3.0, -0.667), 0.667);
// 8  Asymmetric Tanh     - DC bias, even harmonics
clip_atanh(x) = ma.tanh(x + 0.25) - ma.tanh(0.25);
// 9  Wave Fold           - metallic / djent wavefolding
clip_fold(x)  = x - 2.0 * max(0.0, abs(x) - 0.6) * sign_of(x);
// 10 Asymmetric Hard     - hard clip, different thresholds
clip_ahard(x) = min(max(x, -0.6), 0.35);

clip(x) = ba.selectn(11, int(clip_type),
    clip_tanh(x),
    clip_hard(x),
    clip_sine(x),
    clip_atan(x),
    clip_alg(x),
    clip_rat(x),
    clip_exp(x),
    clip_cubic(x),
    clip_atanh(x),
    clip_fold(x),
    clip_ahard(x));

gate_thresh = ba.db2linear(gate_thresh_db);
gate_env(x) = x : abs : max ~ *(0.995);
gate_gain(x) = gate_env(x) : >(gate_thresh) : si.smoo;
gate_stage(x) = x * gate_gain(x);

bright_gain = 1.5 + bright * 1.2;
m45_trim = 1.0 - (m45 * 0.35);
drive = 8.0 + gain * 72.0;

stage1 = *(drive * 0.22 * bright_gain * m45_trim) : clip : *(0.78);
stage2 = *(drive * 0.34 * m45_trim) : +(0.03) : clip : *(0.68);
stage3 = *(drive * 0.46) : clip : *(0.62);
gain_stages = stage1 : stage2 : stage3;

tone_stack =
    filters.low_shelf(bass, 120.0, 0.707)
    : filters.peak_eq(middle - 2.5, 760.0, 0.85)
    : filters.high_shelf(treble, 3400.0, 0.707);

warclaw_stage = (*(1.0 + warclaw * 1.9) : filters.peak_eq(warclaw * 4.0, 950.0, 1.2) : clip) : *(1.0 - warclaw * 0.22);

feedback_tight = 0.75 + feedback * 0.25;
power_amp =
    filters.low_shelf(depth * (1.25 - feedback * 0.35), 95.0, 0.8)
    : filters.high_shelf(presence * feedback_tight, 3600.0, 0.7);

pre_gate_path = gate_stage : *(gain) : gain_stages : tone_stack : warclaw_stage;
post_gate_path = *(gain) : gain_stages : tone_stack : warclaw_stage : gate_stage;
amp_core = _ <: pre_gate_path, post_gate_path : select2(gate_pos) : power_amp : *(master);

process = amp_core, amp_core;
