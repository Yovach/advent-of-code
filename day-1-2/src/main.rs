use core::panic;
use std::{
    collections::{hash_map, HashMap},
    fs,
};

// ([7,7] => 77, [8,5] => 85)
fn concat_numbers(first_number: &u32, last_number: &u32) -> u32 {
    return format!("{}{}", first_number, last_number).parse().unwrap();
}

fn splelled_number_to_number(value: &str) -> Option<u32> {
    if value.eq("one") {
        return Some(1);
    } else if value.eq("two") {
        return Some(2);
    } else if value.eq("three") {
        return Some(3);
    } else if value.eq("four") {
        return Some(4);
    } else if value.eq("five") {
        return Some(5);
    } else if value.eq("six") {
        return Some(6);
    } else if value.eq("seven") {
        return Some(7);
    } else if value.eq("eight") {
        return Some(8);
    } else if value.eq("nine") {
        return Some(9);
    } else {
        return None;
    }
}

fn calculation_from_line(line: &str) -> u32 {
    println!("line : {line}");
    let mut store: HashMap<usize, u32> = HashMap::new();
    // Store all numbers from the line into a vec
    line.chars().enumerate().for_each(|(index, char)| {
        if char.is_numeric() {
            store.insert(index, char.to_digit(10).unwrap());
        }
    });

    println!("store : {:?}", store);

    return 0;

    // match numbers.len() {
    //     0 => panic!("The line don't have any numbers"),
    //     1 => {
    //         let single_number = numbers
    //             .get(0)
    //             .expect("We except to retrieve at least one number");
    //         return concat_numbers(single_number, single_number);
    //     }
    //     _ => {
    //         let first_number = numbers
    //             .get(0)
    //             .expect("We except to retrieve the first number");
    //         let last_number = numbers
    //             .get(numbers.len() - 1)
    //             .expect("We except to retrieve the last number");
    //         return concat_numbers(first_number, last_number);
    //     }
    // };
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let total: u32 = contents
        .split("\n")
        .map(|msg| -> u32 { return calculation_from_line(msg) })
        .fold(0, |previous, current| -> u32 { return previous + current });

    println!("total: {:?}", total);
}
