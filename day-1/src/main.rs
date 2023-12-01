use std::fs;

fn numbers_from_line(line: &str) -> Vec<char> {
    let mut numbers = Vec::new();
    line.chars().for_each(|char| {
        if char.is_numeric() {
            numbers.push(char)
        }
    });
    return numbers;
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let lines: Vec<_> = contents
        .split("\n")
        .map(|msg| -> Vec<char> { return numbers_from_line(msg) })
        .collect();

    println!("lines: {:?}", lines);
}
