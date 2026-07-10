# Windows/ASIO Reference Card

## Prerequisites

- MSVC Build Tools, or Visual Studio 2022 Community with the
  **Desktop development with C++** workload.
- LLVM installed with `choco install llvm` or a manual installation.
- `LIBCLANG_PATH` set to the LLVM `bin/` directory (where `libclang.dll` lives).
- Rust installed through rustup with the MSVC toolchain selected:
  `rustup default stable-msvc`.
- Git for Windows.

## Build

Run these commands in PowerShell:

```powershell
git clone ...
cd meu-novo-plugin
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
cargo xtask build
cargo xtask run
```

## ASIO Setup

- Download the ASIO SDK from Steinberg. Free registration is required.
- Set `CPAL_ASIO_DIR` to the ASIO SDK path (this is the variable the `asio-sys`
  build script reads).
- Build with ASIO support: `cargo build --release --features asio`.
- Run with ASIO support: `cargo run --release --bin standalone --features asio`.
- Use raw cargo here rather than `cargo xtask`: `cargo xtask` does not forward
  `--features` to the underlying build/run.
- Known ASIO drivers include ASIO4ALL for generic devices and native drivers
  from Focusrite, RME, MOTU, and other audio interface vendors.

For example, set the SDK path in PowerShell before building:

```powershell
$env:CPAL_ASIO_DIR = "C:\SDKs\asiosdk"
cargo build --release --features asio
cargo run --release --bin standalone --features asio
```

## Known Issues & Troubleshooting

- **`link.exe` not found**: Visual Studio Build Tools isn't installed or isn't
  in `PATH`. Run the build from the **Developer Command Prompt for VS 2022**.
- **`LIBCLANG_PATH` not set**: `bindgen` fails. Verify that LLVM is installed
  and the path is correct.
- **ASIO device not listed**: Some ASIO drivers require the device to be
  connected before launch. Plug in the device, and then restart the
  application.
- **ASIO devices not appearing**: Verify `CPAL_ASIO_DIR` is set AND use the raw
  cargo commands (`cargo build`/`cargo run --bin standalone`) with
  `--features asio`. `cargo xtask` strips `--features` flags, so an ASIO build
  launched through xtask silently omits ASIO support.
- **Single-device ASIO**: Some ASIO drivers expose only one duplex device. The
  UI shows that device for both input and output.
- **Buffer size mismatch**: WASAPI and ASIO may negotiate buffer sizes that
  differ from the requested size. The **Ring buffer slack** slider adjusts
  internal buffering only.
- **Mojo not available**: Windows has no Mojo SDK. The Rust neural fallback is
  always used. This behavior is expected.
- **VST3/CLAP scanning**: DAW hosts on Windows may take longer to scan VST3
  plugins on first launch.
- **Antivirus false positives**: Some antivirus software flags unsigned VST3
  DLLs. Add the bundle directory to the antivirus exclusions if needed.

## Testing Checklist

- [ ] `cargo xtask check-env` passes.
- [ ] `cargo xtask build` succeeds (no Faust, no Mojo = Rust neural).
- [ ] `cargo xtask run` launches the standalone application with WASAPI
      devices visible.
- [ ] `cargo run --release --bin standalone --features asio` shows ASIO devices
      in the driver dropdown.
- [ ] Audio plays through the plugin with no glitches.
- [ ] Switching devices in the UI works.
- [ ] `cargo xtask bundle distortion --release` produces VST3/CLAP bundles.
- [ ] Load VST3 in a DAW (Reaper/Waveform Free recommended).

## CI Reference

The GitHub Actions workflow at `.github/workflows/ci.yml` includes these
Windows jobs:

- `windows-latest` builds without Faust or Mojo and uses the Rust neural
  backend with WASAPI.
- `windows-asio` builds with `--features asio`. It may fail when the ASIO SDK
  isn't cached.
