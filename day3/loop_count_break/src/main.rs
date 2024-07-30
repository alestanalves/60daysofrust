fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("count = {}", count);
        if count == 11 {
            break;
        }
    }
}
