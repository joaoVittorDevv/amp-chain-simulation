# Manual Verification Procedure — EQ + Neural Amp Fix

**Date**: 2026-04-28
**Scope**: EQ linearity, Q range, Pre-EQ Faust filter, ONNX WaveNet integration
**Test Type**: Manual (no automated test suite)

---

## Prerequisites

- Test machine: Linux with audio interface or built-in audio
- Test signals (generate with `sox` or similar):
  - Sine sweep: `sox -n -r 44100 -c 2 sine_sweep.wav synth 10 20 20000`
  - 1 kHz sine: `sox -n -r 44100 -c 2 sine_1k.wav synth 5 1000 1000`
- FFT analyzer: `baudline`, `foobar2000` (via Wine), or any DAW spectrum analyzer

---

## Test 1: EQ Bypass Linearity

**Requirement**: EQ-04
**Goal**: Confirm EQ bypass produces unity gain — no compression, no coloration.

### Procedure

1. Launch standalone: `cargo run --release --bin standalone`
2. Disable EQ: toggle `EQ Active` OFF
3. Play 1 kHz sine through the plugin at moderate level (-12 dBFS)
4. Measure output level
5. Enable EQ: toggle `EQ Active` ON
6. Set all EQ gains to 0 dB
7. Re-measure output level
8. **Pass criteria**: output level difference < 0.1 dB

### Pass Criteria
- [ ] Output level unchanged when EQ is bypassed with 0 dB gains
- [ ] No audible artifacts or distortion introduced by bypass

---

## Test 2: EQ Frequency Response (3-Band Sweep)

**Requirement**: EQ-01, EQ-02, EQ-03
**Goal**: Confirm each band produces expected frequency-dependent gain.

### Procedure

1. Play sine sweep through the plugin
2. Enable EQ, set all gains to 0 dB
3. For each band, boost +6 dB and observe the peak on the FFT:

| Band     | Frequency | Expected Peak Location |
|----------|-----------|-----------------------|
| Low      | 100 Hz    | ~100 Hz region        |
| Mid      | 1 kHz     | ~1 kHz region         |
| High     | 5 kHz     | ~5 kHz region         |

4. Cut -6 dB and confirm attenuation at the same regions
5. **Pass criteria**: each band affects only its target frequency range (no bleed >3 dB into adjacent bands at default Q=0.707)

### Pass Criteria
- [ ] Low band boosts/cuts around 100 Hz
- [ ] Mid band boosts/cuts around 1 kHz
- [ ] High band boosts/cuts around 5 kHz
- [ ] No unexpected interactions between bands

---

## Test 3: Q Range Verification

**Requirement**: Q-01, Q-02
**Goal**: Confirm Q=0.1 (broad) and Q=10.0 (narrow) produce expected bandwidth extremes.

### Procedure

1. Set Mid band to +6 dB at 1 kHz
2. Set Q to 0.1 (minimum): the boost should be very wide (covers ~2 octaves)
3. Set Q to 10.0 (maximum): the boost should be very narrow (covers ~1/3 octave)
4. **Pass criteria**: audible and measurable difference in bandwidth

### Pass Criteria
- [ ] Q=0.1 produces broad, smooth boost
- [ ] Q=10.0 produces sharp, focused peak
- [ ] No clicks or artifacts when sweeping Q during playback

---

## Test 4: Pre-EQ Tone Stack

**Requirement**: PE-01, PE-02
**Goal**: Confirm Pre-EQ (Faust tone stack) replaces the old 256-sample IR convolution.

### Procedure

1. With EQ enabled, sweep Pre Low Gain from -12 to +12 dB — bass should audibly change
2. Sweep Pre High Gain from -12 to +12 dB — treble should audibly change
3. Sweep Pre Tone Freq — the crossover point between low/high shelves should shift
4. Sweep Pre Tone Q — bandwidth of each shelf should change
5. Verify no `pre_eq_ir.wav` loading message appears in stderr at startup
6. **Pass criteria**: Pre-EQ is active and audibly affects tone shaping

### Pass Criteria
- [ ] Pre Low Gain changes bass content
- [ ] Pre High Gain changes treble content
- [ ] Pre Tone Freq shifts the crossover point
- [ ] No `pre_eq_ir.wav` convolution artifacts

---

## Test 5: Neural Drive — Tanh Fallback

**Requirement**: DR-01, DR-02, DR-03
**Goal**: Confirm neural drive parameter is linear (0–3x) and NOT applied twice.

### Procedure

1. Play sine sweep through the plugin
2. Set Neural Drive = 1.0 (unity)
3. Set Neural Amp Active = ON
4. Measure output level at 1 kHz
5. Set Neural Drive = 2.0 — output level should approximately double (6 dB increase)
6. Set Neural Drive = 3.0 — output level should approximately triple (9.5 dB increase)
7. Set Neural Drive = 0.0 — output should be clean (no distortion)
8. **Pass criteria**: linear relationship between drive value and output energy

### Pass Criteria
- [ ] Neural Drive = 1.0 produces moderate distortion
- [ ] Neural Drive = 3.0 produces significantly heavier distortion than 1.0
- [ ] No double-application of volume (level should not spike unexpectedly)
- [ ] Neural Amp Active OFF passes audio cleanly

---

## Test 6: ONNX WaveNet Loading (if model present)

**Requirement**: NN-01, NN-02, NN-03, NN-04
**Goal**: Confirm `wavenet_drive.onnx` is loaded when present; fallback to tanh when absent.

### Procedure

1. Verify `wavenet_drive.onnx` exists in `neural/drive/`
2. Launch plugin — check stderr for `Wavenet loaded successfully` message
3. If message appears: ONNX model is active — compare distortion character to Neural Drive = 1.0
4. Rename `wavenet_drive.onnx` to `wavenet_drive.onnx.bak`
5. Re-launch — check for NO `Wavenet loaded` message
6. Distortion should still work (via TanhFallbackProcessor)
7. Restore file
8. **Pass criteria**: model loads when present, tanh fallback works when absent

### Pass Criteria
- [ ] `Wavenet loaded successfully` in stderr when model exists
- [ ] No crash when model is absent
- [ ] Fallback tanh distortion is audible

---

## Test 7: Full Pipeline — Stereo Processing

**Goal**: Confirm L and R channels are processed independently through Faust pre-EQ + EQ.

### Procedure

1. Load plugin in a DAW with stereo input
2. Send stereo sine sweep
3. Pan L-only, then R-only — confirm each channel responds to EQ knobs
4. Set Input Select to "Input 1 (Mic)" — confirm mono sum behavior
5. **Pass criteria**: stereo routing works correctly

### Pass Criteria
- [ ] L and R channels are processed independently
- [ ] EQ affects both channels
- [ ] Pre-EQ affects both channels

---

## Quick Smoke Test (5 minutes)

If time is limited, run this subset:

1. Launch standalone
2. Toggle EQ bypass — confirm output level unchanged
3. Boost Low Gain +6 dB at 100 Hz — confirm bass increase
4. Boost High Gain +6 dB at 5 kHz — confirm treble increase
5. Set Neural Drive = 0.0 → 3.0 — confirm increasing distortion
6. Check stderr for no crash messages

**If all smoke tests pass, the implementation is functionally correct.**
