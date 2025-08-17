# 🧮 Interactive Calculator

Uma calculadora interativa desenvolvida em Rust que permite realizar operações matemáticas básicas de forma contínua até o usuário decidir sair.

## 📋 Características

- ✅ **Operações básicas**: Adição (+), Subtração (-), Multiplicação (*), Divisão (/)
- ✅ **Interface interativa**: Loop contínuo para múltiplas operações
- ✅ **Validação de entrada**: Tratamento de erros para entradas inválidas
- ✅ **Proteção contra divisão por zero**
- ✅ **Saída controlada**: Digite '#' para sair do programa
- ✅ **Código modularizado**: Funções separadas para cada responsabilidade

## 🚀 Como usar

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) instalado em seu sistema
- Dependência `calculator-huggoxbyte` (configurada automaticamente)

### Instalação e execução

1. **Clone o repositório:**
   ```bash
   git clone https://github.com/Hugg0xByte/interactive_calculator.git
   cd interactive_calculator
   ```

2. **Execute o projeto:**
   ```bash
   cargo run
   ```

### Exemplo de uso

```
=== CALCULADORA INTERATIVA ===
Operações disponíveis: +, -, *, /
Digite '#' para sair do programa
================================

Escolha a operação (+, -, *, /) ou '#' para sair: 
+
Digite o primeiro número: 
15
Digite o segundo número: 
25
Resultado: 15 + 25 = 40
----------------------------

Escolha a operação (+, -, *, /) ou '#' para sair: 
/
Digite o primeiro número: 
100
Digite o segundo número: 
5
Resultado: 100 / 5 = 20
----------------------------

Escolha a operação (+, -, *, /) ou '#' para sair: 
#
Obrigado por usar a calculadora!
```

## 🏗️ Estrutura do projeto

```
interactive_calculator/
├── src/
│   └── main.rs          # Código principal da aplicação
├── Cargo.toml           # Configurações e dependências
├── Cargo.lock           # Lock file das dependências
└── README.md            # Este arquivo
```

## 🔧 Arquitetura do código

O código está organizado em funções modulares:

- **`main()`**: Função principal que controla o loop da aplicação
- **`run_calculator()`**: Executa uma iteração completa da calculadora
- **`read_operator()`**: Lê e valida o operador matemático
- **`read_number()`**: Lê e valida números do usuário
- **`execute_operation()`**: Executa a operação matemática selecionada

## 📦 Dependências

- **calculator-huggoxbyte**: Biblioteca personalizada que fornece as funções matemáticas básicas
  - `calc1`: Funções de adição e subtração
  - `calc2`: Funções de multiplicação e divisão

## 🛡️ Tratamento de erros

A aplicação inclui tratamento robusto de erros:

- **Operações inválidas**: Solicita nova entrada se o operador não for reconhecido
- **Números inválidos**: Loop até o usuário inserir um número válido
- **Divisão por zero**: Detecta e prev
