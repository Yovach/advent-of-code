use std::{fs, str::Lines, thread::current};

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Cell {
    position: Position,
    value: char,
}

#[derive(Debug)]
struct CellNumber {
    position: Position,
    value: u32,
}

fn items_from_lines(lines: Lines<'_>) -> Vec<Cell> {
    let mut items: Vec<Cell> = Vec::new();
    for (v_index, line) in lines.enumerate() {
        let line_values = line.trim().chars();
        for (h_index, item) in line_values.enumerate() {
            items.push(Cell {
                position: Position {
                    x: h_index,
                    y: v_index,
                },
                value: item,
            })
        }
    }
    return items;
}

fn get_neighbours(items: &Vec<Cell>) {

}

// fn parse_numbers_from_lines(items: Vec<&Cell>) {
//     let mut cell_numbers: Vec<CellNumber> = Vec::new();
//     let mut previous_cell: CellNumber;

//     let mut current_number = String::from("");

//     println!("numbers: {:?}", items);
//     for item in items {

//         current_number.push_str(&item.value.to_string());

//         previous_cell = CellNumber {
//             value  
//         }

//         cell_numbers.push(CellNumber {
//             value: 5,
//         })
//     }
// }

fn parse_lines(lines: Lines<'_>) {
    let items = items_from_lines(lines);
    for (index, item) in items.iter().enumerate() {
        if item.value.is_numeric() {

        }
    }

    // println!("numbers: {:?}", numbers);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("The file can't be read");
    let lines = contents.lines();
    parse_lines(lines);
}
