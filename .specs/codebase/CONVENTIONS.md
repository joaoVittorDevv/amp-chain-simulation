# Code Conventions

**Analyzed:** 2026-07-06

## Naming Conventions

**Files/Modules:** `snake_case`
- `plugin_params.rs`, `signal_chain.rs`, `cabinet_panel.rs`, `main_view.rs`

**Types (structs, enums):** `PascalCase`
- `BaseIO`, `BaseIOParams`, `EditorState`, `InputSelect`, `ActivePanel`
- `FaustProcessor`, `MojoProcessor`, `CabinetEngine`, `AnalyzerDsp`

**Functions/Fields:** `snake_case`
- `process_block()`, `set_eq_params()`, `decode_wav_flat()`
- `neural_output_gain`, `eq_low_freq`, `cabinet_bypass`

**Constants:** `SCREAMING_SNAKE_CASE`
- `APP_NAME`, `DEFAULT_CABINET_IR`, `DEFAULT_PRE_EQ_IR`, `FFT_SIZE`, `HISTORY_LEN`

**Faust labels:** Title-case UI strings
- `"EQ Low Freq"`, `"EQ Mid Gain"`, `"EQ High Q"`

## Import Organization

- External crates first, then `std`, then local modules
- Not strictly alphabetized; pragmatic grouping
- Example from `src/lib.rs:1-14`

## Error Handling

| Context | Pattern |
|---------|---------|
| Build-time | `panic!()` / `expect()` for missing Faust/Mojo |
| Runtime init | Degrade to fallback (cabinet in-memory DB if data dir fails) |
| Audio DSP | Passthrough on convolution error, never panic |
| UI | Capture in `cabinet_error` for display |
| CPAL setup | `AudioEvent::StreamStarted(Result<(), String>)` |

## Parameter Smoothing

```rust
// Gain-style (logarithmic):
FloatParam::new("Name", default, FloatRange::Skewed { ... })
    .with_smoother(SmoothingStyle::Logarithmic(50.0))
    .with_unit(" dB")
    .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
    .with_string_to_value(formatters::s2v_f32_gain_to_db())

// Linear/percentage:
FloatParam::new("Name", default, FloatRange::Linear { min, max })
    .with_smoother(SmoothingStyle::Linear(50.0))
    .with_unit(" %")
```

**Audio thread:** `self.params.param_name.smoothed.next()` — once per block, not per sample.

## Parameter Definition Pattern

```rust
// In BaseIOParams struct:
#[id = "param_id"]
pub param_name: FloatParam,

// In impl Default for BaseIOParams:
param_name: FloatParam::new("Display Name", default, range)
    .with_smoother(SmoothingStyle::Logarithmic(50.0))
    .with_unit(" dB")
    .with_value_to_string(...)
    .with_string_to_value(...),
```

## Comments/Documentation

- Bilingual Portuguese/English
- Stage-labeled comments in audio pipeline: `// Stage 1: Input routing`
- Rustdoc-style on cabinet subsystem (`///` blocks on public API)
- Some stale labels: UI says "Neural Amp (PyTorch)" but active path is Mojo

## Audio Thread Safety

- No `Mutex` on audio thread
- Pre-allocated buffers (`temp_buffer`, scratch, convolver state)
- Zero-copy raw pointer FFI (no allocation in `process_block`)
- Ring buffers (`rtrb`) for audio ↔ UI communication

## FFI Patterns

**Adding external function:**
```rust
extern "C" {
    fn ext_init(sample_rate: f32);
    fn ext_process_block(address: usize, size: usize, ...params);
}
```

**C ABI wrapper (Faust):**
- Opaque handle pointer returned by C constructor
- Label-based param setters via `SET_PARAM("Exact Label", value)` macro
- `unsafe impl Send` assumption for host thread scheduling
