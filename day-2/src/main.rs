use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let games: Vec<&str> = contents.lines().collect();

    for game in &games {
        let data_chunks: Vec<&str> = game.split(":").collect();
        if data_chunks.len() != 2 {
            panic!("excepted a chunks with 2 elements but received : {:?}", data_chunks.len())
        }
        let cube_sets: Vec<&str> = game.split(";").collect();
        println!("cube_sets: {:?}", cube_sets);
    }

    println!("games : {:?}", games);
}
