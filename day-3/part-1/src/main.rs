use std::{fs, str::Lines};

#[derive(Debug, Clone)]
struct Position(usize, usize);

#[derive(Debug, Clone)]
struct Cell {
    position: Position,
    value: char,
}

fn items_from_lines(lines: Lines<'_>) -> Vec<Cell> {
    let mut items: Vec<Cell> = Vec::new();
    for (v_index, line) in lines.enumerate() {
        let line_values = line.trim().chars();
        for (h_index, item) in line_values.enumerate() {
            items.push(Cell {
                position: Position(h_index, v_index),
                value: item,
            })
        }
    }
    return items;
}

fn get_neighbours(items: &Vec<Cell>, target: &Cell, distance: usize) -> Vec<Cell> {
    let mut neighbours: Vec<Cell> = Vec::new();
    for (index, item) in items.iter().enumerate() {
        let difference_x = item.position.0.abs_diff(target.position.0);
        let difference_y = item.position.1.abs_diff(target.position.1);
        if difference_x <= distance && difference_y <= distance {
            neighbours.push(item.clone());
        }
    }

    return neighbours;
}

fn parse_lines(lines: Lines<'_>) {
    let items = items_from_lines(lines);
    for (index, item) in items.iter().enumerate() {
        if index == 26 {
            println!("neigbours : {:?}", get_neighbours(&items, item, 1));
            println!("neigbours : {:?}", get_neighbours(&items, item, 2));
        }
        println!("item: {:?}", item.value);
    }

    // println!("numbers: {:?}", numbers);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let lines = contents.lines();
    parse_lines(lines);
}
