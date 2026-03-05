# BaseIO - Audio Plugin & Standalone Template

**BaseIO** é um chassi modular e moderno para o desenvolvimento de plugins de áudio e aplicações standalone em **Rust**. Ele utiliza os frameworks `nih-plug` e `egui` para oferecer uma interface rústica e de alta performance.

---

## 🏗️ Como Criar um Novo Plugin (Usando como Template)

O BaseIO não deve ser clonado para ser trabalhado na própria raiz. Ele foi feito para atuar como um gerador "Cookiecutter".

### Passo a Passo Mágico:

1. Faça o clone do BaseIO e renomeie a pasta de destino para o nome do seu novo projeto:
   ```bash
   git clone https://github.com/joaoVittorDevv/baseIO-plug.git meu-novo-plugin
   cd meu-novo-plugin
   ```

2. Inicie a configuração iterativa do seu novo projeto:
   ```bash
   make init
   ```

3. **O que o Makefile faz por você por debaixo dos panos?**
   - Pergunta o nome e o desenvolvedor do novo plugin.
   - Substitui as ocorrências do chassi antigo no código, no `Cargo.toml` e no próprio `Makefile`.
   - **Gera um ID VST3 aleatório e seguro (16-bytes UUID)** para evitar conflito com outros plugins na sua DAW.
   - Descarta o histórico Git do BaseIO e re-inicia um controle de versão zerado.
   - Autodestrói os scripts de inicialização (limpeza).
   - Realiza a comprovação automática compilando o projeto inteiro logo em seguida.

---

## 🚀 Como Desenvolver (O Dia a Dia)

Garantir que o [Rust](https://rustup.rs/) esteja instalado. Todas as ações de desenvolvimento foram elegantemente envelopadas no `Makefile`.

### Standalone (Modo Direto)
Execute a aplicação como um host nativo para ouvir imediatamente os testes sem uma DAW:
```bash
make run
```

### Build e Exportação (VST3 / CLAP)
Para gerar e empacotar a versão nativa (.vst3 / .clap) do seu plugin:
```bash
make bundle
```
> Obs: Os formatos gerados aparecerão na pasta `target/bundled/`.

*(Você também pode usar `make help` no terminal para listar todos os comandos)*

---

## 🎨 Arquitetura Modular
Seu novo projeto possuirá a seguinte organização visual:
*   **`lib.rs`**: Core do Plugin e local central de Parâmetros.
*   **`bin/standalone.rs`**: Host Player embutido.
*   **`core/dsp.rs`**: Para onde vai toda a sua matemática DSP.
*   **`core/ui.rs`**: A camada gráfica ultra veloz renderizada via GPU (egui).

---

## 📜 Licença
GPL-3.0-or-later.
