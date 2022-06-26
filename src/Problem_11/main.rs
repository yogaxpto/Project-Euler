use std::collections::linked_list::LinkedList;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("src/Problem_11/matrix.txt").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("error");
    let mut char_digits = String::from("");

    for i in input.chars().into_iter() {
        if i.is_digit(10) {
            char_digits.push(i);
        }
    }
}
