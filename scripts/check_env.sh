#!/bin/bash

# Cores para saída
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🔍 Verificando ambiente de desenvolvimento...${NC}"

# 1. Verificar Faust
if command -v faust >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Faust encontrado: $(faust --version | head -n 1)${NC}"
else
    echo -e "${RED}❌ Faust não encontrado no PATH.${NC}"
    exit 1
fi

# 2. Verificar Mojo
if command -v mojo >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Mojo encontrado: $(mojo --version | head -n 1)${NC}"
else
    # Tentar caminhos comuns do Mojo
    MOJO_PATH="$HOME/.modular/pkg/packages.modular.com_mojo/bin/mojo"
    VENV_MOJO="./.venv/bin/mojo"
    if [ -f "$VENV_MOJO" ]; then
        echo -e "${GREEN}✅ Mojo encontrado no venv local: $VENV_MOJO${NC}"
    elif [ -f "$MOJO_PATH" ]; then
        echo -e "${GREEN}✅ Mojo encontrado via path local: $MOJO_PATH${NC}"
    else
        echo -e "${RED}❌ Mojo não encontrado. Tente 'modular install mojo'.${NC}"
        exit 1
    fi
fi

# 3. Verificar Modular Auth
if command -v modular >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Modular CLI encontrada.${NC}"
    # modular auth check silencioso se possível, senão apenas avisa
else
    echo -e "${YELLOW}⚠️ Modular CLI não encontrada. O Mojo pode precisar dela.${NC}"
fi

# 4. Verificar permissões das pastas
for dir in "dsp" "neural"; do
    if [ ! -d "$dir" ]; then
        mkdir -p "$dir"
    fi
    if [ -w "$dir" ]; then
        echo -e "${GREEN}✅ Permissão de escrita em /$dir confirmada.${NC}"
    else
        echo -e "${RED}❌ Sem permissão de escrita em /$dir.${NC}"
        exit 1
    fi
done

echo -e "${GREEN}🚀 Ambiente pronto para o build!${NC}"
exit 0
