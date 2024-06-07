# Introdução ao Rust

## 1. Filosofia do Rust

- Segurança e Performance
- Concorrência

## 2. Ecossistema Rust

**Ferramentas Principais**

- Rustup: Ferramenta de linha de comando para gerenciar instalações de Rust. Facilita a instalação e atualização do Rust, bem como a alternância entre diferentes versões de Rust.
- rustc: O compilador de Rust. Compila o código fonte em binários executáveis, garantindo que o código esteja livre de erros de memória comuns.

**Gestão de Projetos e Dependências**

- Cargo: Sistema de construção e gerenciador de pacotes de Rust. Simplifica a criação, compilação e gerenciamento de dependências em projetos Rust.

- Cargo.toml: Arquivo de configuração usado pelo Cargo para especificar as dependências do projeto, configurações de compilação e metadados do projeto. Centraliza todas as informações do projeto em um único lugar.

- Crates: Pacotes de bibliotecas ou programas Rust que podem ser compartilhados e reutilizados. Promove a reutilização de código e a modularidade, permitindo que os desenvolvedores aproveitem bibliotecas de terceiros.

**Processo de Compilação**

- Etapas do Processo:

## 3. Comparação com Outros Ecossistemas (Python e JavaScript)

**Prós do Rust**

- Segurança de Memória: Graças ao sistema de propriedade do Rust.
- Performance: Comparable a C/C++.
- Concorrência: Seguro e eficiente devido ao modelo de concorrência do Rust.

**Contras do Rust**

- Curva de Aprendizado: Mais íngreme comparado a Python e JavaScript.
- Ecosistema: Menos maduro em algumas áreas específicas comparado a Python (ex. Data Science) e JavaScript (ex. Desenvolvimento Web).

**Comparação Prática**

**Python:** - Pros: Facilidade de uso, vasta biblioteca padrão, excelente para prototipagem e desenvolvimento rápido. - Contras: Performance inferior, não tão seguro em termos de memória.
**JavaScript:**

- Pros: Dominância no desenvolvimento web, enorme ecossistema e comunidade.
- Contras: Problemas de segurança, inconsistências e peculiaridades da linguagem.

**Exemplo de Aplicações Típicas**

- Rust: Sistemas embarcados, software de sistema, blockchain, aplicações de alta performance.
- Python: Ciência de dados, aprendizado de máquina, automação de tarefas, desenvolvimento web (Django, Flask).
- JavaScript: Aplicações web front-end e back-end (Node.js), desenvolvimento de aplicativos móveis (React Native).

## 4. Recursos e Comunidade

**Livros e Tutoriais:**

- [NearX](https://www.nearx.com.br)
- ["The Rust Programming Language" (também conhecido como "The Book")](https://doc.rust-lang.org/book/index.html).
- [Comprehensive Rust 🦀](https://google.github.io/comprehensive-rust/bare-metal.html)
- [Rustlings](https://rustlings.cool).
- [Rust by Examples](https://doc.rust-lang.org/rust-by-example/index.html).

**Comunidade:**

- [Discord](https://discord.com/invite/rust-lang).
- [Fórum](https://users.rust-lang.org).

## 2 Criando um projeto com cargo

```bash
cargo new learn
```

## 2.1 Básico de Rust

- variaveis
- tipos
- if, else
- for, while, loop
- fn
- struct, enum, impl, trait

## 3. Smartcontract Hello World

## 3.1 Instalando o Soroban e Wasm target

```bash
rustup target add wasm32-unknown-unknown
```

```bash
cargo install --locked soroban-cli --version 21.0.0-rc.1
```

## 3.2 Configurar o client (Soroban)

```bash
soroban network add local \
    --global \
    --rpc-url "http://localhost:8000/soroban/rpc" \
    --network-passphrase "Standalone Network ; February 2017"
```

## 3.3 Criando uma wallet com Soroban

```
soroban keys generate --global bob --network local
```

## 3.4 Criando projeto

```bash
soroban contract init soroban-smartcontracts
```

## 3.4 Fazendo build do contrato

```bash
soroban contract build
```

```bash
cargo build --target wasm32-unknown-unknown --release
```

## 3.5 Testando o contrato

```bash
cargo test
```

## 3.6 Deployando o contrato

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source bob \
  --network local
```

## 3.7 Interagindo com o contrato

```bash
soroban contract invoke \
  --id <contract_id> \
  --source bob \
  --network local \
  -- \
  hello \
  --to Lucas
```