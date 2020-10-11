extern crate primal;

fn main() {
    let mut a: u32 = 1;
    let s: u32 = 1000;
    while a < s / 3 {
        let mut b: u32 = a;
        while b < s / 2 {
            let c: u32 = s - b - a;
            if a * a + b * b == c * c {
                println!("{}", a * b * c);
                return;
            }
            b += 1;
        }
        a += 1;
    }
}
