.PHONY: help check-env pre-build build run bundle clean

help:
	@echo "Targets delegate to 'cargo xtask'. Use 'cargo xtask <verb>' directly."
	@echo "  make check-env  — verify toolchain (faust, mojo)"
	@echo "  make pre-build  — compile .dsp and .mojo sources"
	@echo "  make build      — pre-build + cargo build --release"
	@echo "  make run        — build and launch the standalone app"
	@echo "  make bundle     — build VST3/CLAP distribution bundles"
	@echo "  make clean      — remove generated headers, neural libs, cargo artifacts"

check-env:
	cargo xtask check-env

pre-build:
	cargo xtask pre-build

build:
	cargo xtask build

run:
	cargo xtask run

bundle:
	cargo xtask bundle distortion --release

clean:
	cargo xtask clean
