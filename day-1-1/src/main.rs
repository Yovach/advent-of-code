use core::panic;
use std::fs;

fn numbers_from_line(line: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
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
        let calculation = single_number.clone() + single_number.clone();
        println!("calculation {calculation}");
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

fn calculation_from_numbers(values: Vec<u32>) -> u32 {
    let first_number = values
        .get(0)
        .expect("We except to retrieve the first number");
    let last_number = values
        .get(1)
        .expect("We except to retrieve the second number");

    return format!("{}{}", first_number, last_number).parse().unwrap();
}

fn main() {
    let contents = fs::read_to_string("real_input.txt").expect("The file can't be read");
    let calculations: Vec<_> = contents
        .split("\n")
        .map(|msg| -> Vec<u32> { return numbers_from_line(msg) })
        .map(|msg| -> u32 { return calculation_from_numbers(msg) })
        .collect();

    let total = calculations
        .iter()
        .fold(0, |previous, current| -> u32 { return previous + current });

    println!("lines: {:?}", total);
}
