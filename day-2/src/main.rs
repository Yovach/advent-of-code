use std::fs;

fn get_game_id(chunk: &str) -> u32 {
    let parts: Vec<&str> = chunk.split(" ").collect();
    let game_id = parts.get(1).expect("i expected a game id");
    return game_id.parse::<u32>().unwrap();
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let games: Vec<&str> = contents.lines().collect();

    for game in &games {
        let data_chunks: Vec<&str> = game.split(":").collect();
        if data_chunks.len() != 2 {
            panic!("excepted a chunks with 2 elements but received : {:?}", data_chunks.len())
        }
        let game_id = get_game_id(data_chunks.first().expect("a ID for the game is expected"));
        let cube_sets: Vec<&str> = game.split(";").collect();
        println!("cube_sets: {:?}", game_id);
    }

    println!("games : {:?}", games);
}