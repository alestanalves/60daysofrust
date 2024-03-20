fn main() {
    // Declaração de uma variável imutável
    let name = "Rustacean";
    println!("Hello, {}!", name);

    // Declaração de uma variável mutável
    let mut age = 30;
    println!("Age: {}", age);
    
    // Modificar a variável mutável
    age = 31;
    println!("New Age: {}", age);

    // Variável com tipo de dados explicitamente declarado
    let is_programmer: bool = true;
    println!("Is a programmer: {}", is_programmer);

    // Usando uma constante
    const PI: f32 = 3.14159;
    println!("Value of PI: {}", PI);
}

// let é usado para declarar variáveis. Por padrão, variáveis em Rust são imutáveis (immutable), o que significa que uma vez que um valor é atribuído a elas, não pode ser alterado.
// Para declarar uma variável mutável (cujo valor pode ser alterado), use let mut.
// Rust tem inferência de tipo, mas você pode declarar o tipo explicitamente se desejar.
// const é usado para definir uma constante. As constantes são sempre imutáveis e o tipo deve ser especificado explicitamente.