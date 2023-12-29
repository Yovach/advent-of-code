use std::{collections::HashMap, fs, time::Instant};

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

fn try_parse_u64(value: Option<&&str>) -> u64 {
    return value
        .expect("i expected a value")
        .parse::<u64>()
        .expect("i expected a u64");
}

fn get_location_by_seed(
    categories: &HashMap<Category, HashMap<u64, (u64, u64)>>,
    seed_id: u64,
) -> u64 {
    // println!("seed: {:?}", seed_id);

    let soil_id = get_destination_id(categories, Category::SeedToSoil, seed_id.clone());
    // println!("soil: {:?}", soil_id);

    let fertilizer_id = get_destination_id(categories, Category::SoilToFertilizer, soil_id);
    // println!("fertilizer: {:?}", fertilizer_id);

    let water_id = get_destination_id(categories, Category::FertilizerToWater, fertilizer_id);
    // println!("water: {:?}", water_id);

    let light_id = get_destination_id(categories, Category::WaterToLight, water_id);
    // println!("light_id: {:?}", light_id);

    let temperature_id = get_destination_id(categories, Category::LightToTemperature, light_id);
    // println!("temperature: {:?}", temperature_id);

    let humidity_id =
        get_destination_id(categories, Category::TemperatureToHumidity, temperature_id);
    // println!("humidity: {:?}", humidity_id);

    let location_id = get_destination_id(categories, Category::HumidityToLocation, humidity_id);

    return location_id;
}

fn get_destination_id(
    store: &HashMap<Category, HashMap<u64, (u64, u64)>>,
    category: Category,
    asked_source: u64,
) -> u64 {
    let retrieved_cat = store.get(&category).expect("i expected a category");
    for (source, (destination, range)) in retrieved_cat.iter() {
        if *source <= asked_source && asked_source < (source + range) {
            return destination + (asked_source - source);
        }
    }

    return asked_source;
}

fn main() {
    let now = Instant::now();
    let file_content =
        fs::read_to_string("./real_input.txt").expect("i should be able to read this file");
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

    let mut parsed_categories: HashMap<Category, HashMap<u64, (u64, u64)>> = HashMap::from([
        (Category::SeedToSoil, HashMap::new()),
        (Category::SoilToFertilizer, HashMap::new()),
        (Category::FertilizerToWater, HashMap::new()),
        (Category::WaterToLight, HashMap::new()),
        (Category::LightToTemperature, HashMap::new()),
        (Category::TemperatureToHumidity, HashMap::new()),
        (Category::HumidityToLocation, HashMap::new()),
    ]);

    let mut last_category: Option<Category> = None;
    let mut planted_seeds = Vec::<(u64, u64)>::new();
    // let mut planted_seeds = Vec::<u64>::new();

    for line in &lines {
        if line.starts_with("seeds:") {
            let line_parts: Vec<&str> = line.split("seeds:").collect();
            let seeds_data: Vec<&str> = line_parts
                .get(1)
                .expect("i expected seeds")
                .split(" ")
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .collect();

            for i in (0..seeds_data.len()).step_by(2) {
                let initial_seed_id = try_parse_u64(seeds_data.get(i));
                let range = try_parse_u64(seeds_data.get(i + 1));
                planted_seeds.push((initial_seed_id, range))
            }

            println!(
                "length: {:?}, list: {:?}",
                planted_seeds.len(),
                planted_seeds
            )
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
            let destination = try_parse_u64(elements.get(0));
            let source = try_parse_u64(elements.get(1));
            let range = try_parse_u64(elements.get(2));
            stored_category
                .entry(source)
                .or_insert((destination, range));
        }
    }

    let mut min_location = u64::MAX;

    for (initial_seed_id, range) in planted_seeds {
        println!("initial: {:?}, range: {:?}", initial_seed_id, range);
        for seed_id in initial_seed_id..(initial_seed_id + range) {
            let location_id = get_location_by_seed(&parsed_categories, seed_id);
            if min_location > location_id {
                min_location = location_id
            }
        }
    }

    println!("minimum location: {:?}", min_location);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
