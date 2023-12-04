use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("i should read the file");
    let lines = contents.lines();

    let cards: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in lines {
        let card_data: Vec<&str> = line.split(":").collect();
        let card_id: u32 = card_data
            .first()
            .unwrap()
            .split(" ")
            .last()
            .expect("i expect an ID")
            .parse()
            .unwrap();

        let card_numbers: Vec<&str> = card_data
            .last()
            .unwrap()
            .split("|")
            .map(|numbers| numbers.trim())
            .collect();

        let winning_numbers: Vec<u32> = card_numbers
            .first()
            .expect("i expected winning numbers")
            .split(" ")
            .filter(|number| !number.is_empty())
            .map(|number| number.parse::<u32>().unwrap())
            .collect();

        let played_numbers: Vec<u32> = card_numbers
            .last()
            .expect("i expected played numbers")
            .split(" ")
            .filter(|number| !number.is_empty())
            .map(|number| number.parse::<u32>().unwrap())
            .collect();
        println!(
            "card id : {:?}, winning: {:?}, played: {:?}",
            card_id, winning_numbers, played_numbers
        )
    }

    println!("Hello, world!");
}
