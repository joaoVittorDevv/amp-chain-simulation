.PHONY: help init build run bundle clean build-faust build-mojo pre-build check-env

check-env:
	@bash ./scripts/check_env.sh

# Configuração Automática do LibTorch (necessário para Neural Amp)
export LIBTORCH ?= $(HOME)/libtorch/libtorch
export LIBTORCH_BYPASS_VERSION_CHECK := 1
export LD_LIBRARY_PATH := $(HOME)/libtorch/libtorch/lib:$(LD_LIBRARY_PATH)

# Configuração do Mojo
export MOJO_HOME ?= $(HOME)/.modular/pkg/packages.modular.com_mojo
export LD_LIBRARY_PATH := $(MOJO_HOME)/lib:$(PWD)/neural:$(LD_LIBRARY_PATH)

help:
	@echo "🎵 Plugin Makefile - Comandos Disponíveis:"
	@echo "  make init    - 🚀 Inicializa um novo plugin a partir do template BaseIO"
	@echo "  make run     - 🎧 Executa o standalone host para desenvolvimento/testes"
	@echo "  make build   - 🛠️  Compila o código fonte de forma tradicional (release)"
	@echo "  make bundle  - 📦 Gera as versões finais VST3 e CLAP (.vst3 / .clap)"
	@echo "  make clean   - 🗑️  Limpa o cache e artefatos de build temporários"
	@echo "  make pre-build - ⚙️ Compila os códigos em Faust (.dsp) e Mojo (.mojo)"

init:
	@if [ -f ./init_plugin.py ]; then \
		echo "Iniciando a configuração mágica do seu novo Plugin..."; \
		python3 ./init_plugin.py; \
	else \
		echo "⚠️ Este projeto já foi inicializado. (init_plugin.py não encontrado)."; \
	fi

build-faust:
	@echo "🔨 Compilando arquivos Faust..."
	@mkdir -p dsp
	@if command -v faust >/dev/null 2>&1; then \
		if [ -f dsp/main.dsp ]; then \
			faust -lang cpp -cn mydsp -I faust-ddsp -i dsp/main.dsp -o dsp/FaustModule.hpp; \
		else \
			echo "⚠️ dsp/main.dsp não encontrado. Pulando etapa Faust."; \
		fi \
	else \
		echo "⚠️ Transpilador Faust não encontrado. Pulando etapa Faust."; \
	fi

build-mojo:
	@echo "🔨 Compilando arquivos Mojo..."
	@mkdir -p neural
	@if command -v mojo >/dev/null 2>&1 || [ -f ./.venv/bin/mojo ]; then \
		if [ -f neural/main.mojo ]; then \
			if [ -f ./.venv/bin/mojo ]; then \
				./.venv/bin/mojo build --emit shared-lib neural/main.mojo -o neural/libneural.so; \
			else \
				mojo build --emit shared-lib neural/main.mojo -o neural/libneural.so; \
			fi \
		else \
			echo "⚠️ neural/main.mojo não encontrado. Pulando etapa Mojo."; \
		fi \
	else \
		echo "⚠️ Compilador Mojo não encontrado. Pulando etapa Mojo."; \
	fi

pre-build: check-env build-faust build-mojo

run: pre-build
	./scripts/run_standalone.sh

build: pre-build
	cargo build --release

bundle: pre-build
	cargo xtask bundle distortion --release

clean:
	cargo clean
	rm -f dsp/*.hpp dsp/*.cpp
	rm -f neural/*.so neural/*.dylib neural/*.dll
