use std::fs;
use std::collections::VecDeque;

pub fn run_part_one() {
    let values = parse();
    let preamble_length = 25;
    let mut preamble = VecDeque::<i64>::with_capacity(preamble_length);

    for value in values {
        let mut valid = false;

        if preamble.len() < preamble_length {
            preamble.push_back(value);
        } else {
            for x in preamble.iter() {
                if preamble.contains(&(value - x)) {
                    valid = true;
                    break;
                }
            }

            if !valid {
                println!("Invalid Value: {}", value);
                break;
            } else {
                preamble.pop_front();
                preamble.push_back(value);
            }
        }
    }
}

pub fn run_part_two() {
    let sum_goal : i64 = 90433990;
    let values = parse();
    let mut chain = VecDeque::<i64>::new();
    let mut position : usize = 0;

    while chain.iter().sum::<i64>() != sum_goal {
        if chain.iter().sum::<i64>() > sum_goal {
            chain.pop_front();
        }

        while chain.iter().sum::<i64>() > sum_goal {
            chain.pop_back();
            position -= 1;
        }

        if chain.iter().sum::<i64>() != sum_goal {
            chain.push_back(values[position]);
            position += 1;
        }
    }

    chain.make_contiguous().sort();

    let (sorted, _) = chain.as_slices();
    let front = sorted[0];
    let back = sorted[sorted.len() - 1];

    println!("Encryption Weakness: {}", front + back);
}

fn parse() -> Vec<i64> {
    let contents = fs::read_to_string("./src/day_nine/input.txt")
        .expect("Unable to read file");

    contents.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}
