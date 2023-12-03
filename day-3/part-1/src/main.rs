use std::{fs, str::Lines};

#[derive(Debug)]
struct Cell {
    x: usize,
    y: usize,

    value: char,
}

fn items_from_lines(lines: Lines<'_>) -> Vec<Cell> {
    let mut items: Vec<Cell> = Vec::new();
    for (v_index, line) in lines.enumerate() {
        let line_values = line.trim().chars();
        for (h_index, item) in line_values.enumerate() {
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

    let numbers: Vec<&Cell> = items
        .iter()
        .filter(|cell| cell.value.is_numeric())
        .collect();

    println!("numbers: {:?}", numbers);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let lines = contents.lines();
    parse_lines(lines);
}
