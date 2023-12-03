use std::{fs, str::Lines};

#[derive(Debug)]
struct Cell<'a> {
    x: usize,
    y: usize,

    value: &'a str,
}

fn items_from_lines(lines: Lines<'_>) -> Vec<Cell> {
    let mut items: Vec<Cell> = Vec::new();
    for (v_index, line) in lines.enumerate() {
        let line_chars = line.trim().split("").filter(|item| !item.is_empty());
        for (h_index, item) in line_chars.enumerate() {
            items.push(Cell {
                x: h_index,
                y: v_index,
                value: item,
            })
        }
    }
    return items;
}

fn parse_lines(lines: Lines<'_>) {
    let items = items_from_lines(lines);

    let numbers: Vec<u32> = Vec::new();

    println!("items : {:?}", items);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let lines = contents.lines();
    parse_lines(lines);
}
