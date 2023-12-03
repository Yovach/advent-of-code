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
fn sum_cube_sets_count(cube_sets: Vec<&str>) -> HashMap<&str, u32> {
    let mut store: HashMap<&str, u32> = HashMap::new();

    for cube_set in cube_sets {
        let cubes: Vec<&str> = cube_set.split(",").map(|cube| cube.trim()).collect();

        println!("cube_set : {:?}", cube_set);
        for cube in cubes {
            let cubes_data: Vec<&str> = cube.split(" ").collect();
            let cube_nb = cubes_data
                .first()
                .expect("expected a quantity")
                .parse::<u32>()
                .unwrap();
            let cube_color = cubes_data.last().expect("expected a color");

            println!("color {:?} with quantity {:?}", cube_color, cube_nb);
            store
                .entry(cube_color)
                .and_modify(|value| *value += cube_nb)
                .or_insert(cube_nb);
        }
    }

    println!("store : {:?}", store);

    return store;
}

fn parse_game_data(game: &str) -> (u32, Vec<&str>, HashMap<&str, u32>) {
    let data_chunks: Vec<&str> = game.split(":").collect();
    if data_chunks.len() != 2 {
        panic!(
            "excepted a chunks with 2 elements but received : {:?}",
            data_chunks.len()
        )
    }
    let game_id = get_game_id(data_chunks.first().expect("a ID for the game is expected"));
    let cube_sets = get_cube_chunks(data_chunks.last().expect("a chunks with cube sets"));

    let cube_counts = sum_cube_sets_count(cube_sets.clone());
    return (game_id, cube_sets, cube_counts);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let games: Vec<&str> = contents.lines().collect();

    for game in &games {
        let (game_id, cube_sets, cube_counts) = parse_game_data(game);
        println!("");
        // println!("game_id: {:?}, cube_counts: {:?}", game_id, cube_counts);
    }
}
