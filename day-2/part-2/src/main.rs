use std::{collections::HashMap, fs};

fn get_game_id(chunk: &str) -> u32 {
    let parts: Vec<&str> = chunk.split(" ").collect();
    let game_id = parts.get(1).expect("i expected a game id");
    return game_id.parse::<u32>().unwrap();
}

fn get_cube_chunks(chunk: &str) -> Vec<&str> {
    let cube_sets: Vec<&str> = chunk.trim().split(";").map(|cube| cube.trim()).collect();
    return cube_sets;
}

fn get_game_power(cube_sets: Vec<&str>) -> u32 {
    let mut max_store: HashMap<&str, u32> = HashMap::new();

    for cube_set in cube_sets {
        let cubes: Vec<&str> = cube_set.split(",").map(|cube| cube.trim()).collect();

        // println!("cube_set : {:?}", cube_set);
        for cube in cubes {
            let cubes_data: Vec<&str> = cube.split(" ").collect();
            let cube_nb = cubes_data
                .first()
                .expect("expected a quantity")
                .parse::<u32>()
                .unwrap();
            let cube_color = cubes_data.last().expect("expected a color");

            let max_color = max_store.get(cube_color).unwrap_or(&0);
            // println!("maxcolor : {:?}, {:?}", max_color, cube_nb);
            if cube_nb.gt(max_color) {
                // println!("greater than !!");
                max_store.entry(cube_color).and_modify(|value| {
                    *value = cube_nb;
                }).or_insert(cube_nb);
            }
        }
    }

    // println!("store : {:?}", max_store);

    let red = max_store.get("red").expect("expected a red quantity");
    let green = max_store.get("green").expect("expected a green quantity");
    let blue = max_store.get("blue").expect("expected a blue quantity");

    return red * green * blue;
}

fn parse_game_data(game: &str) -> (u32, Vec<&str>, u32) {
    let data_chunks: Vec<&str> = game.split(":").collect();
    if data_chunks.len() != 2 {
        panic!(
            "excepted a chunks with 2 elements but received : {:?}",
            data_chunks.len()
        )
    }
    let game_id = get_game_id(data_chunks.first().expect("a ID for the game is expected"));
    let cube_sets = get_cube_chunks(data_chunks.last().expect("a chunks with cube sets"));

    println!("game_id: {:?}, cube_sets: {:?}", game_id, cube_sets);

    let game_power = get_game_power(cube_sets.clone());
    return (game_id, cube_sets, game_power);
}

fn main() {
    let contents = fs::read_to_string("real_input.txt").expect("The file can't be read");
    let games: Vec<&str> = contents.lines().collect();

    let mut game_power_sum = 0;
    for game in &games {
        let (game_id, cube_sets, game_power) = parse_game_data(game);

        game_power_sum += game_power;
    }

    println!("game ids sum : {:?}", game_power_sum);
}
