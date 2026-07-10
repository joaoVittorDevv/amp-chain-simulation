# Cross-platform build guide

Use `cargo xtask` as the canonical build entry point on Linux, macOS, and
Windows. The `make` targets are optional Unix shortcuts that delegate to the
same commands.

## Common requirements

Install these tools on every platform:

- The stable Rust toolchain, including Cargo.
- Git.
- A C/C++ compiler and LLVM `libclang`. The build script uses `bindgen` to
  generate Rust bindings for the versioned Faust C++ wrappers.

After installing the platform-specific tools below, validate the environment:

```bash
cargo xtask check-env
```

The command reports the detected Faust and Mojo versions, the neural backend
for local development, and whether `dsp/` and `neural/` are writable.

## Linux

Install Faust, Clang, and `libclang` with your distribution's package manager.
For Debian and Ubuntu, run:

```bash
sudo apt update
sudo apt install clang libclang-dev faust
```

Mojo is optional. If `mojo` is available in `PATH`, a local virtual
environment, or a standard Modular installation, development builds compile
and use the Mojo neural backend. Without Mojo, the build uses the equivalent
Rust backend.

If your distribution installs `libclang` outside the default loader search
path, set `LIBCLANG_PATH` to the directory that contains the library.

## macOS

Install the native command-line tools and the Homebrew packages:

```bash
xcode-select --install
brew install faust llvm
```

Homebrew uses different default prefixes on each architecture:

- Apple Silicon: `/opt/homebrew`
- Intel: `/usr/local`

Add Homebrew LLVM to `PATH` and point `bindgen` at its libraries. Use
`brew --prefix llvm` so the same commands work on both architectures:

```bash
export PATH="$(brew --prefix llvm)/bin:$PATH"
export LIBCLANG_PATH="$(brew --prefix llvm)/lib"
```

Add these exports to your shell profile if you want them to persist. Mojo is
optional on macOS. When it is installed and detected, development builds use
Mojo; otherwise they use Rust.

Build for the architecture of the installed Rust toolchain. To create a
universal plugin bundle, install both Rust targets before using the
`bundle-universal` command provided by `nih-plug`:

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo xtask bundle-universal distortion --release
```

## Windows

Use a 64-bit MSVC Rust toolchain and install the following components:

1. Install Visual Studio Build Tools with the **Desktop development with C++**
   workload and a current Windows SDK.
2. Install LLVM for Windows and include its `bin` directory in `PATH`.
3. Set `LIBCLANG_PATH` to the directory that contains `libclang.dll`.

In PowerShell, the default LLVM installation uses:

```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
cargo xtask check-env
```

Persist the variable through Windows **Environment Variables** if needed.

Faust is optional on Windows because generated `.hpp` files are versioned in
Git. Install Faust only when you change a `.dsp` source. A build fails rather
than using stale DSP code when a `.dsp` file is newer than its generated
header.

### Optional ASIO support

The default Windows build uses WASAPI and does not require the ASIO SDK. The
ASIO SDK and its environment variable are required only when you enable the
`asio` feature.

1. Download and extract the Steinberg ASIO SDK.
2. Set `ASIO_DIR` to the SDK root and keep `LIBCLANG_PATH` configured.
3. Run the pre-build, then pass the feature to Cargo:

```powershell
$env:ASIO_DIR = "C:\SDKs\asiosdk"
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
cargo xtask pre-build
cargo build --release --features asio
cargo run --release --features asio --bin standalone
```

Do not set `ASIO_DIR` for a normal WASAPI build.

## Neural backend selection

Development builds select the neural backend by toolchain capability, not by
operating-system checks alone.

| Platform | Development backend | How to confirm |
| --- | --- | --- |
| Linux | Mojo when detected and successfully built; Rust otherwise | Run `cargo xtask check-env` and read `Neural backend` |
| macOS | Mojo when detected and successfully built; Rust otherwise | Run `cargo xtask check-env` and read `Neural backend` |
| Windows | Rust; the Mojo SDK is not available | Run `cargo xtask check-env` and read `Neural backend: rust` |

`cargo xtask check-env` reports the backend available for local development.
It does not describe the backend embedded in a distribution bundle.

**Release bundles always use the Rust neural backend.** The bundle path sets
`DISTORTION_FORCE_RUST_NEURAL` so VST3 and CLAP artifacts do not depend on a
Mojo shared library or runtime. To reproduce that selection in a non-bundle
build, set any value for the variable before starting a clean build:

```bash
DISTORTION_FORCE_RUST_NEURAL=1 cargo xtask build
```

In PowerShell, run:

```powershell
$env:DISTORTION_FORCE_RUST_NEURAL = "1"
cargo xtask build
```

Unset the variable to restore automatic development selection.

## Canonical commands

Run these commands from the repository root:

```bash
cargo xtask check-env
cargo xtask build
cargo xtask run
cargo xtask bundle distortion --release
```

- `build` runs the Faust and Mojo pre-build when needed, then runs a release
  Cargo build.
- `run` performs the pre-build and launches the standalone application with
  the platform's neural-library search path configured.
- `bundle` creates the VST3 and CLAP distribution bundles with the Rust neural
  backend.

On Unix, `make build`, `make run`, and `make bundle` are shortcuts only. Use
the `cargo xtask` forms in scripts and cross-platform instructions.

## Validation

For documentation-only changes and normal environment validation, run:

```bash
cargo check
```

Hardware and driver behavior cannot be validated in CI. Follow the
[manual UAT guide](UAT.md) on real macOS and Windows systems before a release.
