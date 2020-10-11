fn main() {
    let mut total: u64 = 0;
    let mut result: u64 = fib(1);
    let mut counter: u32 = 2;
    while result < 4 * 10usize.pow(6) as u64 {
        result= fib(counter);
        if result % 2 == 0 {
            total += result;
        }
        counter += 1;
    }
    print!("{}", total);
}

fn fib(n: u32) -> u64 {
    if n < 2 {
        return 1;
    }
    let mut counter: u32 = 2;
    let mut a: u64 = 1;
    let mut b: u64 = 1;
    while counter <= n {
        let temp: u64 = b;
        b += a;
        a = temp;
        counter += 1;
    }
    return b;
}