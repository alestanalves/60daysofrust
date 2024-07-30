use std::fs::File; // Importa a estrutura File da biblioteca padrão para manipulação de arquivos
use std::io::{self, Read}; // Importa a estrutura io e o trait Read para operações de entrada e saída

fn main() {
    // Exemplo de uso de Result
    match read_username_from_file() {
        // Tenta ler o nome de usuário do arquivo
        Ok(username) => println!("Username: {}", username), // Se a operação for bem-sucedida, imprime o nome de usuário
        Err(e) => println!("Error reading username: {}", e), // Se ocorrer um erro, imprime a mensagem de erro
    }

    // Exemplo de uso de Option
    let some_number = Some(42); // Cria uma variável Option com um valor de 42
    let incremented = increment(some_number); // Chama a função increment com o valor Some(42)
    println!("Incremented number: {:?}", incremented); // Imprime o valor incrementado

    let none_number: Option<i32> = None; // Cria uma variável Option sem valor
    let incremented_none = increment(none_number); // Chama a função increment com None
    println!("Incremented none: {:?}", incremented_none); // Imprime o resultado (None)
}

// Função que lê o nome de usuário de um arquivo e retorna um Result
fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // Tenta abrir o arquivo e propaga o erro se falhar
    let mut username = String::new(); // Cria uma string vazia para armazenar o conteúdo do arquivo
    file.read_to_string(&mut username)?; // Tenta ler o conteúdo do arquivo e propaga o erro se falhar
    Ok(username) // Retorna o nome de usuário se tudo der certo
}

// Função que incrementa um valor opcional e retorna um Option
fn increment(number: Option<i32>) -> Option<i32> {
    match number {
        Some(n) => Some(n + 1), // Incrementa o valor se for Some e retorna Some(n + 1)
        None => None, // Retorna None se não houver valor
    }
}
