# Slint App - Aplicação de Exemplo

Uma aplicação de exemplo criada com **Rust** e **Slint** para demonstrar os conceitos básicos de desenvolvimento de interfaces gráficas nativas.

## 📋 Sobre o Projeto

Este projeto é uma aplicação simples que demonstra:
- ✅ Configuração básica de um projeto Rust com Slint
- ✅ Criação de interfaces declarativas com arquivos `.slint`
- ✅ Interação entre código Rust e componentes UI
- ✅ Gerenciamento de estado reativo
- ✅ Manipulação de eventos de usuário

## 🎯 Funcionalidades

- **Contador Interativo**: Botão que incrementa um contador
- **Campo de Texto**: Input de texto para demonstrar widgets básicos
- **Interface Responsiva**: Layout que se adapta ao tamanho da janela
- **Código Documentado**: Todos os arquivos possuem comentários explicativos

## 🛠️ Tecnologias Utilizadas

- **[Rust](https://www.rust-lang.org/)**: Linguagem de programação segura e performática
- **[Slint](https://slint.dev/)**: Framework UI multiplataforma para aplicações nativas
- **Cargo**: Sistema de build e gerenciador de pacotes do Rust

## 📁 Estrutura do Projeto

```
slint-app/
├── Cargo.toml          # Configuração do projeto e dependências
├── build.rs            # Script de build para compilar arquivos .slint  
├── README.md           # Este arquivo
├── src/
│   └── main.rs         # Código principal da aplicação
└── ui/
    └── app.slint       # Definição da interface do usuário
```

## ⚙️ Pré-requisitos

Antes de executar o projeto, certifique-se de ter instalado:

- **Rust** (versão 1.70 ou superior)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Dependências do sistema** (Linux):
  ```bash
  # Ubuntu/Debian
  sudo apt-get install build-essential libfontconfig1-dev libxcb-composite0-dev
  
  # Fedora
  sudo dnf install fontconfig-devel libxcb-devel
  ```

## 🚀 Como Executar

1. **Clone ou navegue até o diretório do projeto**:
   ```bash
   cd slint-app
   ```

2. **Execute a aplicação**:
   ```bash
   cargo run
   ```

3. **Para compilar apenas (sem executar)**:
   ```bash
   cargo build
   ```

## 📖 Entendendo o Código

### Cargo.toml
Configura as dependências do projeto:
- `slint`: Framework UI principal
- `slint-build`: Ferramenta de build para arquivos `.slint`

### build.rs
Script executado durante a compilação que transforma arquivos `.slint` em código Rust utilizável.

### src/main.rs
Código principal que:
- Inicializa a aplicação Slint
- Cria a janela principal
- Inicia o loop de eventos

### ui/app.slint
Interface declarativa que define:
- Layout da aplicação (VerticalBox)
- Componentes visuais (Text, Button, LineEdit)
- Lógica de interação (incremento do contador)
- Propriedades reativas

## 🎨 Personalizando a Interface

Para modificar a aparência ou funcionalidade:

1. **Alterar textos**: Edite as propriedades `text` em `ui/app.slint`
2. **Mudar cores**: Adicione propriedades como `color` ou `background`
3. **Novos widgets**: Importe de `std-widgets.slint` e adicione ao layout
4. **Lógica personalizada**: Modifique os callbacks em `clicked => { ... }`

## 🔧 Comandos Úteis

```bash
# Executar em modo debug
cargo run

# Compilar para produção (otimizado)
cargo build --release

# Verificar código sem compilar  
cargo check

# Executar testes
cargo test

# Limpar arquivos de build
cargo clean
```

## 📚 Recursos de Aprendizado

- **[Documentação do Slint](https://slint.dev/docs/)**: Guia completo do framework
- **[Exemplos do Slint](https://github.com/slint-ui/slint/tree/master/examples)**: Projetos de exemplo
- **[Rust Book](https://doc.rust-lang.org/book/)**: Guia oficial da linguagem Rust
- **[Slint Language Reference](https://slint.dev/docs/slint/)**: Referência da linguagem `.slint`

## 🎓 Próximos Passos

Para expandir seus conhecimentos, experimente:

1. **Adicionar mais widgets**: CheckBox, Slider, ComboBox
2. **Implementar navegação**: Múltiplas telas/páginas
3. **Persistir dados**: Salvar estado em arquivos
4. **Integrar APIs**: Fazer requisições HTTP
5. **Personalizar temas**: Criar estilos customizados
6. **Adicionar ícones**: Usar recursos de imagem

## 🐛 Problemas Comuns

**Erro de compilação**: Verifique se todas as dependências do sistema estão instaladas.

**Interface não aparece**: Certifique-se de que o driver gráfico suporta OpenGL ou use backend alternativo.

**Permissões no Linux**: Alguns sistemas podem precisar de configurações adicionais de permissão.

## 📄 Licença

Este projeto é livre para uso educacional e pessoal.

---

**Desenvolvido para aprendizado de Rust e Slint** 🦀✨