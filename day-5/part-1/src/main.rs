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

fn try_parse_u32(value: Option<&&str>) -> u32 {
    return value
        .expect("i expected a value")
        .parse::<u32>()
        .expect("i expected a u32");
}

fn get_destination_id(
    store: &HashMap<Category, HashMap<u32, (u32, u32)>>,
    category: Category,
    source: u32,
) -> u32 {
    let retrieved_cat = store.get(&category).expect("i expected a category");
    let destination = retrieved_cat.get(&source);

    if destination.is_some() {
        return destination.unwrap().0;
    }

    let mut nearest_key: Option<&(u32, u32)> = None;
    for k in (0..source).rev() {
        let nearest_source = retrieved_cat.get(&k);
        if nearest_source.is_some() {
            nearest_key = Some(nearest_source.unwrap());
            break;
        }
    }

    // si on a pas de clés proche, alors source = destination
    if nearest_key.is_none() {
        return source;
    }

    println!("nearest key : {:?} for {source}", nearest_key);

    return 0;
}

fn main() {
    let file_content =
        fs::read_to_string("./input.txt").expect("i should be able to read this file");
    let lines: Vec<&str> = file_content.lines().collect();

    let category_mappings = HashMap::from([
        ("seed-to-soil", Category::SeedToSoil),
        ("soil-to-fertilizer", Category::SoilToFertilizer),
        ("fertilizer-to-water", Category::FertilizerToWater),
        ("water-to-light", Category::WaterToLight),
        ("light-to-temperature", Category::LightToTemperature),
        ("temperature-to-humidity", Category::TemperatureToHumidity),
        ("humidity-to-location", Category::HumidityToLocation),
    ]);

    let mut parsed_categories: HashMap<Category, HashMap<u32, (u32, u32)>> = HashMap::from([
        (Category::SeedToSoil, HashMap::new()),
        (Category::SoilToFertilizer, HashMap::new()),
        (Category::FertilizerToWater, HashMap::new()),
        (Category::WaterToLight, HashMap::new()),
        (Category::LightToTemperature, HashMap::new()),
        (Category::TemperatureToHumidity, HashMap::new()),
        (Category::HumidityToLocation, HashMap::new()),
    ]);

    let mut last_category: Option<Category> = None;
    let mut planted_seeds: Vec<u32> = Vec::new();

    for line in &lines {
        if line.starts_with("seeds:") {
            let line_parts: Vec<&str> = line.split("seeds:").collect();
            line_parts
                .get(1)
                .expect("i expected seeds")
                .split(" ")
                .filter(|val| !val.is_empty())
                .map(|val| val.trim().parse::<u32>().unwrap())
                .for_each(|value| planted_seeds.push(value))
        } else if line.contains("map:") {
            let line_parts: Vec<&str> = line.split("map:").collect();
            let category = line_parts.get(0).expect("i expected a category").trim();
            last_category = Some(
                *category_mappings
                    .get(category)
                    .expect("i expected to receive a category value (from `category_mappings`)"),
            );
        } else if !line.is_empty() && last_category.is_some() {
            let stored_category = parsed_categories
                .entry(last_category.expect("i expected a category"))
                .or_insert(HashMap::new());

            let elements: Vec<&str> = line.split(" ").collect();
            let destination = try_parse_u32(elements.get(0));
            let source = try_parse_u32(elements.get(1));
            let range = try_parse_u32(elements.get(2));
            stored_category
                .entry(source)
                .or_insert((destination, range));
        }
    }

    println!("store: {:?}", parsed_categories);

    for planted_seed in planted_seeds {
        let soil_id = get_destination_id(&parsed_categories, Category::SeedToSoil, planted_seed);

        println!("seed: {:?}, soil_id: {:?}", planted_seed, soil_id);
    }

    // let mut max_key = &0;
    // let mut categories_store: HashMap<Category, HashMap<u32, (u32, u32)>> =
    //     parsed_categories.clone();
    // for (category, store) in parsed_categories.iter() {
    //     let mut min_key = &0;
    //     store.keys().for_each(|key| {
    //         if max_key.le(key) {
    //             max_key = key;
    //         } else if min_key.ge(key) {
    //             min_key = key;
    //         }
    //     });
    //     categories_store
    //         .entry(category.clone())
    //         .and_modify(|mapping| {
    //             for i in min_key.clone()..max_key.clone() {
    //                 mapping.entry(i).or_insert_with(|| i);
    //             }
    //         });

    //     // println!("max {:?} for category {:?}", max_key, category);
    // }

    // // println!("categories: {:?}", categories_store);

    // let mut min_location = &u32::MAX;

    // // println!("min location: {:?}", min_location);
    // println!("planted seeds: {:?}", planted_seets);

    // for (seed_id, soil_id) in categories_store.get(&Category::SeedToSoil).unwrap() {
    //     // println!("seed: {:?}, soil: {:?}", seed_id, soil_id);

    //     let fertilizer_result = categories_store
    //         .get(&Category::SoilToFertilizer)
    //         .expect("i expected a fertilizer cat")
    //         .get(soil_id);
    //     if fertilizer_result.is_some() {
    //         let fertilizer = fertilizer_result.unwrap();
    //         let water_result = categories_store
    //             .get(&Category::FertilizerToWater)
    //             .expect("i expected a water cat")
    //             .get(fertilizer);
    //         if water_result.is_some() {
    //             let water = water_result.unwrap();
    //             let light_result = categories_store
    //                 .get(&Category::WaterToLight)
    //                 .expect("i expected a light cat")
    //                 .get(water);

    //             if light_result.is_some() {
    //                 let light = light_result.unwrap();
    //                 let temperature_result = categories_store
    //                     .get(&Category::LightToTemperature)
    //                     .expect("i expected a temperature cat")
    //                     .get(light);

    //                 if temperature_result.is_some() {
    //                     let temperature = temperature_result.unwrap();
    //                     let humidity_result = categories_store
    //                         .get(&Category::TemperatureToHumidity)
    //                         .expect("i expected a humidity cat")
    //                         .get(temperature);

    //                     if humidity_result.is_some() {
    //                         let humidity = humidity_result.unwrap();
    //                         let location_result = categories_store
    //                             .get(&Category::HumidityToLocation)
    //                             .expect("i expected a location cat")
    //                             .get(humidity);

    //                         if location_result.is_some() {
    //                             let location = location_result.unwrap();
    //                             // println!("location: {:?}", location);

    //                             if planted_seets.contains(seed_id) {
    //                                 println!("Seed {:?}, soil {:?}, fertilizer {:?}, water {:?}, light {:?}, temperature {:?}, humidity {:?}, location {:?}", seed_id, soil_id, fertilizer, water, light, temperature, humidity, location);

    //                                 if location.le(min_location) {
    //                                     min_location = location;
    //                                 }
    //                             }
    //                         } else {
    //                             // println!("i maybe have a problem here, Seed {:?}, soil {:?}, fertilizer {:?}, water {:?}, light {:?}, temperature {:?}, humidity {:?}", seed_id, soil_id, fertilizer, water, light, temperature, humidity)
    //                         }
    //                     } else {
    //                         // println!("i maybe have a problem here, Seed {:?}, soil {:?}, fertilizer {:?}, water {:?}, light {:?}, temperature {:?}", seed_id, soil_id, fertilizer, water, light, temperature)
    //                     }
    //                 } else {
    //                     // println!("i maybe have a problem here, Seed {:?}, soil {:?}, fertilizer {:?}, water {:?}, light {:?}", seed_id, soil_id, fertilizer, water, light)
    //                 }
    //             } else {
    //                 // println!("i maybe have a problem here, Seed {:?}, soil {:?}, fertilizer {:?}, water {:?}", seed_id, soil_id, fertilizer, water)
    //             }
    //         } else {
    //             // println!(
    //             //     "i maybe have a problem here, Seed {:?}, soil {:?}, fertilizer {:?}",
    //             //     seed_id, soil_id, fertilizer
    //             // )
    //         }
    //     } else {
    //         // println!(
    //         //     "i maybe have a problem here, Seed {:?}, soil {:?}",
    //         //     seed_id, soil_id
    //         // )
    //     }
    // }

    // println!("{:?}", min_location);
}
