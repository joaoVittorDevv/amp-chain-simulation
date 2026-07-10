.PHONY: help check-env pre-build build run bundle bench bench-baseline clean

BENCH_BASELINE ?= main
BENCH_THRESHOLD ?= 5
BENCH_TARGETS := --bench dsp_pipeline --bench faust_eq --bench neural_drive --bench resampler

help:
	@echo "Targets delegate to 'cargo xtask'. Use 'cargo xtask <verb>' directly."
	@echo "  make check-env  — verify toolchain (faust, mojo)"
	@echo "  make pre-build  — compile .dsp and .mojo sources"
	@echo "  make build      — pre-build + cargo build --release"
	@echo "  make run        — build and launch the standalone app"
	@echo "  make bundle     — build VST3/CLAP distribution bundles"
	@echo "  make bench      — compare all benchmarks with the saved baseline (max 5% regression)"
	@echo "  make bench-baseline — save the current benchmark results as the comparison baseline"
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

bench:
	DISTORTION_FORCE_RUST_NEURAL=1 cargo bench $(BENCH_TARGETS) -- --baseline "$(BENCH_BASELINE)" --noise-threshold 0.05
	python3 benches/check_regressions.py target/criterion "$(BENCH_BASELINE)" "$(BENCH_THRESHOLD)"

bench-baseline:
	DISTORTION_FORCE_RUST_NEURAL=1 cargo bench $(BENCH_TARGETS) -- --save-baseline "$(BENCH_BASELINE)"

clean:
	cargo xtask clean
