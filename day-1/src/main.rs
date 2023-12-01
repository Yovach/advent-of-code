use core::panic;
use std::fs;

fn numbers_from_line(line: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    line.chars().for_each(|char| {
        if char.is_numeric() {
            numbers.push(char.to_digit(10).unwrap())
        }
    });
    if numbers.len() == 0 {
        panic!("The line don't have any numbers");
    } else if numbers.len() == 1 {
        let single_number = numbers
            .get(0)
            .expect("We except to retrieve at least one number");
        println!("char: {single_number}");
        return [single_number.clone(), single_number.clone()].to_vec();
    }

    let first_number = numbers
        .get(0)
        .expect("We except to retrieve the first number");
    let last_number = numbers
        .get(numbers.len() - 1)
        .expect("We except to retrieve the last number");

    return [first_number.clone(), last_number.clone()].to_vec();
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let lines: Vec<_> = contents
        .split("\n")
        .map(|msg| -> Vec<u32> { return numbers_from_line(msg) })
        .collect();

    println!("lines: {:?}", lines);
}
