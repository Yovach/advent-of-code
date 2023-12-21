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

    let mut planted_seets: Vec<u32> = Vec::new();

    for line in &lines {
        if line.starts_with("seeds:") {
            let line_parts: Vec<&str> = line.split("seeds:").collect();
            if line_parts.len() == 2 {
                line_parts
                    .get(1)
                    .expect("i expected seeds")
                    .split(" ")
                    .filter(|val| !val.is_empty())
                    .map(|val| {
                        return val.trim().parse::<u32>().unwrap();
                    })
                    .for_each(|value| {
                        planted_seets.push(value);
                    })
            }
        } else if line.starts_with("seed-to-soil map") {
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
                stored_category
                    .entry(source.add(i))
                    .and_modify(|des| {
                        *des = destination + i;
                    })
                    .or_insert(destination.add(i));
                // println!("range: {:?}", i);
            }

            // println!("category: {:?}", stored_category);
        }
    }

    let mut max_key = &0;
    let mut categories_store: HashMap<Category, HashMap<u32, u32>> = parsed_categories.clone();
    for (category, store) in parsed_categories.iter() {
        store.keys().for_each(|key| {
            if max_key.le(key) {
                max_key = key;
            }
        });
        categories_store
            .entry(category.clone())
            .and_modify(|mapping| {
                for i in 0..max_key.clone() {
                    mapping.entry(i).or_insert_with(|| i);
                }
            });

        // println!("max {:?} for category {:?}", max_key, category);
    }

    // println!("categories: {:?}", categories_store);

    let mut min_location = &u32::MAX;

    // println!("min location: {:?}", min_location);
    println!("planted seeds: {:?}", planted_seets);

    for (seed_id, soil_id) in categories_store.get(&Category::SeedToSoil).unwrap() {
        // println!("seed: {:?}, soil: {:?}", seed_id, soil_id);

        let fertilizer_result = categories_store
            .get(&Category::SoilToFertilizer)
            .expect("i expected a fertilizer cat")
            .get(soil_id);
        if fertilizer_result.is_some() {
            let fertilizer = fertilizer_result.unwrap();
            let water_result = categories_store
                .get(&Category::FertilizerToWater)
                .expect("i expected a water cat")
                .get(fertilizer);
            if water_result.is_some() {
                let water = water_result.unwrap();
                let light_result = categories_store
                    .get(&Category::WaterToLight)
                    .expect("i expected a light cat")
                    .get(water);

                if light_result.is_some() {
                    let light = light_result.unwrap();
                    let temperature_result = categories_store
                        .get(&Category::LightToTemperature)
                        .expect("i expected a temperature cat")
                        .get(light);

                    if temperature_result.is_some() {
                        let temperature = temperature_result.unwrap();
                        let humidity_result = categories_store
                            .get(&Category::TemperatureToHumidity)
                            .expect("i expected a humidity cat")
                            .get(temperature);

                        if humidity_result.is_some() {
                            let humidity = humidity_result.unwrap();
                            let location_result = categories_store
                                .get(&Category::HumidityToLocation)
                                .expect("i expected a location cat")
                                .get(humidity);

                            if location_result.is_some() {
                                let location = location_result.unwrap();
                                // println!("location: {:?}", location);

                                if planted_seets.contains(seed_id) {
                                    println!("Seed {:?}, soil {:?}, fertilizer {:?}, water {:?}, light {:?}, temperature {:?}, humidity {:?}, location {:?}", seed_id, soil_id, fertilizer, water, light, temperature, humidity, location);

                                    if location.le(min_location) {
                                        min_location = location;
                                    }
                                }
                            } else {
                                // println!("i maybe have a problem here, Seed {:?}, soil {:?}, fertilizer {:?}, water {:?}, light {:?}, temperature {:?}, humidity {:?}", seed_id, soil_id, fertilizer, water, light, temperature, humidity)
                            }
                        } else {
                            // println!("i maybe have a problem here, Seed {:?}, soil {:?}, fertilizer {:?}, water {:?}, light {:?}, temperature {:?}", seed_id, soil_id, fertilizer, water, light, temperature)
                        }
                    } else {
                        // println!("i maybe have a problem here, Seed {:?}, soil {:?}, fertilizer {:?}, water {:?}, light {:?}", seed_id, soil_id, fertilizer, water, light)
                    }
                } else {
                    // println!("i maybe have a problem here, Seed {:?}, soil {:?}, fertilizer {:?}, water {:?}", seed_id, soil_id, fertilizer, water)
                }
            } else {
                // println!(
                //     "i maybe have a problem here, Seed {:?}, soil {:?}, fertilizer {:?}",
                //     seed_id, soil_id, fertilizer
                // )
            }
        } else {
            // println!(
            //     "i maybe have a problem here, Seed {:?}, soil {:?}",
            //     seed_id, soil_id
            // )
        }
    }

    println!("{:?}", min_location);
}
