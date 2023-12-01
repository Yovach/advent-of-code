use core::panic;
use std::fs;

// ([7,7] => 77, [8,5] => 85)
fn concat_numbers(first_number: u32, last_number: u32) -> u32 {
    return format!("{}{}", first_number, last_number).parse().unwrap();
}

fn calculation_from_line(line: &str) -> u32 {
    // Store all numbers from the line into a vec
    let mut numbers: Vec<u32> = Vec::new();
    line.chars().for_each(|char| {
        if char.is_numeric() {
            numbers.push(char.to_digit(10).unwrap())
        }
    });

    match numbers.len() {
        0 => panic!("The line don't have any numbers"),
        1 => {
            let single_number = numbers
                .get(0)
                .expect("We except to retrieve at least one number");
            return concat_numbers(single_number.clone(), single_number.clone());
        }
        _ => {
            let first_number = numbers
                .get(0)
                .expect("We except to retrieve the first number");
            let last_number = numbers
                .get(numbers.len() - 1)
                .expect("We except to retrieve the last number");
            return concat_numbers(first_number.clone(), last_number.clone());
        }
    };
}

fn main() {
    let contents = fs::read_to_string("real_input.txt").expect("The file can't be read");
    let calculations: Vec<_> = contents
        .split("\n")
        .map(|msg| -> u32 { return calculation_from_line(msg) })
        .collect();

    let total = calculations
        .iter()
        .fold(0, |previous, current| -> u32 { return previous + current });

    println!("total: {:?}", total);
}
