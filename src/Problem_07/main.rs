extern crate primal;
fn main() {
    let mut counter: u16 = 1;
    let mut x: u64 = 2;
    while counter <= 10001 {
        if primal::is_prime(x) {
            counter += 1;
        }
        x += 1;
    }
    println!("{}", x - 1);
}
