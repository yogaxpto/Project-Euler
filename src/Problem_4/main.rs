fn main() {
    let mut max: u32 = 0;
    for i in (1..999).rev() {
        for k in (1..i).rev() {
            let product: u32 = i * k;
            if is_palindrome((product).to_string()) && product > max {
                max = product;
            }
        }
    }
    print!("{}", max);
    return;
}

fn is_palindrome(word: String) -> bool {
    return word == word.chars().rev().collect::<String>();
}
