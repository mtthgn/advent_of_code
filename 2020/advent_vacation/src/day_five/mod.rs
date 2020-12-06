use std::fs;

pub fn run_part_one() {
    let mut row_ids = parse();

    row_ids.sort();
    row_ids.reverse();

    println!("Highest Seat ID: {}", row_ids[0]);
}

pub fn run_part_two() {
    let mut row_ids = parse();

    row_ids.sort();

    for (index, row_id) in row_ids.iter().enumerate() {
        if index != 0 {
            let previous_id = row_ids[index - 1];

            if (row_id - previous_id) > 1 {
                println!("previous_id: {}, current_id: {}", previous_id, row_id);
            }
        }

        // println!("index: {}, row_id: {}", index, row_id);
    }
}

// This function assumes no bad data!
fn traverse(string: &str, left_char: char, right_char: char, min_value : usize, max_value: usize) -> usize {
    if min_value == max_value { return min_value; }

    let mut characters = string.chars();
    let character = characters.next().unwrap();
    let midpoint = (max_value - min_value) / 2;

    if character == left_char {
        return traverse(characters.as_str(), left_char, right_char, min_value, min_value + midpoint);
    } else {
        return traverse(characters.as_str(), left_char, right_char, max_value - midpoint, max_value);
    }
}

fn parse() -> Vec<usize> {
    let contents = fs::read_to_string("./src/day_five/input.txt")
        .expect("Unable to read file");

    return contents.lines().map(|line|{
        let string = line.to_string();
        let length = string.len();
        let row_string = &string[0..(length -3)];
        let column_string = &string[(length - 3)..(length)];

        let row = traverse(row_string, 'F', 'B', 0, 127);
        let column = traverse(column_string, 'L', 'R', 0, 7);
        let id = (row * 8) + column;

        return id;
    }).collect();
}
