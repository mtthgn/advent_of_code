use std::vec::Vec;
use std::fs;
use std::collections::HashSet;

pub fn run_part_one() {
    let mut past_numbers  = HashSet::new();
    let sum = 2020;
    let numbers = input_as_numbers();

    for number in numbers {
        let difference = sum - number;

        if past_numbers.contains(&difference) {
            println!("first number: {}, second number: {}, multiple: {}", number, difference, number * difference);
            return;
        } else {
            past_numbers.insert(number);
        }
    }
}

pub fn run_part_two() {
    let mut past_numbers  = HashSet::new();
    let sum = 2020;
    let numbers = input_as_numbers();

    for number in numbers {
        if past_numbers.len() == 0 {
            past_numbers.insert(number);
        } else {
            for past_number in past_numbers.iter() {
                let mid_point = number + past_number;
                let difference = sum - mid_point;

                if past_numbers.contains(&difference) {
                    println!("first number: {}, second number: {}, third: {}, multiple: {}", number, past_number, difference, number * past_number * difference);
                    return;
                }
            }

            past_numbers.insert(number);
        }
    }
}

fn input_as_numbers() -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let contents = fs::read_to_string("./src/day_one/input.txt")
        .expect("Unable to read file.");

    for number_string in contents.lines() {
        let number = number_string.parse::<i32>().unwrap();
        numbers.push(number);
    }

    return numbers;
}
