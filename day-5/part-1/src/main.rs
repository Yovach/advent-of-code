use std::{collections::HashMap, fs, ops::Add};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum Category {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn main() {
    let file_content =
        fs::read_to_string("./input.txt").expect("i should be able to read this file");
    let lines: Vec<&str> = file_content.lines().collect();

    let mut parsed_categories: HashMap<Category, HashMap<u32, u32>> = HashMap::new();
    let mut last_category: Option<Category> = None;

    for line in &lines {
        if line.starts_with("seed-to-soil map") {
            last_category = Some(Category::SeedToSoil);
        } else if line.starts_with("soil-to-fertilizer map") {
            last_category = Some(Category::SoilToFertilizer);
        } else if line.starts_with("fertilizer-to-water map") {
            last_category = Some(Category::FertilizerToWater);
        } else if line.starts_with("water-to-light map") {
            last_category = Some(Category::WaterToLight);
        } else if line.starts_with("light-to-temperature map") {
            last_category = Some(Category::LightToTemperature);
        } else if line.starts_with("temperature-to-humidity map") {
            last_category = Some(Category::TemperatureToHumidity);
        } else if line.starts_with("humidity-to-location map") {
            last_category = Some(Category::HumidityToLocation);
        } else if !line.is_empty() && last_category.is_some() {
            let category = last_category.expect("i expected a category");
            let stored_category = parsed_categories.entry(category).or_insert(HashMap::new());

            let elements: Vec<&str> = line.split(" ").collect();
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

            // println!("\ncategory: {:?}", category);
            for i in 0..range {
                // println!("seed: {:?} => soil {:?}", source.add(i), destination.add(i));
                stored_category.insert(source.add(i), destination.add(i));
                // println!("range: {:?}", i);
            }

            // println!("category: {:?}", stored_category);
        }
    }

    let mut categories_store: HashMap<Category, HashMap<u32, u32>> = parsed_categories.clone();
    for (category, store) in parsed_categories.iter() {
        categories_store
            .entry(category.clone())
            .and_modify(|mapping| {
                let mut max_key = 0;
                store.keys().for_each(|key| {
                    if max_key.le(key) {
                        max_key = key.clone();
                    }
                });

                for i in 0..max_key {
                    mapping.entry(i).or_insert(i);
                }
            });

        // println!("max {:?} for category {:?}", max_key, category);
    }

    println!("categories : {:?}", categories_store.get(&Category::SeedToSoil).unwrap());

    // println!("{:?}", lines);
}
