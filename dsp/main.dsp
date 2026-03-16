import("stdfaust.lib");
diff = library("diff.lib");
filters = library("filters.lib");

// --- Equalizador Paramétrico de 3 Bandas Estável ---
// Utilizamos SVF (State Variable Filter) da biblioteca faust-ddsp/filters.lib

// Banda 1 (Low Shelf)
f1 = hslider("EQ Low Freq [unit:Hz]", 100, 20, 1000, 1) : si.smoo;
g1 = hslider("EQ Low Gain [unit:dB]", 0, -12, 12, 0.1) : si.smoo;
q1 = hslider("EQ Low Q", 0.707, 0.707, 10, 0.01) : si.smoo;
eq_low = filters.low_shelf(g1, f1, q1);

// Banda 2 (Mid Peak)
f2 = hslider("EQ Mid Freq [unit:Hz]", 1000, 100, 10000, 1) : si.smoo;
g2 = hslider("EQ Mid Gain [unit:dB]", 0, -12, 12, 0.1) : si.smoo;
q2 = hslider("EQ Mid Q", 0.707, 0.707, 10, 0.01) : si.smoo;
eq_mid = filters.peak_eq(g2, f2, q2);

// Banda 3 (High Shelf)
f3 = hslider("EQ High Freq [unit:Hz]", 5000, 1000, 20000, 1) : si.smoo;
g3 = hslider("EQ High Gain [unit:dB]", 0, -12, 12, 0.1) : si.smoo;
q3 = hslider("EQ High Q", 0.707, 0.707, 10, 0.01) : si.smoo;
eq_high = filters.high_shelf(g3, f3, q3);

// DSP Pipeline Estável (Cascata) com Soft Clipping (ma.tanh)
eq_chain = eq_low : eq_mid : eq_high : ma.tanh;

// Orquestração: Estéreo in/out
process = _,_ : (eq_chain, eq_chain);