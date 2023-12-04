use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("i should read the file");
    let lines = contents.lines();

    let mut total_pts = 0;
    for line in lines {
        let card_data: Vec<&str> = line.split(":").collect();
        let card_id: u32 = card_data
            .first()
            .unwrap()
            .split(" ")
            .last()
            .expect("i expect an ID")
            .parse()
            .unwrap();

        let card_numbers: Vec<&str> = card_data
            .last()
            .unwrap()
            .split("|")
            .map(|numbers| numbers.trim())
            .collect();

        let winning_numbers: Vec<u32> = card_numbers
            .first()
            .expect("i expected winning numbers")
            .split(" ")
            .filter(|number| !number.is_empty())
            .map(|number| number.parse::<u32>().unwrap())
            .collect();

        let played_numbers: Vec<u32> = card_numbers
            .last()
            .expect("i expected played numbers")
            .split(" ")
            .filter(|number| !number.is_empty())
            .map(|number| number.parse::<u32>().unwrap())
            .collect();

        let mut card_pts = 0;

        let mut matches = 0;
        for played_number in played_numbers.iter() {
            if winning_numbers.contains(played_number) {
                if matches == 0 {
                    card_pts += 1;
                } else {
                    card_pts *= 2
                }

                matches += 1;
            }
        }

        total_pts += card_pts;

        println!("card id : {:?}, pts: {:?}", card_id, card_pts)
    }

    println!("total: {:?}", total_pts);
}
