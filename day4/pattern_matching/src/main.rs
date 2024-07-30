// Define uma enumeração chamada Coin com quatro variantes
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Função que recebe um Coin e retorna seu valor em centavos
fn value_in_cents(coin: Coin) -> u8 {
    // Usa a expressão match para determinar o valor da moeda
    match coin {
        // Se a moeda for Penny, retorna 1
        Coin::Penny => 1,
        // Se a moeda for Nickel, retorna 5
        Coin::Nickel => 5,
        // Se a moeda for Dime, retorna 10
        Coin::Dime => 10,
        // Se a moeda for Quarter, retorna 25
        Coin::Quarter => 25,
    }
}

fn main() {
    // Cria uma variável coin e atribui a ela a variante Coin::Dime
    let coin = Coin::Dime;
    // Imprime o valor em centavos da moeda
    println!("Value in cents: {}", value_in_cents(coin));

    // Cria uma variável number e atribui a ela o valor 7
    let number = 7;
    // Usa a expressão match para comparar number com diferentes valores
    match number {
        // Se number for 1, imprime "One"
        1 => println!("One"),
        // Se number for 2, imprime "Two"
        2 => println!("Two"),
        // Se number for 3, imprime "Three"
        3 => println!("Three"),
        // Se number for 4, 5 ou 6, imprime "Four, Five, or Six"
        4 | 5 | 6 => println!("Four, Five, or Six"),
        // Para qualquer outro valor, imprime "Greater than Six"
        _ => println!("Greater than Six"),
    }

    // Cria uma variável some_value e atribui a ela Some(3)
    let some_value = Some(3);
    // Usa a expressão match para comparar some_value com Some(3)
    match some_value {
        // Se some_value for Some(3), imprime "Three"
        Some(3) => println!("Three"),
        // Para qualquer outro valor, imprime "Not Three"
        _ => println!("Not Three"),
    }

    // Usa if let para verificar se some_value é Some(3)
    if let Some(3) = some_value {
        // Se for Some(3), imprime "Matched with if let: Three"
        println!("Matched with if let: Three");
    }
}
