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
    category: &Category,
    source: &u32,
) -> u32 {
    let retrieved_cat = store.get(&category).expect("i expected a category");
    let destination = retrieved_cat.get(&source);

    if destination.is_some() {
        return destination.unwrap().0;
    }

    let mut nearest_key_result: Option<(u32, u32, u32)> = None;
    for index in (0..source.clone()).rev() {
        let nearest_source = retrieved_cat.get(&index);
        if nearest_source.is_some() {
            let (destination, range) = nearest_source.unwrap();
            nearest_key_result = Some((index, *destination, *range));
            break;
        }
    }

    // si on a pas de clés proche, alors source = destination
    if nearest_key_result.is_none() {
        return source.clone();
    }

    let (nearest_source, destination, range) = nearest_key_result.unwrap();
    for i in 0..range.clone() {
        if nearest_source.clone().add(i).eq(&source) {
            return destination.clone().add(i);
        }
    }

    return source.clone();
    // panic!("i was not able to retrieve a value for {:?} with category : {:?}", source, category);
}

fn main() {
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

    // println!("store: {:?}", parsed_categories);

    let mut locations: Vec<u32> = Vec::new();

    for seed_id in planted_seeds {
        let soil_id = get_destination_id(&parsed_categories, &Category::SeedToSoil, &seed_id);
        let fertilizer_id =
            get_destination_id(&parsed_categories, &Category::SoilToFertilizer, &soil_id);
        let water_id = get_destination_id(
            &parsed_categories,
            &Category::FertilizerToWater,
            &fertilizer_id,
        );
        let light_id = get_destination_id(&parsed_categories, &Category::WaterToLight, &water_id);
        let temperature_id =
            get_destination_id(&parsed_categories, &Category::LightToTemperature, &light_id);
        let humidity_id = get_destination_id(
            &parsed_categories,
            &Category::TemperatureToHumidity,
            &temperature_id,
        );
        let location_id = get_destination_id(
            &parsed_categories,
            &Category::HumidityToLocation,
            &humidity_id,
        );

        locations.push(location_id.clone());
    }

    locations.sort();
    println!("minimal locations: {:?}", locations.get(0))
}
