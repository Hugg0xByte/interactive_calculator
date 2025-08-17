use calculator_huggoxbyte::calc1::{add, sub};
use calculator_huggoxbyte::calc2::{multiply, rate};
use std::io;

fn main() {
    println!("=== CALCULADORA INTERATIVA ===");
    println!("Operações disponíveis: +, -, *, /");
    println!("Digite '#' para sair do programa");
    println!("================================");

    loop {
        match run_calculator() {
            true => continue, // Continua o loop
            false => break,   // Sai do loop
        }
    }

    println!("Obrigado por usar a calculadora!");
}

/// Executa uma iteração da calculadora
/// Retorna true para continuar, false para sair
fn run_calculator() -> bool {
    // Lê o operador
    let operator = match read_operator() {
        Some(op) => op,
        None => return false, // Usuário quer sair
    };

    // Lê os números
    let num_a = read_number("Digite o primeiro número: ");
    let num_b = read_number("Digite o segundo número: ");

    // Executa a operação
    let result = execute_operation(&operator, num_a, num_b);

    // Mostra o resultado
    println!("Resultado: {} {} {} = {}", num_a, operator, num_b, result);
    println!("----------------------------");

    true // Continua o programa
}

/// Lê o operador do usuário
/// Retorna Some(operador) ou None se o usuário quer sair
fn read_operator() -> Option<String> {
    println!("Escolha a operação (+, -, *, /) ou '#' para sair: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a operação");

    let operator = input.trim().to_string();

    // Verifica se o usuário quer sair
    if operator == "#" {
        return None;
    }

    // Valida se é uma operação válida
    match operator.as_str() {
        "+" | "-" | "*" | "/" => Some(operator),
        _ => {
            println!("❌ Operação inválida! Use: +, -, *, / ou # para sair");
            read_operator() // Recursão para pedir novamente
        }
    }
}

/// Lê um número do usuário
fn read_number(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler o número");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("❌ Por favor, digite um número válido!");
                continue;
            }
        }
    }
}

/// Executa a operação matemática
fn execute_operation(operator: &str, num_a: u32, num_b: u32) -> u32 {
    match operator {
        "+" => add(num_a, num_b),
        "-" => sub(num_a, num_b),
        "*" => multiply(num_a, num_b),
        "/" => {
            if num_b == 0 {
                println!("❌ Erro: Divisão por zero!");
                return 0;
            }
            rate(num_a, num_b)
        }
        _ => {
            println!("❌ Operação não reconhecida");
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adicao() {
        let resultado = execute_operation("+", 5, 3);
        assert_eq!(resultado, 8);
    }

    #[test]
    fn test_subtracao() {
        let resultado = execute_operation("-", 10, 4);
        assert_eq!(resultado, 6);
    }

    #[test]
    fn test_multiplicacao() {
        let resultado = execute_operation("*", 7, 6);
        assert_eq!(resultado, 42);
    }

    #[test]
    fn test_divisao() {
        let resultado = execute_operation("/", 20, 4);
        assert_eq!(resultado, 5);
    }

    #[test]
    fn test_divisao_por_zero() {
        let resultado = execute_operation("/", 10, 0);
        assert_eq!(resultado, 0); // Deve retornar 0 quando divide por zero
    }

    #[test]
    fn test_operador_invalido() {
        let resultado = execute_operation("%", 5, 3);
        assert_eq!(resultado, 0); // Deve retornar 0 para operador inválido
    }

    #[test]
    fn test_numeros_grandes() {
        let resultado = execute_operation("+", 1000, 2000);
        assert_eq!(resultado, 3000);
    }

    #[test]
    fn test_com_zero() {
        assert_eq!(execute_operation("+", 0, 5), 5);
        assert_eq!(execute_operation("*", 0, 100), 0);
    }
}
