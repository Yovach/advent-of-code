use std::{collections::HashMap, fs, ops::Add};

#[derive(Debug)]
enum Category {
    SEED_TO_SOIL,
    SOIL_TO_FERTILIZER,
    FERTILIZER_TO_WATER,
    WATER_TO_LIGHT,
    LIGHT_TO_TEMPERATURE,
    TEMPERATURE_TO_HUMIDITY,
    HUMIDITY_TO_LOCATION,
}

fn main() {
    let file_content =
        fs::read_to_string("./input.txt").expect("i should be able to read this file");
    let lines: Vec<&str> = file_content.lines().collect();

    let categories_store: HashMap<Category, HashMap<u32, u32>> = HashMap::new();
    let mut lastCategory: Option<Category> = None;

    for line in &lines {
        if line.starts_with("seed-to-soil map") {
            lastCategory = Some(Category::SEED_TO_SOIL);
        } else if line.starts_with("soil-to-fertilizer map") {
            lastCategory = Some(Category::SOIL_TO_FERTILIZER);
        } else if line.starts_with("fertilizer-to-water map") {
            lastCategory = Some(Category::FERTILIZER_TO_WATER);
        } else if line.starts_with("water-to-light map") {
            lastCategory = Some(Category::WATER_TO_LIGHT);
        } else if line.starts_with("light-to-temperature map") {
            lastCategory = Some(Category::LIGHT_TO_TEMPERATURE);
        } else if line.starts_with("temperature-to-humidity map") {
            lastCategory = Some(Category::TEMPERATURE_TO_HUMIDITY);
        } else if line.starts_with("humidity-to-location map") {
            lastCategory = Some(Category::HUMIDITY_TO_LOCATION);
        } else if !line.is_empty() && lastCategory.is_some() {
            let elements: Vec<&str> = line.split(" ").collect();
            if elements.len() == 3 {
                let destination = elements
                    .get(0)
                    .expect("i expected a `destination` value")
                    .parse::<u32>()
                    .expect("i expected a u32 for `destination` value");
                let source = elements
                    .get(1)
                    .expect("i expected a `source` value")
                    .parse::<u32>()
                    .expect("i expected a u32 for `source` value");
                let range = elements
                    .get(2)
                    .expect("i expected a `range` value")
                    .parse::<u32>()
                    .expect("i expected a u32 for `range` value");

                println!("\ncategory: {:?}", lastCategory);
                for i in 0..range {
                    println!("seed: {:?} => soil {:?}", source.add(i), destination.add(i));
                    // println!("range: {:?}", i);
                }
            }
        }
    }

    // println!("{:?}", lines);
}
