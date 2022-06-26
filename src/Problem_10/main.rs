extern crate primal;

fn main() {
    let mut sum: u64 = 0;
    for i in 1..2 * 10u64.pow(6) {
        if primal::is_prime(i) {
            sum += i;
        }
    }
    println!("{}", sum);
}
