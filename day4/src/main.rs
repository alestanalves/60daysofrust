fn main() {
    // Ownership example
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid

    // Uncommenting the next line would cause a compile-time error
    // println!("{}", s1); // error: borrow of moved value: `s1`

    println!("{}", s2); // prints: hello

    // Borrowing example with immutable reference
    let s3 = String::from("world");
    let len = calculate_length(&s3); // borrowing s3 as an immutable reference
    println!("The length of '{}' is {}", s3, len); // s3 is still valid

    // Borrowing example with mutable reference
    let mut s4 = String::from("Rust");
    change(&mut s4); // borrowing s4 as a mutable reference
    println!("Changed string: {}", s4); // prints: Changed string: Rust programming

    // Lifetimes example
    let s5 = String::from("long string is long");
    {
        let r = &s5; // r borrows s5
        println!("{}", r); // r is valid here
    } // r goes out of scope here
    // s5 is still valid here
    println!("{}", s5);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" programming");
}