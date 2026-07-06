# Project: Distortion Plugin

**Created:** 2026-07-06
**Last updated:** 2026-07-06

## Vision

A professional-grade guitar distortion audio plugin (VST3/CLAP + standalone) that combines Faust DSP modeling, Mojo neural processing, and high-quality cabinet impulse response simulation into a single, low-latency processing chain for modern metal guitar tones.

## Goals

- [ ] Dual amplifier architecture: Neural (Mojo tanh) + MLC ZERO V Signature modeling (Faust)
- [ ] Seamless A/B switching between amp models with independent parameter sets
- [ ] Professional cabinet IR library with import, management, and convolution
- [ ] Real-time FFT spectrum analyzer for visual feedback
- [ ] Full parametric EQ (Faust 3-band) with adjustable Q per band
- [ ] Standalone mode with flexible audio routing (CPAL)
- [ ] Low-latency, zero-allocation audio thread
- [ ] Cross-platform: Linux primary, macOS/Windows compatible

## Target Audience

- Modern metal guitarists and producers
- Home studio users seeking high-gain amp simulation
- Live performers using VST3/CLAP hosts

## Tech Stack

- **Core:** Rust 2021
- **Plugin framework:** nih_plug (VST3 + CLAP)
- **DSP:** Faust (analog modeling), Mojo (neural/saturation)
- **Convolution:** FFTConvolver (pre-EQ + cabinet IR)
- **UI:** egui/eframe (plugin editor + standalone)
- **Audio I/O:** CPAL (standalone)

## Design Targets (not yet verified — feature is under development)

- [ ] Real-time performance: < 5ms latency at 48kHz, 128-sample buffer
- [ ] Zero allocations on audio thread (asserted by nih_plug)
- [ ] Toggle between Neural and MLC ZERO V amp models with < 10ms transition
- [ ] Cabinet IR switching with smooth crossfade, no audible artifacts
