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
clip_type = nentry("Clip Type", 0, 0, 1, 1);

// --- Selectable clipping / saturation curves ---
// 0  Asymmetric Tanh     - DC bias, even harmonics (default)
clip_atanh(x) = ma.tanh(x + 0.25) - ma.tanh(0.25);
// 1  Exponential         - very aggressive (RAT/DS-1 voicing)
clip_exp(x)   = (1.0 - exp(0.0 - abs(x))) * ba.if(x > 0, 1.0, ba.if(x < 0, -1.0, 0.0));

clip(x) = ba.selectn(2, int(clip_type),
    clip_atanh(x),
    clip_exp(x));

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
