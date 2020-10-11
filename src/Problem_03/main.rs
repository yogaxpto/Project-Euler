fn main() {
    let mut input: u64 = 600851475143;
    let mut result: u64 = 2;
    //println!("{}",is_prime(result));
    while result * result <= input {
        if input % result == 0 {
            input /= result;
        } else {
            result += 1;
        }
    }
    print!("{}", input);
}
