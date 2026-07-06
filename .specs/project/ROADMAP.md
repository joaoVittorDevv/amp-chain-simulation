# Roadmap

**Created:** 2026-07-06
**Last updated:** 2026-07-06

## Current Features

- [x] 7-stage DSP pipeline (Input → Faust EQ → Pre-EQ Convolver → Mojo Neural → Cabinet IR → Master → NaN Sanitizer)
- [x] VST3 + CLAP plugin exports
- [x] Standalone mode with CPAL audio routing
- [x] Faust 3-band parametric EQ (Low/Mid/High with freq, gain, Q)
- [x] Mojo neural drive (tanh saturation, zero-copy FFI)
- [x] Cabinet IR system (SQLite library, FFT convolution, import/export)
- [x] FFT spectrum analyzer (2048-point, Blackman-Harris window)
- [x] egui-based UI (shared between plugin and standalone)

## v0.2 — MLC ZERO V Signature Amp

**Status:** 🔴 Planning

| ID | Feature | Priority | Status |
|----|---------|----------|--------|
| AMP-01 | Amp model selector (Neural ↔ MLC ZERO V) | P1 | Planning |
| AMP-02 | MLC ZERO V Drive II channel (Faust model) | P1 | Planning |
| AMP-03 | MLC ZERO V tone controls (Bass/Mid/Treble/Presence/Depth) | P1 | Planning |
| AMP-04 | MLC ZERO V switches (Bright, M45, WARCLAW, Feedback) | P1 | Planning |
| AMP-05 | MLC ZERO V gate + master volume | P1 | Planning |
| AMP-06 | MLC ZERO V UI panel (egui controls) | P1 | Planning |
| AMP-07 | Standalone parity for MLC controls | P1 | Planning |

## Future

| ID | Feature | Priority |
|----|---------|----------|
| AMP-08 | MLC ZERO V Clean channel | P3 |
| AMP-09 | MLC ZERO V Drive I channel | P3 |
| AMP-10 | Preset system (save/load amp settings) | P3 |
| AMP-11 | Automated DSP pipeline tests | P2 |

## Completed Features

| ID | Feature | Completed |
|----|---------|-----------|
| EQ-01 | Faust 3-band parametric EQ | ✅ |
| NEURAL-01 | Mojo neural drive (tanh, zero-copy FFI) | ✅ |
| PRE-01 | Pre-EQ convolution (embedded WAV IR) | ✅ |
| CAB-01 | Cabinet IR library + convolution | ✅ |
| UI-01 | Signal chain UI + spectrum analyzer | ✅ |
| VST-01 | VST3 + CLAP plugin exports | ✅ |
| SA-01 | Standalone mode (CPAL audio I/O + eframe UI) | ✅ |
| FFT-01 | FFT spectrum analyzer (2048-pt, Blackman-Harris) | ✅ |
