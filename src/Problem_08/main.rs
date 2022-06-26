use std::collections::linked_list::LinkedList;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut max: usize = 0;
    let mut f = File::open("src/Problem_8/number.txt").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something");
    let mut char_digits = String::from("");

    for i in input.chars().into_iter() {
        if i.is_digit(10) {
            char_digits.push(i);
        }
    }

    let mut stack: LinkedList<u64> = LinkedList::new();
    for i in char_digits.chars() {
        let number: u64 = parse_number(i);
        if stack.clone().len() < 13 {
            stack.push_back(number);
        } else {
            let mut product: usize = 1;
            let temp = stack.clone();
            for val in temp {
                product *= val as usize;
            }
            if max < product {
                max = product;
            }
            stack.pop_front();
            stack.push_back(number);
        }
    }
    println!("{}", max);
}

fn parse_number(value: char) -> u64 {
    //quick way to parse the value.
    match value {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("Invalid value: {}", value),
    }
}
