use std::{fs, str::Lines};

#[derive(Debug)]
struct Position(usize, usize);

#[derive(Debug)]
struct Cell(Position, char);

fn items_from_lines(lines: Lines<'_>) -> Vec<Cell> {
    let mut items: Vec<Cell> = Vec::new();
    for (v_index, line) in lines.enumerate() {
        let line_values = line.trim().chars();
        for (h_index, item) in line_values.enumerate() {
            items.push(Cell(
                Position(h_index, v_index),
                item,
            ))
        }
    }
    return items;
}

fn get_neighbours(items: &Vec<Cell>) {}

fn parse_lines(lines: Lines<'_>) {
    let items = items_from_lines(lines);
    for (index, item) in items.iter().enumerate() {
        if item.1.is_numeric() {
            println!("item: {:?}", item.1);
        }
    }

    // println!("numbers: {:?}", numbers);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let lines = contents.lines();
    parse_lines(lines);
}
