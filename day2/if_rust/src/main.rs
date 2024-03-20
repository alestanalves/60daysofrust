fn main() {
    let x = 5;

    if x < 0 {
        println!("Negative");
    } else if x > 0 {
        println!("Positive");
    } else {
        println!("Zero");
    }
}

// Explicit is better than implicit.
// fn main() {
//     let x = 5;

//     match x {
//         n if n < 0 => println!("Negative"),
//         n if n > 0 => println!("Positive"),
//         _ => println!("Zero"), // Match requires all possible outcomes to be handled
//     }
// }