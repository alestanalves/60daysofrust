fn main() {
    let x = 5; // defining an immutable variable

    match x {
        n if n < 0 => println!("{} is negative", n),
        n if n > 0 => println!("{} is positive", n),
        _ => println!("{} is zero", x), // Match requires all possible outcomes to be handled
    }
}