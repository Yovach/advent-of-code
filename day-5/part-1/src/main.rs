use std::fs;

fn main() {
    let file_content = fs::read_to_string("./input.txt").expect("i should be able to read this file");
    let lines: Vec<&str> = file_content.lines().collect();

    for line in &lines {
        let elements: Vec<&str> = line.split(" ").collect();
        if elements.len() == 3 {
            let destination = elements.get(0).expect("i expected a `destination` value").parse::<u32>().expect("i expected a u32 for `destination` value");
            let source = elements.get(1).expect("i expected a `source` value").parse::<u32>().expect("i expected a u32 for `source` value");
            let range = elements.get(2).expect("i expected a `range` value").parse::<u32>().expect("i expected a u32 for `range` value");
            println!("{destination} {source} {range}");
        
                }
    }

    // println!("{:?}", lines);
}