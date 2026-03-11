import("stdfaust.lib");

// --- Equalizador Paramétrico de 3 Bandas ---
// Cada banda usa a estrutura de Peak EQ, configurada com: fi.peak_eq_cq(gain_dB, freq, Q)
// Aplicamos si.smoo globalmente para evitar o "Zipper Noise" em tempo-real.

// Banda 1 (Low)
f1 = hslider("EQ Low Freq [unit:Hz]", 100, 20, 1000, 1) : si.smoo;
g1 = hslider("EQ Low Gain [unit:dB]", 0, -24, 24, 0.1) : si.smoo;
q1 = hslider("EQ Low Q", 1, 0.1, 10, 0.01) : si.smoo;
eq_low = fi.peak_eq_cq(g1, f1, q1);

// Banda 2 (Mid)
f2 = hslider("EQ Mid Freq [unit:Hz]", 1000, 100, 10000, 1) : si.smoo;
g2 = hslider("EQ Mid Gain [unit:dB]", 0, -24, 24, 0.1) : si.smoo;
q2 = hslider("EQ Mid Q", 1, 0.1, 10, 0.01) : si.smoo;
eq_mid = fi.peak_eq_cq(g2, f2, q2);

// Banda 3 (High)
f3 = hslider("EQ High Freq [unit:Hz]", 5000, 1000, 20000, 1) : si.smoo;
g3 = hslider("EQ High Gain [unit:dB]", 0, -24, 24, 0.1) : si.smoo;
q3 = hslider("EQ High Q", 1, 0.1, 10, 0.01) : si.smoo;
eq_high = fi.peak_eq_cq(g3, f3, q3);

// DSP Pipeline Customizável (Cascata)
eq_chain = eq_low : eq_mid : eq_high;

// Orquestração: Estéreo in/out (processando eq_chain paralelamente por canal)
process = _,_ : (eq_chain, eq_chain) : _,_;