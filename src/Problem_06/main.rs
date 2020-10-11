fn main() {
    let sum_squares: u64 = (1..101).map(|x: u64| x.pow(2)).sum::<u64>();
    let square_sum: u64 = (1..101).sum::<u64>().pow(2);
    println!("{}", square_sum - sum_squares);
    return;
}