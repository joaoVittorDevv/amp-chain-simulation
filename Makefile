.PHONY: help init build run bundle clean

help:
	@echo "🎵 Plugin Makefile - Comandos Disponíveis:"
	@echo "  make init    - 🚀 Inicializa um novo plugin a partir do template BaseIO"
	@echo "  make run     - 🎧 Executa o standalone host para desenvolvimento/testes"
	@echo "  make build   - 🛠️  Compila o código fonte de forma tradicional (release)"
	@echo "  make bundle  - 📦 Gera as versões finais VST3 e CLAP (.vst3 / .clap)"
	@echo "  make clean   - 🗑️  Limpa o cache e artefatos de build temporários"

init:
	@if [ -f ./init_plugin.py ]; then \
		echo "Iniciando a configuração mágica do seu novo Plugin..."; \
		python3 ./init_plugin.py; \
	else \
		echo "⚠️ Este projeto já foi inicializado. (init_plugin.py não encontrado)."; \
	fi

run:
	./scripts/run_standalone.sh

build:
	cargo build --release

bundle:
	cargo xtask bundle distortion --release

clean:
	cargo clean
