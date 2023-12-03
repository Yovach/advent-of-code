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
fn check_if_game_is_possible(cube_sets: Vec<&str>) -> bool {
    let mut is_possible = true;

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

            if cube_color.eq(&"red") {
                if cube_nb > 12 {
                    is_possible = false;
                    continue;
                }
            } else if cube_color.eq(&"green") {
                if cube_nb > 13 {
                    is_possible = false;
                    continue;
                }
            } else if cube_color.eq(&"blue") {
                if cube_nb > 14 {
                    is_possible = false;
                    continue;
                }
            }
        }
    }

    // println!("store : {:?}", store);

    return is_possible;
}

fn parse_game_data(game: &str) -> (u32, Vec<&str>, bool) {
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

    let is_possible = check_if_game_is_possible(cube_sets.clone());
    return (game_id, cube_sets, is_possible);
}

fn main() {
    let contents = fs::read_to_string("real_input.txt").expect("The file can't be read");
    let games: Vec<&str> = contents.lines().collect();

    let mut game_ids_sum = 0;
    for game in &games {
        let (game_id, cube_sets, is_possible) = parse_game_data(game);

        if is_possible {
            game_ids_sum += game_id;
        }
    }

    println!("game ids sum : {:?}", game_ids_sum);
}
