use std::{collections::HashMap, fs, str::Lines};

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

fn get_numbers_with_positions(items: &Vec<Cell>) {
    // let mut numbers_list: HashMap<Position, u32> = HashMap::new();
    let mut numbers_list: Vec<String> = Vec::new();

    let mut previous_cell: Option<&Cell> = Option::None;
    let mut current_number = "".to_string();
    for item in items {
        if item.value.is_numeric() {
            if previous_cell.is_none() {
                current_number.push_str(item.value.to_string().as_str())
            } else {
                if previous_cell.unwrap().position.0 > item.position.0 {
                    numbers_list.push(current_number.clone());
                    current_number = "".to_string();
                    continue;
                } else if previous_cell.unwrap().position.0 == (item.position.0 - 1) {
                    current_number.push_str(item.value.to_string().as_str())
                }
                // current_number.push_str(item.value.to_string().as_str());
                // println!(
                //     "previous: {:?}, current: {:?}, sum: {:?}",
                //     previous_cell.unwrap().position.0,
                //     item.position.0,
                //     0 // (item.position.0 - previous_cell.unwrap().position.0)
                // )
                // println!(
                //     "sum : {:?}, previous: {:?}, current: {:?}",
                //     previous_cell.unwrap().position.0 == (item.position.0 - 1),
                //     previous_cell,
                //     item,
                // )
                // if previous_cell.unwrap().position.0 == (item.position.0 - 1) {
                //     current_number.push_str(item.value.to_string().as_str())
                // }
            }
        } else {
            numbers_list.push(current_number.clone());
            current_number = "".to_string();
        }
        previous_cell = Option::Some(item);
    }

    println!("number list : {:?}", numbers_list);
}

fn get_neighbours(items: &Vec<Cell>, target: &Cell, distance: usize) -> Vec<Cell> {
    let mut neighbours: Vec<Cell> = Vec::new();
    for item in items {
        let difference_x = item.position.0.abs_diff(target.position.0);
        let difference_y = item.position.1.abs_diff(target.position.1);
        if difference_x <= distance && difference_y <= distance && (difference_x + difference_y > 0)
        {
            neighbours.push(item.clone());
        }
    }

    return neighbours;
}

fn parse_lines(lines: Lines<'_>) {
    let items = items_from_lines(lines);
    for (index, item) in items.iter().enumerate() {
        if index == 1 {
            let neighbours = get_neighbours(&items, item, 1);
            println!("neighbours of {:?} : {:?}", index, neighbours);
        }
    }

    get_numbers_with_positions(&items);

    // println!("numbers: {:?}", numbers);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let lines = contents.lines();
    parse_lines(lines);
}
